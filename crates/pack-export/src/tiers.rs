//! Reverse-LOD sidecar emission for game `.10d` assets (native only).
//!
//! Each hero asset carries its Tier 1 mesh + Tier 4 hierarchical surfels
//! in a single sealed container (the container encoder sorts sections by
//! ascending type, so only tiers that sort after the mesh can coexist
//! with it). The remaining tiers are emitted here as sibling sealed
//! `.10d` pack entries so the bundle preserves the verifier's reverse-LOD
//! ordering contract:
//!
//! * `…--tier2--<hash>.10d` — QEM-decimated QuantizedMesh (~half the
//!   hero triangle budget), a sealed `.10d` container.
//! * `…--tier3--<hash>.impo` — hemi-octahedral impostor atlas as the raw
//!   IMPO wire payload (44-byte mini header + RGBA8 atlas), rasterized
//!   deterministically from the hero mesh with the authored albedo; alpha
//!   carries view-depth. It is not wrapped in a `.10d` because this
//!   QualiaDB revision's IMPO header-size constant disagrees with the
//!   actual struct size, so every IMPO codec/verifier path panics.
//!
//! Tier files are deterministic in the hero bytes, so repacking is
//! reproducible and the manifest digests stay honest.

use qualia_core_db::container_10d::impostor_section::{
    hemi_octahedral_decode, FLAG_DEPTH_IN_ALPHA, FLAG_HEMI_OCTAHEDRAL,
};
use qualia_core_db::render::compile_10d::{compile_mesh_to_10d, decode_10d_mesh};
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::specialized_libs::computational_geometry::{
    decimate_qem, DecimateOptions, Point3,
};
use sha2::{Digest, Sha256};

/// Hemi-octahedral view grid per axis (grid² total views).
pub const IMPOSTOR_GRID: u16 = 8;
/// Texels per view edge; alpha encodes view-space depth.
pub const IMPOSTOR_VIEW_RES: u16 = 8;
/// Below this triangle count a decimated tier adds no value.
pub const TIER2_MIN_TRIANGLES: usize = 16;
/// Tier 2 keeps roughly half the hero triangle budget.
const TIER2_FACE_FRACTION: f64 = 0.5;

/// Byte length of the IMPO wire header. This is the true
/// `ImpostorAtlasMiniHeader` layout (44 bytes); the upstream
/// `IMPOSTOR_MINI_HEADER_SIZE` constant says 64, which makes every
/// encode/decode/verify path in `impostor_section` panic, so the section
/// cannot ride inside a `.10d` on this QualiaDB revision. The raw wire
/// payload is emitted as a standalone `.impo` pack entry instead — when the
/// constant is fixed upstream, these bytes drop into SectionType 19
/// unchanged.
pub const IMPOSTOR_WIRE_HEADER_SIZE: usize = 44;
const MAGIC_IMPO: [u8; 4] = *b"IMPO";
const IMPOSTOR_SECTION_VERSION: u16 = 1;

pub struct TierSidecars {
    /// `(entry key, sealed bytes, triangle count)` for the decimated tier.
    pub tier2: Option<(String, Vec<u8>, usize)>,
    /// `(entry key, raw IMPO wire bytes)` for the impostor atlas.
    pub tier3: (String, Vec<u8>),
}

fn sha256_hex8(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    format!("{:02x}{:02x}{:02x}{:02x}", digest[0], digest[1], digest[2], digest[3])
}

/// Hero file keys look like `10d/<slug>--<sha8>.10d`; tier keys re-derive
/// the same stem so the family stays visually grouped in listings. Tier 3
/// lands under `impo/` because the IMPO section codec is broken upstream
/// (see `IMPOSTOR_WIRE_HEADER_SIZE`): it is the raw wire payload, not a
/// `.10d` container.
fn sidecar_key(hero_file: &str, dir: &str, tag: &str, ext: &str, bytes: &[u8]) -> Result<String, String> {
    let stem = hero_file
        .strip_prefix("10d/")
        .and_then(|rest| rest.strip_suffix(".10d"))
        .ok_or_else(|| format!("hero file {hero_file} is not under 10d/"))?;
    Ok(format!(
        "{dir}/{stem}--{tag}--{}.{ext}",
        sha256_hex8(bytes)
    ))
}

