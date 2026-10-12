//! Authored material parameters and shared original surface dependencies.
//! Qualia generates UVs/maps, encodes PNG and verifies content-addressed images.
//! No renderer, image codec, mip kernel or alternative package format lives here.

use crate::asset_catalog::{AssetRecipe, ParametricRecipe};
use qualia_core_db::container_10d::{MaterialRecord, OpacityMode};
use qualia_core_db::render::surface_authoring::{
    encode_surface_png_into, generate_surface_maps_into, SurfaceRecipe,
};
use qualia_core_db::render::texture_decode::TextureDecodeLimits;
use qualia_core_db::render::texture_ingestion::ingest_texture_payload;
use std::cell::RefCell;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

const SIDE: u32 = 128;
struct SurfaceSet {
    digests: [[u8; 32]; 3],
}
struct Image {
    digest: [u8; 32],
    bytes: Vec<u8>,
    role: u8,
}
#[derive(Default)]
struct Library {
    surfaces: BTreeMap<&'static str, SurfaceSet>,
    images: BTreeMap<([u8; 32], u8), Image>,
}
thread_local! { static LIBRARY: RefCell<Library> = RefCell::new(Library::default()); }

fn parameters(family: &str) -> SurfaceRecipe {
    let (contrast, relief, bands, strength) = match family {
        "weathered-timber" => (0.22, 0.003, [18, 1], 0.72),
        "soil-and-gravel" => (0.20, 0.004, [0, 0], 0.0),
        "weathered-concrete" => (0.10, 0.0015, [0, 0], 0.0),
        "cloth-and-skin" | "shade-canvas" => (0.06, 0.0005, [32, 32], 0.42),
        "living-foliage" => (0.09, 0.0003, [0, 12], 0.28),
        "rubber" => (0.06, 0.001, [24, 2], 0.30),
        "terracotta" | "lime-render" => (0.10, 0.001, [0, 0], 0.0),
        "fur-and-hide" => (0.08, 0.0005, [0, 20], 0.30),
        _ => (0.035, 0.00015, [0, 0], 0.0),
    };
    SurfaceRecipe {
        seed: qualia_core_db::q_hash(family) as u32,
        tint: [1.0; 3],
        contrast,
        relief,
        bands,
        band_strength: strength,
        roughness: 0.97,
        metallic: 1.0,
    }
}

fn ensure_surface(library: &mut Library, family: &'static str) -> Result<[[u8; 32]; 3], String> {
    if let Some(surface) = library.surfaces.get(family) {
        return Ok(surface.digests);
    }
    if library.surfaces.len() >= 32 {
        return Err("surface family budget exceeded".into());
    }
    let size = (SIDE * SIDE * 4) as usize;
    let mut colour = vec![0; size];
    let mut normal = vec![0; size];
    let mut orm = vec![0; size];
    generate_surface_maps_into(parameters(family), SIDE, &mut colour, &mut normal, &mut orm)
        .map_err(|e| format!("surface authoring: {e:?}"))?;
    let mut digests = [[0; 32]; 3];
    for (role, rgba) in [colour, normal, orm].iter().enumerate() {
        let mut png = vec![0; size + SIDE as usize + 4096];
        let used = encode_surface_png_into(SIDE, rgba, &mut png)
            .map_err(|e| format!("surface PNG: {e:?}"))?;
        png.truncate(used);
        let ingested =
            ingest_texture_payload("image/png", Some(&png), TextureDecodeLimits::default())
                .map_err(|e| e.to_string())?;
        digests[role] = ingested.digest;
        library
            .images
            .entry((ingested.digest, role as u8))
            .or_insert(Image {
                digest: ingested.digest,
                bytes: ingested.bytes,
                role: role as u8,
            });
    }
    library.surfaces.insert(family, SurfaceSet { digests });
    Ok(digests)
}

pub fn material(recipe: &AssetRecipe) -> Result<MaterialRecord, String> {
    let family = recipe.material_family;
    let mut material = MaterialRecord::legacy_default();
    material.id = qualia_core_db::q_hash(&format!("urn:rolling-commons:material:{}", family.id));
    material.base_color = [1.0; 4];
    material.multiply_vertex_color = true;
    material.metallic = family.metallic;
    material.roughness = family.roughness;
    material.wind_animated = family.id == "living-foliage";
    if family.id == "framed-glass" {
        material.opacity_mode = OpacityMode::Blend;
        material.base_color[3] = 0.44;
        material.double_sided = true;
        material.casts_shadow = false;
    } else if recipe.render_role != "water" {
        let maps = LIBRARY.with(|library| ensure_surface(&mut library.borrow_mut(), family.id))?;
        material.base_color_texture = maps[0];
        material.normal_texture = maps[1];
        material.metallic_roughness_texture = maps[2];
        material.occlusion_texture = maps[2];
        material.normal_scale = if family.id == "living-foliage" {
            0.30
        } else {
            0.65
        };
    }
    Ok(material)
}