/// Serialise the IMPO wire header by hand at fixed little-endian offsets.
/// Equivalent to `bytemuck::bytes_of(&ImpostorAtlasMiniHeader)` — the struct
/// layout is correct; only the upstream size constant is wrong.
pub fn impostor_wire_header(
    flags: u16,
    grid: u16,
    view_res: u16,
    sphere_center: [f32; 3],
    sphere_radius: f32,
    atlas_byte_size: u32,
    channel_format: u8,
) -> [u8; IMPOSTOR_WIRE_HEADER_SIZE] {
    let mut out = [0u8; IMPOSTOR_WIRE_HEADER_SIZE];
    out[0..4].copy_from_slice(&MAGIC_IMPO);
    out[4..6].copy_from_slice(&IMPOSTOR_SECTION_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&flags.to_le_bytes());
    out[8..10].copy_from_slice(&grid.to_le_bytes());
    out[10..12].copy_from_slice(&view_res.to_le_bytes());
    for (axis, value) in sphere_center.iter().enumerate() {
        out[12 + axis * 4..16 + axis * 4].copy_from_slice(&value.to_le_bytes());
    }
    out[24..28].copy_from_slice(&sphere_radius.to_le_bytes());
    out[28..32].copy_from_slice(&atlas_byte_size.to_le_bytes());
    out[32] = channel_format;
    // bytes 33..44: reserved, zeroed
    out
}

fn bounds_of(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    (min, max)
}