pub struct PreparedSurface {
    pub mesh: qualia_core_db::render::assets::Mesh,
    pub normals: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    /// Original mesh vertex indices, retained when UV seams split vertices.
    pub source_vertices: Vec<u32>,
}

pub fn prepare_surface(
    recipe: &AssetRecipe,
    mesh: qualia_core_db::render::assets::Mesh,
) -> Result<PreparedSurface, String> {
    use qualia_core_db::render::mesh_surface_frames::{
        box_project_mesh_with_sources_into, vertex_normals_into,
    };
    let scale = match recipe.material_family.id {
        "soil-and-gravel" => 1.5,
        "cloth-and-skin" | "shade-canvas" => 0.5,
        "weathered-timber" => 0.8,
        _ => 1.0,
    };
    let count = mesh
        .triangles
        .len()
        .checked_mul(3)
        .filter(|n| *n <= 262_144)
        .ok_or_else(|| "surface chart topology exceeds authoring budget".to_owned())?;
    let mut source_normals = vec![[0.0; 3]; mesh.positions.len()];
    vertex_normals_into(&mesh.positions, &mesh.triangles, &mut source_normals)
        .map_err(|e| format!("source normals: {e:?}"))?;
    let mut positions = vec![[0.0; 3]; count];
    let mut triangles = vec![[0; 3]; mesh.triangles.len()];
    let mut normals = vec![[0.0; 3]; count];
    let mut uv = vec![[0.0; 2]; count];
    let mut charts = vec![[u32::MAX; 6]; mesh.positions.len()];
    let mut source_vertices = vec![0; count];
    let mut surface_kind = recipe.parametric.as_ref();
    while let Some(
        ParametricRecipe::Transform { source, .. }
        | ParametricRecipe::Yawed { source, .. }
        | ParametricRecipe::Oriented { source, .. },
    ) = surface_kind
    {
        surface_kind = Some(source.as_ref());
    }
    let smooth = !matches!(
        surface_kind,
        None | Some(ParametricRecipe::PanelGroup { .. }) | Some(ParametricRecipe::TiltedBox { .. })
    );
    let receipt = box_project_mesh_with_sources_into(
        &mesh.positions,
        &mesh.triangles,
        &source_normals,
        scale,
        smooth,
        &mut positions,
        &mut triangles,
        &mut normals,
        &mut uv,
        &mut charts,
        Some(&mut source_vertices),
    )
    .map_err(|e| format!("surface charts: {e:?}"))?;
    positions.truncate(receipt.vertex_count);
    normals.truncate(receipt.vertex_count);
    uv.truncate(receipt.vertex_count);
    source_vertices.truncate(receipt.vertex_count);
    Ok(PreparedSurface {
        mesh: qualia_core_db::render::assets::Mesh {
            positions,
            triangles,
            min: mesh.min,
            max: mesh.max,
        },
        normals,
        uv,
        source_vertices,
    })
}

pub fn texture_receipts() -> Result<js_sys::Array, JsValue> {
    LIBRARY.with(|library| {
        let images = js_sys::Array::new();
        for image in library.borrow().images.values() {
            let object = js_sys::Object::new();
            let key = qualia_core_db::render::asset_package::texture_hmc_key(&image.digest);
            js_sys::Reflect::set(&object, &"key".into(), &key.into())?;
            js_sys::Reflect::set(&object, &"mime".into(), &"image/png".into())?;
            js_sys::Reflect::set(
                &object,
                &"role".into(),
                &JsValue::from_f64(image.role as f64),
            )?;
            js_sys::Reflect::set(
                &object,
                &"digest".into(),
                &js_sys::Uint8Array::from(image.digest.as_slice()),
            )?;
            js_sys::Reflect::set(
                &object,
                &"bytes".into(),
                &js_sys::Uint8Array::from(image.bytes.as_slice()),
            )?;
            images.push(&object);
        }
        Ok(images)
    })
}

pub fn hydrate(
    portal: &mut qualia_core_db::QualiaPortal,
    textures: &js_sys::Array,
) -> Result<u32, JsValue> {
    if textures.length() > 96 {
        return Err(JsValue::from_str("texture dependency count exceeds bound"));
    }
    let mut loaded = 0;
    for texture in textures.iter() {
        let get = |key: &str| js_sys::Reflect::get(&texture, &key.into());
        let digest = js_sys::Uint8Array::new(&get("digest")?).to_vec();
        let bytes = js_sys::Uint8Array::new(&get("bytes")?).to_vec();
        let mime = get("mime")?
            .as_string()
            .ok_or_else(|| JsValue::from_str("texture MIME missing"))?;
        let role = get("role")?
            .as_f64()
            .filter(|n| n.fract() == 0.0 && (0.0..=3.0).contains(n))
            .ok_or_else(|| JsValue::from_str("texture role invalid"))? as u8;
        if portal.load_texture_encoded(&digest, &mime, &bytes, role, 0.5)? {
            loaded += 1;
        }
    }
    Ok(loaded)
}