/// QEM-decimate the hero mesh to the Tier 2 budget and compile the result
/// as a standalone sealed `.10d` QuantizedMesh container.
fn tier2_container(mesh: &Mesh) -> Result<Option<(Vec<u8>, usize)>, String> {
    if mesh.triangles.len() < TIER2_MIN_TRIANGLES {
        return Ok(None);
    }
    let target = (mesh.triangles.len() as f64 * TIER2_FACE_FRACTION) as usize;
    if target < 4 {
        return Ok(None);
    }
    let verts_f64: Vec<Point3> = mesh
        .positions
        .iter()
        .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
        .collect();
    let mut out_v = vec![Point3::default(); verts_f64.len()];
    let mut out_t = vec![[0u32; 3]; mesh.triangles.len()];
    let report = decimate_qem(
        &verts_f64,
        &mesh.triangles,
        DecimateOptions::to_faces(target),
        &mut out_v,
        &mut out_t,
    )
    .map_err(|e| format!("tier2 decimate: {e:?}"))?;
    if report.faces == 0 {
        return Ok(None);
    }
    let positions: Vec<[f32; 3]> = out_v[..report.vertices]
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect();
    let triangles: Vec<[u32; 3]> = out_t[..report.faces].to_vec();
    let (min, max) = bounds_of(&positions);
    let lod_mesh = Mesh {
        positions,
        triangles,
        min,
        max,
    };
    let bytes = compile_mesh_to_10d(&lod_mesh).map_err(|e| format!("tier2 compile: {e:?}"))?;
    Ok(Some((bytes, report.faces)))
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = dot(v, v).sqrt();
    if len < 1e-12 {
        return [0.0, 1.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

/// Deterministically rasterize the hero mesh into a hemi-octahedral impostor
/// atlas: `grid²` orthographic views over the upper hemisphere, `res²` RGBA8
/// texels per view. RGB carries the authored albedo; alpha carries the
/// nearest-surface view-space depth in `[0,255]`, 0 where nothing covers
/// the texel. This is cold-path authoring — bounded `Vec` scratch is fine.
fn impostor_atlas(mesh: &Mesh, base_rgb: [f32; 3]) -> Vec<u8> {
    let grid = IMPOSTOR_GRID as usize;
    let res = IMPOSTOR_VIEW_RES as usize;
    let center = [
        (mesh.min[0] + mesh.max[0]) * 0.5,
        (mesh.min[1] + mesh.max[1]) * 0.5,
        (mesh.min[2] + mesh.max[2]) * 0.5,
    ];
    let radius = ((mesh.max[0] - mesh.min[0]).powi(2)
        + (mesh.max[1] - mesh.min[1]).powi(2)
        + (mesh.max[2] - mesh.min[2]).powi(2))
    .sqrt()
        * 0.5
        + f32::EPSILON;
    let mut atlas = vec![0u8; grid * grid * res * res * 4];
    let rgb = [
        (base_rgb[0].clamp(0.0, 1.0) * 255.0).round() as u8,
        (base_rgb[1].clamp(0.0, 1.0) * 255.0).round() as u8,
        (base_rgb[2].clamp(0.0, 1.0) * 255.0).round() as u8,
    ];
    for gy in 0..grid {
        for gx in 0..grid {
            // Hemi-octahedral cell center → unit view direction; the
            // hemisphere pole maps to world +Y (top-down game camera).
            let d = hemi_octahedral_decode([
                (gx as f32 + 0.5) / grid as f32,
                (gy as f32 + 0.5) / grid as f32,
            ]);
            let n = [d[0], d[2], d[1]];
            let up = if n[1].abs() > 0.9 { [0.0, 0.0, 1.0] } else { [0.0, 1.0, 0.0] };
            let e1 = normalize(cross(n, up));
            let e2 = cross(e1, n);
            let view = (gy * grid + gx) * res * res;
            let mut depth = vec![f32::NEG_INFINITY; res * res];
            for tri in &mesh.triangles {
                let mut px = [0.0f32; 3];
                let mut py = [0.0f32; 3];
                let mut ps = [0.0f32; 3];
                for (corner, &vi) in tri.iter().enumerate() {
                    let p = mesh.positions[vi as usize];
                    let rel = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
                    let u = dot(rel, e1) / radius;
                    let v = dot(rel, e2) / radius;
                    px[corner] = (u * 0.5 + 0.5) * (res as f32 - 1.0);
                    py[corner] = (v * 0.5 + 0.5) * (res as f32 - 1.0);
                    ps[corner] = dot(rel, n);
                }
                let min_x = px.iter().cloned().fold(f32::INFINITY, f32::min).floor() as i32;
                let max_x = px.iter().cloned().fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
                let min_y = py.iter().cloned().fold(f32::INFINITY, f32::min).floor() as i32;
                let max_y = py.iter().cloned().fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
                for ty in min_y.max(0)..=max_y.min(res as i32 - 1) {
                    for tx in min_x.max(0)..=max_x.min(res as i32 - 1) {
                        // Barycentric half-space test in view space.
                        let fx = tx as f32;
                        let fy = ty as f32;
                        let d0 = (px[1] - px[0]) * (fy - py[0]) - (py[1] - py[0]) * (fx - px[0]);
                        let d1 = (px[2] - px[1]) * (fy - py[1]) - (py[2] - py[1]) * (fx - px[1]);
                        let d2 = (px[0] - px[2]) * (fy - py[2]) - (py[0] - py[2]) * (fx - px[2]);
                        let all_pos = d0 >= 0.0 && d1 >= 0.0 && d2 >= 0.0;
                        let all_neg = d0 <= 0.0 && d1 <= 0.0 && d2 <= 0.0;
                        if !(all_pos || all_neg) {
                            continue;
                        }
                        let area = d0 + d1 + d2;
                        let s = if area.abs() > 1e-12 {
                            (d1 * ps[0] + d2 * ps[1] + d0 * ps[2]) / area
                        } else {
                            (ps[0] + ps[1] + ps[2]) / 3.0
                        };
                        let idx = ty as usize * res + tx as usize;
                        if s > depth[idx] {
                            depth[idx] = s;
                        }
                    }
                }
            }
            for idx in 0..res * res {
                let base = (view + idx) * 4;
                if depth[idx].is_finite() {
                    atlas[base] = rgb[0];
                    atlas[base + 1] = rgb[1];
                    atlas[base + 2] = rgb[2];
                    // Depth in alpha: nearest visible surface → 255, far
                    // silhouette plane → 0. Uncovered texels stay 0.
                    let t = ((depth[idx] + radius) / (2.0 * radius)).clamp(0.0, 1.0);
                    atlas[base + 3] = (t * 255.0).round() as u8;
                }
            }
        }
    }
    atlas
}

/// Build the sealed Tier 2 + Tier 3 sibling containers for one hero asset.
/// `hero_file` is the manifest `file` key (`10d/<slug>--<sha>.10d`); the
/// sidecar keys inherit its stem so the family stays grouped.
pub fn emit_sidecars(
    hero_file: &str,
    hero_bytes: &[u8],
    base_rgb: [f32; 3],
) -> Result<TierSidecars, String> {
    let mesh = decode_10d_mesh(hero_bytes).map_err(|e| format!("{hero_file}: hero decode: {e:?}"))?;

    let tier2 = match tier2_container(&mesh)? {
        Some((bytes, tris)) => {
            Some((sidecar_key(hero_file, "10d", "tier2", "10d", &bytes)?, bytes, tris))
        }
        None => None,
    };

    let atlas = impostor_atlas(&mesh, base_rgb);
    let center = [
        (mesh.min[0] + mesh.max[0]) * 0.5,
        (mesh.min[1] + mesh.max[1]) * 0.5,
        (mesh.min[2] + mesh.max[2]) * 0.5,
    ];
    let radius = ((mesh.max[0] - mesh.min[0]).powi(2)
        + (mesh.max[1] - mesh.min[1]).powi(2)
        + (mesh.max[2] - mesh.min[2]).powi(2))
    .sqrt()
        * 0.5
        + f32::EPSILON;
    let header = impostor_wire_header(
        FLAG_HEMI_OCTAHEDRAL | FLAG_DEPTH_IN_ALPHA,
        IMPOSTOR_GRID,
        IMPOSTOR_VIEW_RES,
        center,
        radius,
        atlas.len() as u32,
        0,
    );
    let mut bytes = Vec::with_capacity(IMPOSTOR_WIRE_HEADER_SIZE + atlas.len());
    bytes.extend_from_slice(&header);
    bytes.extend_from_slice(&atlas);
    let key = sidecar_key(hero_file, "impo", "tier3", "impo", &bytes)?;

    Ok(TierSidecars {
        tier2,
        tier3: (key, bytes),
    })
}
