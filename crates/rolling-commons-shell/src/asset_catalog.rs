//! Original Kestrel Flats source recipes. QualiaDB owns mesh assembly and `.10d`.
//! These are authored game assets, separate from the engine and its licence.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::scene_primitives::Primitive;
use qualia_core_db::specialized_libs::computational_geometry::{authoring, parametric_cad, Point3};

/// Game-owned source parameters; Qualia's public computational geometry
/// builds the mesh before the shared `.10d` compiler seals it.
#[derive(Debug, Clone)]
pub enum ParametricRecipe {
    Cylinder {
        center: [f32; 3],
        radius: f32,
        height: f32,
        segments: u32,
    },
    Sphere {
        center: [f32; 3],
        radius: f32,
        latitude: u32,
        longitude: u32,
    },
    Torus {
        center: [f32; 3],
        major: f32,
        minor: f32,
        segments: u32,
        tube_segments: u32,
    },
    Revolve {
        center: [f32; 3],
        profile: &'static [[f32; 2]],
        segments: usize,
    },
    /// Figure sealed as one mesh. Head, shoulders, and stance are geometry,
    /// not a box and not a who-kind. `light` 0 none, 1 hard act-light, 2 halo.
    Figure {
        center: [f32; 3],
        kind: u32,
        phase: f32,
        marked: bool,
        yaw: f32,
        light: u32,
        /// 0 nothing in the hands. 1 a carried bundle. Not a wrecked body.
        carry: u32,
    },
    /// Compound model on the geometry path (lathe, sphere, cylinder, torus).
    /// Not a box. `kind` picks the silhouette; nothing here borrows the tank.
    Rig {
        center: [f32; 3],
        kind: u32,
        span: f32,
    },
}

impl ParametricRecipe {
    pub fn source_bytes(&self) -> Vec<u8> {
        format!("RC-PARAMETRIC/1;{self:?}").into_bytes()
    }

    pub fn compile(&self) -> Result<Mesh, String> {
        let (center, mesh) = match self {
            Self::Cylinder {
                center,
                radius,
                height,
                segments,
            } => (
                *center,
                authoring::cylinder(*radius, *height, *segments).map_err(|e| e.to_string())?,
            ),
            Self::Sphere {
                center,
                radius,
                latitude,
                longitude,
            } => (
                *center,
                authoring::uv_sphere(*radius, *latitude, *longitude).map_err(|e| e.to_string())?,
            ),
            Self::Torus {
                center,
                major,
                minor,
                segments,
                tube_segments,
            } => (
                *center,
                authoring::torus(*major, *minor, *segments, *tube_segments)
                    .map_err(|e| e.to_string())?,
            ),
            Self::Figure {
                center,
                kind,
                phase,
                marked,
                yaw,
                light,
                carry,
            } => {
                return compile_figure(*center, *kind, *phase, *marked, *yaw, *light, *carry);
            }
            Self::Rig { center, kind, span } => {
                return compile_rig(*center, *kind, *span);
            }
            Self::Revolve {
                center,
                profile,
                segments,
            } => {
                let profile: Vec<Point3> = profile
                    .iter()
                    .map(|p| Point3::new(p[0] as f64, p[1] as f64, 0.0))
                    .collect();
                let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); profile.len() * *segments];
                let mut triangles = vec![[0u32; 3]; (profile.len() - 1) * *segments * 2];
                let (nv, nt) = parametric_cad::revolve_profile(
                    &profile,
                    *segments,
                    &mut vertices,
                    &mut triangles,
                )
                .map_err(|e| format!("revolve: {e:?}"))?;
                let positions: Vec<[f32; 3]> = vertices[..nv]
                    .iter()
                    .map(|p| [p.x as f32, p.y as f32, p.z as f32])
                    .collect();
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for p in &positions {
                    for axis in 0..3 {
                        min[axis] = min[axis].min(p[axis]);
                        max[axis] = max[axis].max(p[axis]);
                    }
                }
                (
                    *center,
                    Mesh {
                        positions,
                        triangles: triangles[..nt].to_vec(),
                        min,
                        max,
                    },
                )
            }
        };
        Ok(authoring::transform_mesh(
            &mesh,
            &authoring::translation(center[0] as f64, center[1] as f64, center[2] as f64),
        ))
    }
}

pub struct AssetRecipe {
    pub id: &'static str,
    pub parts: Vec<Primitive>,
    pub parametric: Option<ParametricRecipe>,
    pub color: [f32; 4],
    pub vibe_source: Option<String>,
    /// Material signature on the part, when the colour is a spectrum reading.
    pub shell_signature: Option<&'static str>,
    /// `colour` or `sound`. Not a baked texture id.
    pub spectrum_reading: Option<&'static str>,
}

fn block(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Box { center, size }
}

fn roof(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Roof { center, size }
}

fn asset(id: &'static str, color: [f32; 4], parts: Vec<Primitive>) -> AssetRecipe {
    AssetRecipe {
        id,
        parts,
        parametric: None,
        color,
        vibe_source: None,
        shell_signature: None,
        spectrum_reading: None,
    }
}

fn parametric(id: &'static str, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(recipe),
        color,
        vibe_source: None,
        shell_signature: None,
        spectrum_reading: None,
    }
}

/// Display colour of the water-tank **shell** from the HDPE optical spectrum
/// reading. Water state does not swap this. Sucrose (albedo 0.85) is not it.
/// Alpha stays opaque so the silhouette stays readable at both zooms.
pub fn hdpe_shell_colour() -> [f32; 4] {
    let sig = vibe::physics::MaterialSignature::lookup("hdpe_tank_shell")
        .expect("hdpe tank shell is on the physics stack");
    assert_ne!(sig.name, "Sucrose Cube", "shell must not read as sugar");
    let optical = sig
        .optical
        .as_ref()
        .expect("colour is the optical reading of the one spectrum axis");
    let albedo = optical.albedo as f32;
    let absorb = optical.absorption as f32;
    // The reading itself: diffuse reflectance after absorption. Not a texture
    // id and not the old cyan/grey water-online swap. Sucrose albedo is 0.85;
    // this shell stays at or below its own 0.18 albedo.
    let shade = (albedo * (1.0 - absorb)).clamp(0.0, albedo);
    let ior_cool = ((optical.ior as f32) - 1.50).clamp(0.0, 0.08);
    [
        shade,
        (shade + ior_cool * albedo).min(albedo),
        (shade + ior_cool * albedo * 0.5).min(albedo),
        1.0,
    ]
}



fn mesh_bounds(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for a in 0..3 {
            min[a] = min[a].min(p[a]);
            max[a] = max[a].max(p[a]);
        }
    }
    (min, max)
}

fn append_mesh(dst: &mut Mesh, src: &Mesh) {
    let base = dst.positions.len() as u32;
    dst.positions.extend_from_slice(&src.positions);
    for t in &src.triangles {
        dst.triangles.push([t[0] + base, t[1] + base, t[2] + base]);
    }
}

fn empty_mesh() -> Mesh {
    Mesh {
        positions: Vec::new(),
        triangles: Vec::new(),
        min: [0.0; 3],
        max: [0.0; 3],
    }
}

/// Limb from the geometry library. Pivot is the hip or shoulder; swing is
/// motion on the part, not a rigid pop of a box.
fn limb(radius: f32, length: f32, segments: u32, pivot: [f32; 3], rx: f64, rz: f64) -> Result<Mesh, String> {
    let cyl = authoring::cylinder(radius, length, segments).map_err(|e| e.to_string())?;
    let hy = f64::from(length) * 0.5;
    let spun = authoring::mat_mul(
        &authoring::rotation_z(rz),
        &authoring::mat_mul(
            &authoring::rotation_x(rx),
            &authoring::translation(0.0, -hy, 0.0),
        ),
    );
    let world = authoring::mat_mul(
        &authoring::translation(f64::from(pivot[0]), f64::from(pivot[1]), f64::from(pivot[2])),
        &spun,
    );
    Ok(authoring::transform_mesh(&cyl, &world))
}

fn ring_at(major: f32, minor: f32, at: [f32; 3]) -> Result<Mesh, String> {
    let major = major.max(minor + 0.01);
    let torus = authoring::torus(major, minor, 16, 6).map_err(|e| e.to_string())?;
    Ok(authoring::transform_mesh(
        &torus,
        &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
    ))
}

struct Cut {
    torso_h: f32,
    hip: f32,
    waist: f32,
    chest: f32,
    shoulder: f32,
    neck: f32,
    leg: f32,
    leg_r: f32,
    stance: f32,
    head: f32,
    arm: f32,
    hat: f32,
}

fn cut_of(kind: u32) -> Cut {
    match kind % 6 {
        1 => Cut { torso_h: 0.50, hip: 0.16, waist: 0.17, chest: 0.24, shoulder: 0.36, neck: 0.07, leg: 0.46, leg_r: 0.075, stance: 0.24, head: 0.15, arm: 0.34, hat: 0.0 },
        2 => Cut { torso_h: 0.78, hip: 0.22, waist: 0.14, chest: 0.16, shoulder: 0.22, neck: 0.05, leg: 0.55, leg_r: 0.05, stance: 0.30, head: 0.11, arm: 0.40, hat: 0.0 },
        3 => Cut { torso_h: 0.58, hip: 0.11, waist: 0.10, chest: 0.14, shoulder: 0.19, neck: 0.05, leg: 0.74, leg_r: 0.05, stance: 0.15, head: 0.12, arm: 0.46, hat: 0.22 },
        4 => Cut { torso_h: 0.42, hip: 0.18, waist: 0.20, chest: 0.22, shoulder: 0.28, neck: 0.07, leg: 0.34, leg_r: 0.085, stance: 0.22, head: 0.16, arm: 0.26, hat: 0.0 },
        5 => Cut { torso_h: 0.92, hip: 0.15, waist: 0.12, chest: 0.15, shoulder: 0.21, neck: 0.05, leg: 0.70, leg_r: 0.045, stance: 0.22, head: 0.11, arm: 0.48, hat: 0.0 },
        _ => Cut { torso_h: 0.64, hip: 0.10, waist: 0.09, chest: 0.13, shoulder: 0.20, neck: 0.055, leg: 0.84, leg_r: 0.05, stance: 0.14, head: 0.12, arm: 0.50, hat: 0.0 },
    }
}

/// One figure. Same computational-geometry path as the tank: a spun torso,
/// a sphere head, cylindrical limbs. The `.10d` holds this mesh, not a box.
fn compile_figure(
    center: [f32; 3],
    kind: u32,
    phase: f32,
    marked: bool,
    yaw: f32,
    light: u32,
    carry: u32,
) -> Result<Mesh, String> {
    let c = cut_of(kind);
    let swing = phase.sin();
    let bob = swing.abs() * 0.03;
    // Poses differ by cut so two figures are not one walk cycle in duplicate.
    // 0 stride, 1 weight on one leg with an arm out, others a quieter step.
    let (leg_l, leg_r, arm_l, arm_r) = if carry > 0 {
        // Carrying: arms in, short step. Not the walking stride and not a broken body.
        (f64::from(swing) * 0.22, f64::from(-swing) * 0.18, -1.15, -0.8)
    } else {
        match kind % 6 {
            1 => (0.12_f64, -0.42, 0.55, -1.15),
            _ => (
                f64::from(swing) * 0.8,
                f64::from(-swing) * 0.8,
                f64::from(-swing) * 0.5,
                f64::from(swing) * 0.5,
            ),
        }
    };
    let profile_pts = [
        Point3::new(f64::from(c.hip), 0.0, 0.0),
        Point3::new(f64::from(c.waist), f64::from(c.torso_h * 0.32), 0.0),
        Point3::new(f64::from(c.chest), f64::from(c.torso_h * 0.58), 0.0),
        Point3::new(f64::from(c.shoulder), f64::from(c.torso_h * 0.80), 0.0),
        Point3::new(f64::from(c.neck), f64::from(c.torso_h * 0.94), 0.0),
        Point3::new(0.0, f64::from(c.torso_h), 0.0),
    ];
    let segments = 32usize;
    let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); profile_pts.len() * segments];
    let mut triangles = vec![[0u32; 3]; (profile_pts.len() - 1) * segments * 2];
    let (nv, nt) = parametric_cad::revolve_profile(&profile_pts, segments, &mut vertices, &mut triangles)
        .map_err(|e| format!("torso: {e:?}"))?;
    let positions: Vec<[f32; 3]> = vertices[..nv].iter().map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let (min, max) = mesh_bounds(&positions);
    let torso_local = Mesh { positions, triangles: triangles[..nt].to_vec(), min, max };
    let torso = authoring::transform_mesh(&torso_local, &authoring::translation(0.0, f64::from(c.leg), 0.0));
    let head_y = c.leg + c.torso_h + c.head * 0.72;
    let head = authoring::transform_mesh(
        &authoring::uv_sphere(c.head, 12, 18).map_err(|e| e.to_string())?,
        &authoring::translation(0.0, f64::from(head_y), 0.0),
    );
    let hip_y = c.leg * 0.98;
    let left_leg = limb(c.leg_r, c.leg, 16, [-c.stance, hip_y, 0.0], leg_l, 0.06)?;
    let right_leg = limb(c.leg_r, c.leg, 16, [c.stance, hip_y, 0.0], leg_r, -0.06)?;
    let shoulder_y = c.leg + c.torso_h * 0.78;
    let left_arm = limb(c.leg_r * 0.7, c.arm, 14, [-(c.shoulder + 0.02), shoulder_y, 0.0], arm_l, 0.4)?;
    let right_arm = limb(c.leg_r * 0.7, c.arm, 14, [c.shoulder + 0.02, shoulder_y, 0.0], arm_r, -0.28)?;
    // What they carry. A whole person holding a bundle, never a broken body.
    // Buried in the torso when their hands are empty so every figure stays one topology.
    const BUNDLE: &[[f32; 2]] = &[
        [0.0, -0.16],
        [0.14, -0.12],
        [0.16, 0.02],
        [0.08, 0.14],
        [0.0, 0.16],
    ];
    let sack = ParametricRecipe::Revolve {
        center: [0.0, 0.0, 0.0],
        profile: BUNDLE,
        segments: 18,
    }
    .compile()?;
    let bundle = if carry > 0 {
        authoring::transform_mesh(
            &sack,
            &authoring::translation(
                f64::from(c.shoulder * 0.15),
                f64::from(c.leg + c.torso_h * 0.42),
                0.22,
            ),
        )
    } else {
        authoring::transform_mesh(
            &sack,
            &authoring::mat_mul(
                &authoring::translation(0.0, f64::from(c.leg + c.torso_h * 0.5), 0.0),
                &authoring::scale(0.15, 0.15, 0.15),
            ),
        )
    };
    // Quiet direct-mark. Buried when not directing. Never a foot pad.
    let mark = if marked {
        ring_at(c.head * 0.95, 0.018, [0.0, head_y + c.head + 0.06, 0.0])?
    } else {
        ring_at(0.05, 0.012, [0.0, c.leg + c.torso_h * 0.5, 0.0])?
    };
    // Hat brim or a buried ring so every cut has the same part count.
    let brim = if c.hat > 0.05 {
        ring_at(c.hat, 0.02, [0.0, head_y + c.head * 0.35, 0.0])?
    } else {
        ring_at(0.05, 0.012, [0.0, head_y, 0.0])?
    };
    // Hard light only for a villain act. A condition such as nowhere to go
    // keeps this ring inside the torso so it does not read as guilt.
    let act_ring = match light {
        1 => ring_at((c.shoulder + 0.05).max(0.16), 0.022, [0.0, c.leg + c.torso_h * 0.72, 0.0])?,
        2 => ring_at(c.head * 1.7, 0.016, [0.0, head_y + c.head + 0.02, 0.0])?,
        _ => ring_at(0.05, 0.012, [0.0, c.leg + c.torso_h * 0.45, 0.0])?,
    };
    let mut mesh = empty_mesh();
    for part in [&torso, &head, &left_leg, &right_leg, &left_arm, &right_arm, &bundle, &mark, &brim, &act_ring] {
        append_mesh(&mut mesh, part);
    }
    let placed = authoring::transform_mesh(
        &mesh,
        &authoring::translation(f64::from(center[0]), f64::from(center[1] + bob), f64::from(center[2])),
    );
    let yawed = authoring::transform_mesh(
        &placed,
        &authoring::mat_mul(
            &authoring::translation(f64::from(center[0]), 0.0, f64::from(center[2])),
            &authoring::mat_mul(
                &authoring::rotation_y(f64::from(yaw)),
                &authoring::translation(-f64::from(center[0]), 0.0, -f64::from(center[2])),
            ),
        ),
    );
    let (min, max) = mesh_bounds(&yawed.positions);
    Ok(Mesh {
        positions: yawed.positions,
        triangles: yawed.triangles,
        min,
        max,
    })
}

/// Anchor used to keep Saltwind geometry out of the Kestrel place.
pub fn recipe_anchor_x(recipe: &AssetRecipe) -> f32 {
    if let Some(spec) = &recipe.parametric {
        return match spec {
            ParametricRecipe::Cylinder { center, .. }
            | ParametricRecipe::Sphere { center, .. }
            | ParametricRecipe::Torus { center, .. }
            | ParametricRecipe::Revolve { center, .. }
            | ParametricRecipe::Figure { center, .. }
            | ParametricRecipe::Rig { center, .. } => center[0],
        };
    }
    recipe
        .parts
        .first()
        .map(|p| match p {
            Primitive::Box { center, .. } | Primitive::Roof { center, .. } => center[0],
        })
        .unwrap_or(0.0)
}

/// Fictional participants. Not likenesses, not a chatbot, not a who-kind.
/// `party` records are `x,z,r,g,b,shape,mark,phase,yaw,light` separated by `;`.
/// Soft roster 24. The mesh is a figure, sealed as `.10d`, not a box.
pub fn participant_markers(party: &str) -> Vec<AssetRecipe> {
    const IDS: [&str; 24] = [
        "rc:participant/0", "rc:participant/1", "rc:participant/2", "rc:participant/3",
        "rc:participant/4", "rc:participant/5", "rc:participant/6", "rc:participant/7",
        "rc:participant/8", "rc:participant/9", "rc:participant/10", "rc:participant/11",
        "rc:participant/12", "rc:participant/13", "rc:participant/14", "rc:participant/15",
        "rc:participant/16", "rc:participant/17", "rc:participant/18", "rc:participant/19",
        "rc:participant/20", "rc:participant/21", "rc:participant/22", "rc:participant/23",
    ];
    let mut out = Vec::new();
    for (n, rec) in party.split(';').take(IDS.len()).enumerate() {
        if rec.trim().is_empty() {
            continue;
        }
        let nums: Vec<f32> = rec.split(',').filter_map(|s| s.trim().parse().ok()).collect();
        if nums.len() < 6 {
            continue;
        }
        let color = [nums[2].clamp(0.0, 1.0), nums[3].clamp(0.0, 1.0), nums[4].clamp(0.0, 1.0), 1.0];
        out.push(AssetRecipe {
            id: IDS[n],
            parts: Vec::new(),
            parametric: Some(ParametricRecipe::Figure {
                center: [nums[0], 0.0, nums[1]],
                kind: nums[5] as u32,
                phase: nums.get(7).copied().unwrap_or(0.0),
                marked: nums.get(6).copied().unwrap_or(0.0) > 0.5,
                yaw: nums.get(8).copied().unwrap_or(0.0),
                light: nums.get(9).copied().unwrap_or(0.0) as u32,
                carry: nums.get(10).copied().unwrap_or(0.0) as u32,
            }),
            color,
            vibe_source: Some(part_record(
                IDS[n],
                "did:webizen:game:participant-figure-v1",
                "Fictional participant figure",
                color[0],
                0.22,
                1.46,
                0.35,
            )),
            shell_signature: Some("did:webizen:game:participant-figure-v1"),
            spectrum_reading: Some("colour"),
        });
    }
    out
}

/// First-pass blockout kit. Keep IDs stable when replacing geometry with
/// finished, independently packaged assets. Dimensions are scene metres.
pub fn kestrel_flats(
    upgrades: u32,
    panels: u32,
    online: bool,
    approved: bool,
    water_online: bool,
    garden_active: bool,
) -> Vec<AssetRecipe> {
    const TANK_PROFILE: &[[f32; 2]] = &[
        [0.0, -0.66],
        [0.48, -0.66],
        [0.62, -0.49],
        [0.62, 0.42],
        [0.51, 0.61],
        [0.0, 0.61],
    ];
    let mut scene = Vec::with_capacity(120);
    scene.push(asset(
        "rc:asset/ground",
        [0.62, 0.78, 0.40, 1.0],
        vec![block([0.0, -0.08, 0.0], [14.0, 0.16, 14.0])],
    ));
    scene.push(asset(
        "rc:asset/main-road",
        [0.78, 0.55, 0.37, 1.0],
        vec![
            block([0.0, 0.01, 0.5], [12.5, 0.04, 1.2]),
            block([1.6, 0.012, -1.8], [1.1, 0.04, 5.5]),
        ],
    ));
    scene.push(parametric(
        "rc:asset/camp-shelter",
        [0.96, 0.53, 0.35, 1.0],
        ParametricRecipe::Rig {
            center: [-3.0, 0.0, 2.6],
            kind: 10,
            span: 1.15 + upgrades.min(8) as f32 * 0.06,
        },
    ));
    scene.push(asset(
        "rc:asset/camp-platform",
        [0.62, 0.39, 0.25, 1.0],
        vec![
            block([-3.0, -0.01, 2.7], [2.8, 0.12, 2.5]),
            block([-4.2, 0.25, 2.7], [0.08, 0.5, 0.08]),
            block([-1.8, 0.25, 2.7], [0.08, 0.5, 0.08]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop",
        if online {
            [0.99, 0.77, 0.40, 1.0]
        } else {
            [0.93, 0.65, 0.41, 1.0]
        },
        vec![
            block([3.2, 0.8, -2.2], [2.4, 1.6, 1.8]),
            roof([3.2, 1.6, -2.2], [2.65, 0.5, 2.05]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-doors",
        [0.24, 0.42, 0.52, 1.0],
        vec![
            block([3.2, 0.65, -1.27], [1.15, 1.3, 0.05]),
            block([2.55, 0.8, -1.25], [0.07, 1.6, 0.07]),
            block([3.85, 0.8, -1.25], [0.07, 1.6, 0.07]),
        ],
    ));
    if panels > 0 {
        scene.push(asset(
            "rc:asset/solar-array",
            [0.10, 0.27, 0.47, 1.0],
            (0..panels.min(8))
                .map(|i| block([2.6 + i as f32 * 1.2, 1.96, -2.2], [1.0, 0.07, 1.3]))
                .collect(),
        ));
    }
    scene.push(asset(
        "rc:asset/market-stall",
        [0.98, 0.64, 0.31, 1.0],
        vec![
            block([-2.2, 0.52, -3.4], [1.6, 1.0, 1.0]),
            roof([-2.2, 1.05, -3.4], [2.0, 0.42, 1.4]),
            block([-2.2, 0.72, -2.85], [1.7, 0.12, 0.5]),
        ],
    ));
    scene.push(asset(
        "rc:asset/salvage-pile",
        [0.46, 0.33, 0.23, 1.0],
        vec![
            block([4.0, 0.3, 3.2], [1.0, 0.6, 0.9]),
            block([4.6, 0.5, 3.5], [0.7, 1.0, 0.6]),
            block([3.5, 0.45, 3.9], [0.8, 0.9, 0.7]),
        ],
    ));
    scene.push(asset(
        "rc:asset/salvage-rack",
        [0.32, 0.34, 0.33, 1.0],
        vec![
            block([5.0, 0.65, 2.6], [0.08, 1.3, 0.08]),
            block([6.0, 0.65, 2.6], [0.08, 1.3, 0.08]),
            block([5.5, 1.2, 2.6], [1.1, 0.08, 0.6]),
            block([5.5, 0.7, 2.6], [1.1, 0.08, 0.6]),
        ],
    ));
    scene.push(asset(
        "rc:asset/committee-sign",
        if approved {
            [0.20, 0.59, 0.32, 1.0]
        } else {
            [0.59, 0.59, 0.56, 1.0]
        },
        vec![
            block([0.0, 1.0, -0.6], [0.07, 2.0, 0.07]),
            block([0.25, 1.85, -0.6], [0.5, 0.3, 0.05]),
        ],
    ));
    // Shell colour is the HDPE spectrum reading. Water online does not swap it.
    // The van stays a later asset. Licence stays in .10d provenance, not on the mesh.
    let _ = water_online;
    let mut tank = parametric(
        "rc:asset/water-tank",
        hdpe_shell_colour(),
        ParametricRecipe::Revolve {
            center: [-5.0, 1.02, -0.5],
            profile: TANK_PROFILE,
            // Dense enough that the rim reads up close and the belly reads zoomed out.
            segments: 48,
        },
    );
    tank.shell_signature = Some("did:q42:material:hdpe-tank-shell-v1");
    tank.spectrum_reading = Some("colour");
    tank.vibe_source = Some(part_record(
        "rc:asset/water-tank",
        "did:q42:material:hdpe-tank-shell-v1",
        "HDPE water-tank shell",
        0.18,
        0.40,
        1.54,
        0.08,
    ));
    scene.push(tank);
    scene.extend(authored_camp());
    scene.push(asset(
        "rc:asset/tank-stand",
        [0.31, 0.30, 0.27, 1.0],
        vec![
            block([-5.0, 0.1, -0.5], [1.35, 0.2, 1.35]),
            block([-4.45, 0.35, -0.5], [0.08, 0.6, 0.08]),
            block([-5.55, 0.35, -0.5], [0.08, 0.6, 0.08]),
        ],
    ));
    scene.push(asset(
        "rc:asset/garden-beds",
        if garden_active {
            [0.29, 0.54, 0.19, 1.0]
        } else {
            [0.36, 0.27, 0.18, 1.0]
        },
        vec![
            block([-4.8, 0.09, -4.2], [1.4, 0.18, 0.55]),
            block([-4.8, 0.09, -3.3], [1.4, 0.18, 0.55]),
            block([-3.0, 0.09, -4.2], [1.4, 0.18, 0.55]),
            block([-3.0, 0.09, -3.3], [1.4, 0.18, 0.55]),
        ],
    ));
    scene.push(asset(
        "rc:asset/garden-fence",
        [0.55, 0.39, 0.23, 1.0],
        vec![
            block([-3.9, 0.35, -4.8], [3.9, 0.07, 0.07]),
            block([-3.9, 0.35, -2.7], [3.9, 0.07, 0.07]),
            block([-5.8, 0.35, -3.75], [0.07, 0.07, 2.2]),
        ],
    ));
    scene.push(asset(
        "rc:asset/energy-battery",
        [0.69, 0.62, 0.37, 1.0],
        vec![
            block([5.55, 0.4, -3.8], [0.8, 0.8, 0.55]),
            block([5.55, 0.85, -3.8], [0.9, 0.1, 0.65]),
        ],
    ));
    scene.push(asset(
        "rc:asset/utility-cable",
        [0.24, 0.25, 0.23, 1.0],
        vec![
            block([4.8, 0.06, -3.1], [1.6, 0.07, 0.07]),
            block([4.0, 0.06, -2.65], [0.07, 0.07, 0.95]),
        ],
    ));
    scene.push(asset(
        "rc:asset/repair-van",
        [0.57, 0.63, 0.55, 1.0],
        vec![
            block([0.0, 0.56, 4.55], [1.8, 0.85, 0.85]),
            block([0.6, 0.87, 4.55], [0.55, 0.45, 0.85]),
            block([-0.55, 0.16, 4.12], [0.28, 0.3, 0.12]),
            block([0.55, 0.16, 4.12], [0.28, 0.3, 0.12]),
        ],
    ));
    scene.push(asset(
        "rc:asset/van-windows",
        [0.12, 0.27, 0.34, 1.0],
        vec![
            block([0.68, 0.9, 4.08], [0.36, 0.27, 0.04]),
            block([-0.23, 0.8, 4.08], [0.44, 0.25, 0.04]),
        ],
    ));
    scene.push(asset(
        "rc:asset/community-hall",
        [0.99, 0.78, 0.54, 1.0],
        vec![
            block([0.0, 0.65, -5.25], [2.2, 1.3, 1.2]),
            roof([0.0, 1.3, -5.25], [2.45, 0.55, 1.45]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hall-entry",
        [0.27, 0.36, 0.31, 1.0],
        vec![
            block([0.0, 0.45, -4.62], [0.55, 0.9, 0.04]),
            block([0.0, 0.02, -4.35], [0.9, 0.05, 0.6]),
        ],
    ));
    scene.push(asset(
        "rc:asset/communications-mast",
        [0.57, 0.58, 0.54, 1.0],
        vec![
            block([5.75, 1.3, 5.3], [0.1, 2.6, 0.1]),
            block([5.75, 2.35, 5.3], [0.8, 0.06, 0.06]),
            block([5.75, 1.7, 5.3], [0.55, 0.06, 0.06]),
        ],
    ));
    scene.push(asset(
        "rc:asset/mast-node",
        [0.25, 0.48, 0.52, 1.0],
        vec![block([5.75, 2.55, 5.3], [0.26, 0.32, 0.26])],
    ));
    // Landscape and architectural detail remain game-owned source recipes.
    // The rounded forms below use Qualia's authoring and parametric CAD APIs.
    scene.push(asset(
        "rc:asset/grass-verges",
        [0.40, 0.70, 0.32, 1.0],
        vec![
            block([-3.8, 0.012, 5.6], [4.0, 0.035, 1.35]),
            block([3.7, 0.012, 5.6], [4.5, 0.035, 1.35]),
            block([-3.5, 0.012, -6.2], [5.2, 0.035, 0.75]),
            block([4.0, 0.012, -6.2], [4.5, 0.035, 0.75]),
        ],
    ));
    scene.push(asset(
        "rc:asset/footpaths",
        [0.94, 0.80, 0.59, 1.0],
        vec![
            block([-4.1, 0.04, 1.15], [0.72, 0.055, 4.2]),
            block([-3.85, 0.04, -1.7], [2.6, 0.055, 0.62]),
            block([3.2, 0.04, -0.9], [0.68, 0.055, 1.75]),
            block([0.0, 0.04, -3.9], [0.62, 0.055, 2.1]),
        ],
    ));
    scene.push(asset(
        "rc:asset/road-markings",
        [0.88, 0.79, 0.48, 1.0],
        (-5..=5)
            .step_by(2)
            .map(|x| block([x as f32, 0.041, 0.5], [0.7, 0.012, 0.045]))
            .collect(),
    ));
    scene.push(asset(
        "rc:asset/workshop-roof-panels",
        [0.18, 0.27, 0.32, 1.0],
        vec![
            block([3.2, 1.59, -2.2], [2.74, 0.07, 2.12]),
            block([3.2, 1.81, -2.2], [2.52, 0.06, 0.13]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-windows",
        if online {
            [0.98, 0.78, 0.38, 1.0]
        } else {
            [0.24, 0.46, 0.54, 1.0]
        },
        vec![
            block([2.43, 1.0, -1.23], [0.38, 0.43, 0.045]),
            block([3.97, 1.0, -1.23], [0.38, 0.43, 0.045]),
            block([4.43, 0.95, -2.2], [0.045, 0.42, 0.75]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-sign",
        [0.91, 0.64, 0.28, 1.0],
        vec![block([3.2, 1.49, -1.19], [0.9, 0.18, 0.045])],
    ));
    if panels > 0 {
        scene.push(asset(
            "rc:asset/solar-cells",
            [0.13, 0.57, 0.80, 1.0],
            (0..panels.min(8))
                .flat_map(|i| {
                    (0..3).map(move |j| {
                        block(
                            [2.25 + i as f32 * 1.2 + j as f32 * 0.34, 2.015, -2.2],
                            [0.26, 0.015, 1.02],
                        )
                    })
                })
                .collect(),
        ));
    }
    scene.push(asset(
        "rc:asset/market-canopy",
        [0.96, 0.35, 0.32, 1.0],
        vec![
            block([-2.75, 1.35, -3.4], [0.45, 0.05, 1.5]),
            block([-1.88, 1.35, -3.4], [0.45, 0.05, 1.5]),
            block([-2.2, 1.17, -2.61], [2.1, 0.11, 0.09]),
        ],
    ));
    scene.push(asset(
        "rc:asset/market-produce",
        [0.88, 0.61, 0.19, 1.0],
        vec![
            block([-2.68, 0.84, -2.78], [0.35, 0.18, 0.29]),
            block([-2.13, 0.84, -2.78], [0.35, 0.18, 0.29]),
            block([-1.59, 0.84, -2.78], [0.35, 0.18, 0.29]),
        ],
    ));
    scene.push(asset(
        "rc:asset/camp-deck-boards",
        [0.67, 0.48, 0.28, 1.0],
        (0..5)
            .map(|i| block([-3.0, 0.06, 1.75 + i as f32 * 0.46], [2.65, 0.025, 0.39]))
            .collect(),
    ));
    scene.push(asset(
        "rc:asset/garden-soil",
        [0.19, 0.15, 0.11, 1.0],
        [-4.2, -3.3]
            .into_iter()
            .flat_map(|z| {
                [-4.8, -3.0]
                    .into_iter()
                    .map(move |x| block([x, 0.195, z], [1.24, 0.035, 0.43]))
            })
            .collect(),
    ));
    if garden_active {
        scene.push(asset(
            "rc:asset/garden-shoots",
            [0.27, 0.78, 0.26, 1.0],
            [-4.2, -3.3]
                .into_iter()
                .flat_map(|z| {
                    [-5.2, -4.7, -4.2, -3.4, -2.9, -2.4]
                        .into_iter()
                        .map(move |x| block([x, 0.36, z], [0.12, 0.31, 0.15]))
                })
                .collect(),
        ));
    }
    scene.push(asset(
        "rc:asset/hall-facade",
        [0.99, 0.88, 0.66, 1.0],
        vec![
            block([-0.74, 0.67, -4.62], [0.25, 1.12, 0.06]),
            block([0.74, 0.67, -4.62], [0.25, 1.12, 0.06]),
            block([0.0, 1.21, -4.62], [1.8, 0.12, 0.07]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hall-windows",
        [0.25, 0.58, 0.67, 1.0],
        vec![
            block([-0.77, 0.7, -4.59], [0.33, 0.51, 0.05]),
            block([0.77, 0.7, -4.59], [0.33, 0.51, 0.05]),
        ],
    ));
    scene.push(asset(
        "rc:asset/van-trim",
        [0.94, 0.72, 0.27, 1.0],
        vec![
            block([0.0, 0.53, 4.09], [1.85, 0.1, 0.05]),
            block([0.9, 0.48, 4.55], [0.06, 0.22, 0.88]),
        ],
    ));
    scene.push(asset(
        "rc:asset/tank-pipes",
        [0.16, 0.39, 0.49, 1.0],
        vec![
            block([-5.0, 0.28, 0.2], [0.13, 0.12, 1.4]),
            block([-4.32, 0.28, 0.9], [1.45, 0.12, 0.13]),
        ],
    ));
    scene.push(parametric(
        "rc:asset/tank-rim",
        [0.83, 0.88, 0.83, 1.0],
        ParametricRecipe::Torus {
            center: [-5.0, 1.56, -0.5],
            major: 0.53,
            minor: 0.055,
            segments: 20,
            tube_segments: 6,
        },
    ));
    scene.push(parametric(
        "rc:asset/tank-indicator",
        if water_online {
            [0.22, 0.94, 0.64, 1.0]
        } else {
            [0.95, 0.36, 0.20, 1.0]
        },
        ParametricRecipe::Sphere {
            center: [-5.0, 1.02, 0.12],
            radius: 0.13,
            latitude: 6,
            longitude: 10,
        },
    ));
    for (id, center) in [
        ("rc:asset/tree-trunk-nw", [-6.15, 0.85, -5.5]),
        ("rc:asset/tree-trunk-ne", [6.1, 0.85, -5.2]),
        ("rc:asset/tree-trunk-sw", [-6.1, 0.85, 5.2]),
        ("rc:asset/tree-trunk-se", [6.1, 0.85, 4.8]),
    ] {
        scene.push(parametric(
            id,
            [0.56, 0.32, 0.19, 1.0],
            ParametricRecipe::Cylinder {
                center,
                radius: 0.17,
                height: 1.7,
                segments: 8,
            },
        ));
    }
    for (id, center) in [
        ("rc:asset/tree-crown-nw", [-6.15, 2.02, -5.5]),
        ("rc:asset/tree-crown-ne", [6.1, 2.02, -5.2]),
        ("rc:asset/tree-crown-sw", [-6.1, 2.02, 5.2]),
        ("rc:asset/tree-crown-se", [6.1, 2.02, 4.8]),
    ] {
        scene.push(parametric(
            id,
            [0.25, 0.65, 0.34, 1.0],
            ParametricRecipe::Sphere {
                center,
                radius: 0.79,
                latitude: 7,
                longitude: 10,
            },
        ));
    }
    for (id, center) in [
        ("rc:asset/salvage-drum-a", [5.35, 0.38, 3.65]),
        ("rc:asset/salvage-drum-b", [5.76, 0.38, 3.64]),
        ("rc:asset/salvage-drum-c", [5.55, 0.38, 4.1]),
    ] {
        scene.push(parametric(
            id,
            [0.67, 0.29, 0.19, 1.0],
            ParametricRecipe::Cylinder {
                center,
                radius: 0.19,
                height: 0.74,
                segments: 12,
            },
        ));
    }
    for (id, center) in [
        ("rc:asset/street-light-west", [-1.5, 1.25, 0.9]),
        ("rc:asset/street-light-east", [4.9, 1.25, 0.9]),
    ] {
        scene.push(parametric(
            id,
            [0.31, 0.37, 0.38, 1.0],
            ParametricRecipe::Cylinder {
                center,
                radius: 0.055,
                height: 2.5,
                segments: 8,
            },
        ));
    }
    for (id, center) in [
        ("rc:asset/street-globe-west", [-1.5, 2.5, 0.9]),
        ("rc:asset/street-globe-east", [4.9, 2.5, 0.9]),
    ] {
        scene.push(parametric(
            id,
            [0.97, 0.85, 0.54, 1.0],
            ParametricRecipe::Sphere {
                center,
                radius: 0.19,
                latitude: 6,
                longitude: 10,
            },
        ));
    }
    for (id, center) in [
        ("rc:asset/van-wheel-fl", [-0.58, 0.2, 4.1]),
        ("rc:asset/van-wheel-fr", [0.58, 0.2, 4.1]),
        ("rc:asset/van-wheel-rl", [-0.58, 0.2, 5.0]),
        ("rc:asset/van-wheel-rr", [0.58, 0.2, 5.0]),
    ] {
        scene.push(parametric(
            id,
            [0.08, 0.10, 0.10, 1.0],
            ParametricRecipe::Sphere {
                center,
                radius: 0.23,
                latitude: 5,
                longitude: 8,
            },
        ));
    }
    // Soft, asymmetric canopy lobes give each tree a readable silhouette.
    // The colours and forms are original game-authored data; Qualia generates
    // and seals every surface.
    for (id, center, radius, color) in [
        (
            "rc:asset/canopy-nw-a",
            [-6.48, 2.16, -5.40],
            0.58,
            [0.34, 0.75, 0.36, 1.0],
        ),
        (
            "rc:asset/canopy-nw-b",
            [-5.83, 2.31, -5.62],
            0.56,
            [0.53, 0.81, 0.38, 1.0],
        ),
        (
            "rc:asset/canopy-ne-a",
            [5.75, 2.22, -5.13],
            0.59,
            [0.36, 0.76, 0.48, 1.0],
        ),
        (
            "rc:asset/canopy-ne-b",
            [6.45, 2.30, -5.28],
            0.55,
            [0.61, 0.84, 0.42, 1.0],
        ),
        (
            "rc:asset/canopy-sw-a",
            [-6.48, 2.20, 5.28],
            0.56,
            [0.36, 0.74, 0.44, 1.0],
        ),
        (
            "rc:asset/canopy-sw-b",
            [-5.85, 2.29, 5.05],
            0.59,
            [0.60, 0.82, 0.39, 1.0],
        ),
        (
            "rc:asset/canopy-se-a",
            [5.76, 2.17, 4.74],
            0.58,
            [0.37, 0.73, 0.38, 1.0],
        ),
        (
            "rc:asset/canopy-se-b",
            [6.44, 2.26, 4.88],
            0.56,
            [0.58, 0.81, 0.37, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Sphere {
                center,
                radius,
                latitude: 16,
                longitude: 24,
            },
        ));
    }
    // People are the fictional participants, not sphere residents.
    // Bright flowering planters mark the shared hall and garden route.
    for (id, center, color) in [
        (
            "rc:asset/flower-a",
            [-5.25, 0.40, -4.15],
            [0.99, 0.45, 0.55, 1.0],
        ),
        (
            "rc:asset/flower-b",
            [-4.58, 0.43, -4.16],
            [0.99, 0.76, 0.31, 1.0],
        ),
        (
            "rc:asset/flower-c",
            [-3.32, 0.41, -3.28],
            [0.91, 0.48, 0.78, 1.0],
        ),
        (
            "rc:asset/flower-d",
            [-2.68, 0.43, -3.30],
            [0.99, 0.68, 0.31, 1.0],
        ),
        (
            "rc:asset/flower-e",
            [-1.20, 0.30, -4.49],
            [0.98, 0.44, 0.46, 1.0],
        ),
        (
            "rc:asset/flower-f",
            [1.23, 0.32, -4.50],
            [0.97, 0.75, 0.31, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Sphere {
                center,
                radius: 0.14,
                latitude: 6,
                longitude: 9,
            },
        ));
    }
    // Framing forms are world assets, never a CSS substitute for Portal
    // lighting or sky. Their parallax makes the miniature feel like a place.
    for (id, center, radius, color) in [
        (
            "rc:asset/hill-west",
            [-7.0, -0.40, -5.5],
            1.65,
            [0.54, 0.75, 0.43, 1.0],
        ),
        (
            "rc:asset/hill-east",
            [7.0, -0.42, -5.7],
            1.70,
            [0.45, 0.69, 0.47, 1.0],
        ),
        (
            "rc:asset/cloud-a",
            [-4.0, 3.84, -7.0],
            0.62,
            [0.99, 0.91, 0.76, 1.0],
        ),
        (
            "rc:asset/cloud-b",
            [-3.38, 4.03, -7.0],
            0.75,
            [1.0, 0.95, 0.82, 1.0],
        ),
        (
            "rc:asset/cloud-c",
            [-2.64, 3.83, -7.0],
            0.61,
            [0.99, 0.91, 0.76, 1.0],
        ),
        (
            "rc:asset/cloud-d",
            [3.43, 3.99, -7.2],
            0.65,
            [0.99, 0.94, 0.83, 1.0],
        ),
        (
            "rc:asset/cloud-e",
            [4.06, 3.84, -7.2],
            0.53,
            [0.99, 0.91, 0.76, 1.0],
        ),
        (
            "rc:asset/sun-disk",
            [0.80, 4.65, -7.3],
            0.57,
            [1.0, 0.78, 0.39, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Sphere {
                center,
                radius,
                latitude: 9,
                longitude: 14,
            },
        ));
    }
    scene
}


fn ink_colour(albedo: f32, absorb: f32, ior: f32) -> [f32; 4] {
    let shade = (albedo * (1.0 - absorb)).clamp(0.02, albedo);
    let cool = ((ior - 1.50) * albedo).clamp(0.0, 0.08);
    [
        shade,
        (shade + cool).min(albedo.max(shade)),
        (shade + cool * 0.5).min(albedo.max(shade)),
        1.0,
    ]
}

/// Qualia-shaped part record. Lives in the game repo. Not a baked clip.
fn part_record(
    part: &str,
    signature: &str,
    name: &str,
    albedo: f32,
    absorb: f32,
    ior: f32,
    sound: f32,
) -> String {
    format!(
        "part {part}\nsignature {signature}\nsignature_name \"{name}\"\nfacet optical\nreading colour\nspectrum_axis emf\nalbedo {albedo}\nabsorption {absorb}\nior {ior}\nsound_absorption {sound}\nmotion t=0 pos=0,0,0; t=1 pos=0.4,0,0\nbaked_clip false\nbaked_frame false\nconstruct editable\nwrites_spatial false\n"
    )
}

fn signed(
    recipe: AssetRecipe,
    signature: &'static str,
    name: &str,
    albedo: f32,
    absorb: f32,
    ior: f32,
    sound: f32,
) -> AssetRecipe {
    let mut recipe = recipe;
    recipe.color = ink_colour(albedo, absorb, ior);
    recipe.shell_signature = Some(signature);
    recipe.spectrum_reading = Some("colour");
    recipe.vibe_source = Some(part_record(
        recipe.id,
        signature,
        name,
        albedo,
        absorb,
        ior,
        sound,
    ));
    recipe
}


fn lathe(
    profile: &'static [[f32; 2]],
    segments: usize,
    scale: [f32; 3],
    at: [f32; 3],
) -> Result<Mesh, String> {
    let spun = ParametricRecipe::Revolve {
        center: [0.0, 0.0, 0.0],
        profile,
        segments,
    }
    .compile()?;
    Ok(authoring::transform_mesh(
        &spun,
        &authoring::mat_mul(
            &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
            &authoring::scale(f64::from(scale[0]), f64::from(scale[1]), f64::from(scale[2])),
        ),
    ))
}

fn wheel(radius: f32, tube: f32, at: [f32; 3]) -> Result<Mesh, String> {
    let ring = authoring::torus(radius, tube, 20, 8).map_err(|e| e.to_string())?;
    Ok(authoring::transform_mesh(
        &ring,
        &authoring::mat_mul(
            &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
            &authoring::rotation_x(std::f64::consts::FRAC_PI_2),
        ),
    ))
}

fn ball(radius: f32, at: [f32; 3]) -> Result<Mesh, String> {
    let sphere = authoring::uv_sphere(radius, 10, 14).map_err(|e| e.to_string())?;
    Ok(authoring::transform_mesh(
        &sphere,
        &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
    ))
}

fn tube(radius: f32, height: f32, at: [f32; 3], rot_z: f64) -> Result<Mesh, String> {
    let cyl = authoring::cylinder(radius, height, 16).map_err(|e| e.to_string())?;
    Ok(authoring::transform_mesh(
        &cyl,
        &authoring::mat_mul(
            &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
            &authoring::rotation_z(rot_z),
        ),
    ))
}

fn merge_at(parts: &[Mesh], center: [f32; 3]) -> Mesh {
    let mut mesh = empty_mesh();
    for part in parts {
        append_mesh(&mut mesh, part);
    }
    let placed = authoring::transform_mesh(
        &mesh,
        &authoring::translation(f64::from(center[0]), f64::from(center[1]), f64::from(center[2])),
    );
    let (min, max) = mesh_bounds(&placed.positions);
    Mesh {
        positions: placed.positions,
        triangles: placed.triangles,
        min,
        max,
    }
}

/// One coloured piece. The tint is the surface reading for every vertex
/// of that piece, before a small position wobble so it is not one albedo.
struct TintMesh {
    mesh: Mesh,
    tint: [f32; 4],
}

fn slab(w: f32, h: f32, d: f32, at: [f32; 3]) -> Result<Mesh, String> {
    let body = authoring::box_mesh(w, h, d).map_err(|e| e.to_string())?;
    Ok(authoring::transform_mesh(
        &body,
        &authoring::translation(f64::from(at[0]), f64::from(at[1]), f64::from(at[2])),
    ))
}

fn tint(mesh: Mesh, tint: [f32; 4]) -> TintMesh {
    TintMesh { mesh, tint }
}

const GLASS: [f32; 4] = [0.45, 0.68, 0.78, 1.0];
const RUBBER: [f32; 4] = [0.07, 0.07, 0.08, 1.0];
const LAMP: [f32; 4] = [0.98, 0.90, 0.45, 1.0];

fn wheels(radius: f32, tube: f32, spots: &[[f32; 3]]) -> Result<Vec<TintMesh>, String> {
    let mut out = Vec::with_capacity(spots.len());
    for at in spots {
        out.push(tint(wheel(radius, tube, *at)?, RUBBER));
    }
    Ok(out)
}

/// Bodies are slabs with a glass band and lamps, not a spun lump.
/// Wheels stay toruses. Kinds have to read apart with no label.
const BELL_FLY: &[[f32; 2]] = &[
    [0.0, 0.72],
    [0.18, 0.55],
    [0.55, 0.22],
    [0.92, 0.02],
    [1.05, -0.08],
];

fn rig_pieces(kind: u32, span: f32) -> Result<Vec<TintMesh>, String> {
    let span = span.max(0.6);
    let parts = match kind {
        0 => {
            let body = [0.72, 0.16, 0.14, 1.0];
            let mut v = vec![
                tint(slab(1.70, 0.32, 0.78, [0.05, 0.40, 0.0])?, body),
                tint(slab(0.78, 0.34, 0.70, [-0.12, 0.70, 0.0])?, body),
                tint(slab(0.70, 0.22, 0.02, [-0.12, 0.72, 0.36])?, GLASS),
                tint(slab(0.70, 0.22, 0.02, [-0.12, 0.72, -0.36])?, GLASS),
                tint(slab(0.02, 0.22, 0.62, [0.28, 0.72, 0.0])?, GLASS),
                tint(ball(0.06, [0.88, 0.42, 0.28])?, LAMP),
                tint(ball(0.06, [0.88, 0.42, -0.28])?, LAMP),
            ];
            v.extend(wheels(0.20, 0.055, &[
                [0.55, 0.20, 0.40],
                [0.55, 0.20, -0.40],
                [-0.55, 0.20, 0.40],
                [-0.55, 0.20, -0.40],
            ])?);
            v
        }
        1 => {
            let body = [0.28, 0.36, 0.18, 1.0];
            let mut v = vec![
                tint(slab(1.65, 0.42, 0.86, [0.0, 0.58, 0.0])?, body),
                tint(slab(0.72, 0.38, 0.78, [-0.10, 0.92, 0.0])?, body),
                tint(slab(0.02, 0.24, 0.66, [0.26, 0.94, 0.0])?, GLASS),
                tint(slab(0.64, 0.22, 0.02, [-0.10, 0.94, 0.40])?, GLASS),
                tint(wheel(0.22, 0.05, [-0.95, 0.72, 0.0])?, RUBBER),
                tint(ball(0.07, [0.84, 0.62, 0.32])?, LAMP),
                tint(ball(0.07, [0.84, 0.62, -0.32])?, LAMP),
            ];
            v.extend(wheels(0.30, 0.08, &[
                [0.52, 0.30, 0.48],
                [0.52, 0.30, -0.48],
                [-0.50, 0.30, 0.48],
                [-0.50, 0.30, -0.48],
            ])?);
            v
        }
        2 => {
            let body = [0.90, 0.88, 0.80, 1.0];
            let mut v = vec![
                tint(slab(2.20, 1.05, 0.95, [0.0, 0.78, 0.0])?, body),
                tint(slab(0.70, 0.28, 0.02, [0.35, 1.05, 0.48])?, GLASS),
                tint(slab(0.70, 0.28, 0.02, [0.35, 1.05, -0.48])?, GLASS),
                tint(slab(0.02, 0.32, 0.70, [1.10, 1.02, 0.0])?, GLASS),
                tint(ball(0.07, [1.12, 0.55, 0.32])?, LAMP),
                tint(ball(0.07, [1.12, 0.55, -0.32])?, LAMP),
            ];
            v.extend(wheels(0.24, 0.06, &[
                [0.72, 0.24, 0.50],
                [0.72, 0.24, -0.50],
                [-0.72, 0.24, 0.50],
                [-0.72, 0.24, -0.50],
            ])?);
            v
        }
        3 => {
            let body = [0.93, 0.74, 0.16, 1.0];
            let mut v = vec![
                tint(slab(3.50, 1.25, 1.05, [0.0, 0.90, 0.0])?, body),
                tint(slab(2.40, 0.28, 0.02, [0.15, 1.20, 0.53])?, GLASS),
                tint(slab(2.40, 0.28, 0.02, [0.15, 1.20, -0.53])?, GLASS),
                tint(slab(0.02, 0.40, 0.80, [1.74, 1.05, 0.0])?, GLASS),
            ];
            v.extend(wheels(0.28, 0.07, &[
                [1.15, 0.28, 0.56],
                [1.15, 0.28, -0.56],
                [-0.15, 0.28, 0.56],
                [-0.15, 0.28, -0.56],
                [-1.20, 0.28, 0.56],
                [-1.20, 0.28, -0.56],
            ])?);
            v
        }
        4 => {
            let body = [0.20, 0.48, 0.55, 1.0];
            let pod = [0.85, 0.82, 0.70, 1.0];
            let mut v = vec![
                tint(slab(2.25, 1.00, 0.98, [0.0, 0.74, 0.0])?, body),
                tint(slab(1.20, 0.22, 0.78, [-0.10, 1.32, 0.0])?, pod),
                tint(slab(0.80, 0.22, 0.02, [0.20, 1.00, 0.50])?, GLASS),
                tint(slab(0.02, 0.28, 0.70, [1.12, 0.95, 0.0])?, GLASS),
            ];
            v.extend(wheels(0.24, 0.06, &[
                [0.70, 0.24, 0.52],
                [0.70, 0.24, -0.52],
                [-0.75, 0.24, 0.52],
                [-0.75, 0.24, -0.52],
            ])?);
            v
        }
        5 => {
            let body = [0.55, 0.34, 0.18, 1.0];
            let mut v = vec![
                tint(slab(1.80, 0.42, 0.85, [0.15, 0.48, 0.0])?, body),
                tint(tube(0.04, 0.70, [-0.95, 0.32, 0.0], std::f64::consts::FRAC_PI_2)?, [0.25, 0.25, 0.27, 1.0]),
            ];
            v.extend(wheels(0.20, 0.05, &[
                [0.35, 0.20, 0.46],
                [0.35, 0.20, -0.46],
            ])?);
            v
        }
        6 => {
            let body = [0.86, 0.82, 0.68, 1.0];
            let door = [0.45, 0.32, 0.20, 1.0];
            let mut v = vec![
                tint(slab(2.40, 1.05, 1.00, [0.10, 0.72, 0.0])?, body),
                tint(slab(0.90, 0.16, 0.90, [0.10, 1.28, 0.0])?, [0.72, 0.40, 0.22, 1.0]),
                tint(slab(0.36, 0.70, 0.02, [0.20, 0.62, 0.51])?, door),
                tint(tube(0.035, 0.45, [-1.20, 0.40, 0.0], std::f64::consts::FRAC_PI_2)?, [0.3, 0.3, 0.32, 1.0]),
            ];
            v.extend(wheels(0.22, 0.055, &[
                [0.45, 0.22, 0.54],
                [0.45, 0.22, -0.54],
                [-0.35, 0.22, 0.54],
                [-0.35, 0.22, -0.54],
            ])?);
            v
        }
        7 => vec![
            tint(tube(0.03, 0.55, [0.0, 0.28, 0.0], 0.0)?, [0.35, 0.35, 0.38, 1.0]),
            tint(
                authoring::transform_mesh(
                    &authoring::cylinder(0.42, 0.03, 20).map_err(|e| e.to_string())?,
                    &authoring::mat_mul(
                        &authoring::translation(0.0, 0.62, 0.0),
                        &authoring::mat_mul(
                            &authoring::rotation_x(0.7),
                            &authoring::scale(1.15, 1.0, 0.7),
                        ),
                    ),
                ),
                [0.08, 0.16, 0.42, 1.0],
            ),
        ],
        8 => vec![
            tint(tube(0.16, 0.28, [0.0, 0.16, 0.0], 0.0)?, [0.18, 0.19, 0.22, 1.0]),
            tint(tube(0.03, 0.08, [-0.06, 0.34, 0.0], 0.0)?, [0.75, 0.18, 0.14, 1.0]),
            tint(tube(0.03, 0.08, [0.06, 0.34, 0.0], 0.0)?, [0.20, 0.55, 0.28, 1.0]),
        ],
        9 => vec![
            tint(slab(0.46, 0.16, 0.28, [0.0, 0.12, 0.0])?, [0.55, 0.56, 0.58, 1.0]),
            tint(tube(0.015, 0.18, [0.0, 0.16, 0.0], std::f64::consts::FRAC_PI_2)?, [0.3, 0.3, 0.32, 1.0]),
            tint(ball(0.025, [0.16, 0.18, 0.10])?, LAMP),
        ],
        _ => {
            let canvas = [0.72, 0.55, 0.32, 1.0];
            let post = [0.35, 0.24, 0.14, 1.0];
            vec![
                tint(lathe(BELL_FLY, 32, [span, span, span], [0.0, 0.0, 0.0])?, canvas),
                tint(tube(0.04, 0.70, [0.55 * span, 0.35, 0.72 * span], 0.0)?, post),
                tint(tube(0.04, 0.70, [-0.55 * span, 0.35, 0.72 * span], 0.0)?, post),
            ]
        }
    };
    Ok(parts)
}

/// Models that have to read apart with no label. None borrows the tank.
fn compile_rig(center: [f32; 3], kind: u32, span: f32) -> Result<Mesh, String> {
    let parts = rig_pieces(kind, span)?;
    let meshes: Vec<Mesh> = parts.into_iter().map(|p| p.mesh).collect();
    Ok(merge_at(&meshes, center))
}

/// First authored batch. Each part has its own silhouette, signature, colour
/// reading, sound reading, and a two-pose motion. Shower blocks, roads, and
/// living things stay out until a source names them.
fn authored_camp() -> Vec<AssetRecipe> {
    const TENT_PROFILE: &[[f32; 2]] = &[[0.0, -0.35], [0.62, -0.35], [0.04, 0.62]];
    let z = 6.15_f32;
    let mut out = Vec::with_capacity(16);
    out.push(signed(
        parametric(
            "rc:asset/tent",
            [1.0, 1.0, 1.0, 1.0],
            ParametricRecipe::Revolve {
                center: [-6.2, 0.55, z],
                profile: TENT_PROFILE,
                segments: 28,
            },
        ),
        "did:webizen:game:canvas-tent-v1",
        "Canvas tent fly",
        0.55,
        0.15,
        1.45,
        0.40,
    ));
    out.push(signed(
        parametric("rc:asset/car", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-4.3, 0.0, z], kind: 0, span: 1.0 }),
        "did:webizen:game:car-body-v1", "Car body", 0.22, 0.35, 1.52, 0.08,
    ));
    out.push(signed(
        parametric("rc:asset/four-wd", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-2.3, 0.0, z], kind: 1, span: 1.0 }),
        "did:webizen:game:four-wd-body-v1", "Four-wheel-drive body", 0.28, 0.30, 1.52, 0.09,
    ));
    out.push(signed(
        parametric("rc:asset/passenger-van", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-0.15, 0.0, z], kind: 2, span: 1.0 }),
        "did:webizen:game:van-body-v1", "Van body", 0.33, 0.28, 1.50, 0.10,
    ));
    out.push(signed(
        parametric("rc:asset/bus", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [2.7, 0.0, z], kind: 3, span: 1.0 }),
        "did:webizen:game:bus-body-v1", "Bus body", 0.50, 0.18, 1.50, 0.11,
    ));
    out.push(signed(
        parametric("rc:asset/camper-van", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-5.5, 0.0, z - 1.7], kind: 4, span: 1.0 }),
        "did:webizen:game:camper-van-shell-v1", "Camper van shell", 0.42, 0.25, 1.50, 0.12,
    ));
    out.push(signed(
        parametric("rc:asset/camper-trailer", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-2.6, 0.0, z - 1.7], kind: 5, span: 1.0 }),
        "did:webizen:game:camper-trailer-shell-v1", "Camper trailer shell", 0.38, 0.22, 1.48, 0.14,
    ));
    out.push(signed(
        parametric("rc:asset/caravan", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [0.4, 0.0, z - 1.7], kind: 6, span: 1.0 }),
        "did:webizen:game:caravan-shell-v1", "Caravan shell", 0.48, 0.20, 1.50, 0.13,
    ));
    out.push(signed(
        parametric(
            "rc:asset/stove",
            [1.0, 1.0, 1.0, 1.0],
            ParametricRecipe::Cylinder {
                center: [3.3, 0.22, z - 1.7],
                radius: 0.16,
                height: 0.36,
                segments: 24,
            },
        ),
        "did:webizen:game:stove-body-v1",
        "Contained stove body",
        0.12,
        0.55,
        1.60,
        0.05,
    ));
    out.push(signed(
        parametric(
            "rc:asset/fire-pit",
            [1.0, 1.0, 1.0, 1.0],
            ParametricRecipe::Torus {
                center: [4.3, 0.12, z - 1.7],
                major: 0.38,
                minor: 0.08,
                segments: 24,
                tube_segments: 12,
            },
        ),
        "did:webizen:game:fire-pit-rim-v1",
        "Open fire-pit rim",
        0.20,
        0.45,
        1.55,
        0.20,
    ));
    // Parts on the rig, not the rig. No wheels.
    out.push(signed(
        parametric("rc:asset/solar-panel", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-6.3, 0.0, 3.15], kind: 7, span: 1.0 }),
        "did:webizen:game:solar-cell-v1", "Solar panel cell", 0.08, 0.70, 1.90, 0.02,
    ));
    out.push(signed(
        parametric("rc:asset/battery", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-5.35, 0.0, 3.15], kind: 8, span: 1.0 }),
        "did:webizen:game:battery-pack-v1", "Battery pack housing", 0.15, 0.40, 1.45, 0.06,
    ));
    out.push(signed(
        parametric("rc:asset/inverter", [1.0, 1.0, 1.0, 1.0], ParametricRecipe::Rig { center: [-4.55, 0.0, 3.15], kind: 9, span: 1.0 }),
        "did:webizen:game:inverter-housing-v1", "Inverter housing", 0.18, 0.35, 1.50, 0.07,
    ));
    // Fictional participant. No face, no data likeness.
    out.push(signed(
        parametric(
            "rc:asset/participant",
            [1.0, 1.0, 1.0, 1.0],
            ParametricRecipe::Figure {
                center: [5.5, 0.0, z - 1.7],
                kind: 2,
                phase: 0.6,
                marked: false,
                yaw: 0.4,
                light: 0,
                carry: 0,
            },
        ),
        "did:webizen:game:participant-cloth-v1",
        "Fictional participant cloth",
        0.40,
        0.25,
        1.40,
        0.50,
    ));
    out
}

/// Saltwind Reach is a second authored territory sharing the same scene
/// coordinates, event world and QualiaPortal viewport as Kestrel Flats.
/// The water crossing is a visual state of the rule-gated bridge project.
pub fn saltwind_reach(
    bridge_open: bool,
    bridge_braced: bool,
    high_tide: bool,
    pump_online: bool,
    orchard_active: bool,
) -> Vec<AssetRecipe> {
    let mut scene = Vec::with_capacity(40);
    scene.push(asset(
        "rc:asset/saltwind-ground",
        [0.72, 0.78, 0.47, 1.0],
        vec![block([16.0, -0.08, 0.0], [14.0, 0.16, 14.0])],
    ));
    scene.push(asset(
        "rc:asset/channel",
        if high_tide {
            [0.18, 0.48, 0.66, 1.0]
        } else {
            [0.23, 0.67, 0.75, 1.0]
        },
        vec![block(
            [8.0, if high_tide { 0.04 } else { -0.06 }, 0.0],
            [2.0, if high_tide { 0.20 } else { 0.08 }, 14.0],
        )],
    ));
    scene.push(asset(
        "rc:asset/bridge",
        if bridge_open {
            [0.87, 0.59, 0.38, 1.0]
        } else {
            [0.52, 0.39, 0.34, 1.0]
        },
        if bridge_open {
            vec![block([8.0, 0.11, 0.5], [2.6, 0.20, 1.5])]
        } else {
            vec![
                block([7.18, 0.11, 0.5], [0.70, 0.20, 1.5]),
                block([8.82, 0.11, 0.5], [0.70, 0.20, 1.5]),
            ]
        },
    ));
    scene.push(asset(
        "rc:asset/bridge-rails",
        [0.92, 0.75, 0.48, 1.0],
        // Rails follow the deck. A broken span does not keep a rail across the gap.
        if bridge_open {
            vec![
                block([8.0, 0.42, -0.28], [2.6, 0.08, 0.09]),
                block([8.0, 0.42, 1.27], [2.6, 0.08, 0.09]),
            ]
        } else {
            vec![
                block([7.18, 0.42, -0.28], [0.70, 0.08, 0.09]),
                block([7.18, 0.42, 1.27], [0.70, 0.08, 0.09]),
                block([8.82, 0.42, -0.28], [0.70, 0.08, 0.09]),
                block([8.82, 0.42, 1.27], [0.70, 0.08, 0.09]),
            ]
        },
    ));
    if bridge_braced && !bridge_open {
        scene.push(asset(
            "rc:asset/bridge-bracing",
            [0.95, 0.67, 0.37, 1.0],
            vec![
                block([7.6, 0.22, 0.48], [0.17, 0.44, 1.68]),
                block([8.4, 0.22, 0.48], [0.17, 0.44, 1.68]),
            ],
        ));
    }
    scene.push(asset(
        "rc:asset/saltwind-main-road",
        [0.89, 0.72, 0.50, 1.0],
        vec![
            block([16.0, 0.01, 0.5], [13.0, 0.05, 1.35]),
            block([16.0, 0.01, -1.65], [1.0, 0.05, 5.5]),
        ],
    ));
    scene.push(asset(
        "rc:asset/saltwind-footpaths",
        [0.97, 0.85, 0.65, 1.0],
        vec![
            block([12.3, 0.04, -2.0], [0.7, 0.05, 3.1]),
            block([19.0, 0.04, -2.1], [0.7, 0.05, 3.0]),
            block([17.5, 0.04, 3.4], [3.8, 0.05, 0.65]),
        ],
    ));
    scene.push(asset(
        "rc:asset/saltwind-barn",
        [0.92, 0.49, 0.38, 1.0],
        vec![
            block([15.3, 0.79, 3.3], [2.4, 1.58, 1.85]),
            roof([15.3, 1.59, 3.3], [2.8, 0.58, 2.15]),
        ],
    ));
    scene.push(asset(
        "rc:asset/barn-door",
        [0.37, 0.48, 0.50, 1.0],
        vec![block([15.3, 0.61, 2.35], [0.83, 1.2, 0.06])],
    ));
    scene.push(asset(
        "rc:asset/barn-trim",
        [0.99, 0.78, 0.49, 1.0],
        vec![
            block([15.3, 1.47, 2.31], [1.7, 0.15, 0.07]),
            block([14.13, 0.82, 2.31], [0.13, 1.5, 0.07]),
            block([16.47, 0.82, 2.31], [0.13, 1.5, 0.07]),
        ],
    ));
    scene.push(parametric(
        "rc:asset/wind-pump-tower",
        [0.96, 0.82, 0.61, 1.0],
        ParametricRecipe::Cylinder {
            center: [19.0, 1.42, -2.0],
            radius: 0.35,
            height: 2.84,
            segments: 16,
        },
    ));
    scene.push(parametric(
        "rc:asset/wind-pump-cap",
        if pump_online {
            [0.36, 0.77, 0.68, 1.0]
        } else {
            [0.66, 0.66, 0.64, 1.0]
        },
        ParametricRecipe::Sphere {
            center: [19.0, 2.82, -2.0],
            radius: 0.49,
            latitude: 9,
            longitude: 14,
        },
    ));
    scene.push(asset(
        "rc:asset/wind-pump-blades",
        if pump_online {
            [0.98, 0.69, 0.34, 1.0]
        } else {
            [0.72, 0.61, 0.51, 1.0]
        },
        vec![
            block([19.0, 2.82, -1.46], [0.16, 2.1, 0.09]),
            block([19.0, 2.82, -1.46], [2.1, 0.16, 0.09]),
        ],
    ));
    scene.push(asset(
        "rc:asset/pump-trough",
        [0.32, 0.66, 0.73, 1.0],
        vec![block([19.0, 0.16, -0.8], [1.5, 0.3, 0.62])],
    ));
    scene.push(asset(
        "rc:asset/saltwind-market",
        [0.99, 0.73, 0.43, 1.0],
        vec![
            block([11.8, 0.54, -2.6], [1.7, 1.08, 1.15]),
            roof([11.8, 1.16, -2.6], [2.1, 0.45, 1.45]),
        ],
    ));
    scene.push(asset(
        "rc:asset/saltwind-market-awning",
        [0.43, 0.71, 0.76, 1.0],
        vec![block([11.8, 1.28, -1.8], [2.15, 0.09, 0.8])],
    ));
    scene.push(asset(
        "rc:asset/saltwind-quay",
        [0.58, 0.39, 0.26, 1.0],
        vec![
            block([9.45, 0.10, 3.7], [1.2, 0.20, 2.8]),
            block([9.45, 0.13, -3.8], [1.2, 0.20, 2.0]),
        ],
    ));
    scene.push(asset(
        "rc:asset/canal-boat",
        [0.94, 0.38, 0.32, 1.0],
        vec![
            block([8.0, 0.13, 3.8], [0.86, 0.25, 1.85]),
            block([8.0, 0.35, 3.8], [0.09, 0.5, 0.09]),
        ],
    ));
    scene.push(asset(
        "rc:asset/orchard-beds",
        if orchard_active {
            [0.44, 0.69, 0.32, 1.0]
        } else {
            [0.57, 0.48, 0.31, 1.0]
        },
        vec![
            block([18.3, 0.10, 3.6], [3.5, 0.2, 1.2]),
            block([18.3, 0.10, 5.3], [3.5, 0.2, 1.2]),
        ],
    ));
    for (trunk_id, crown_id, x, z, hue) in [
        (
            "rc:asset/orchard-tree-a-trunk",
            "rc:asset/orchard-tree-a-crown",
            11.4,
            4.8,
            [0.50, 0.74, 0.38, 1.0],
        ),
        (
            "rc:asset/orchard-tree-b-trunk",
            "rc:asset/orchard-tree-b-crown",
            20.5,
            4.9,
            [0.38, 0.69, 0.40, 1.0],
        ),
        (
            "rc:asset/orchard-tree-c-trunk",
            "rc:asset/orchard-tree-c-crown",
            21.0,
            -5.0,
            [0.56, 0.79, 0.40, 1.0],
        ),
    ] {
        scene.push(parametric(
            trunk_id,
            [0.58, 0.36, 0.23, 1.0],
            ParametricRecipe::Cylinder {
                center: [x, 0.88, z],
                radius: 0.17,
                height: 1.76,
                segments: 9,
            },
        ));
        scene.push(parametric(
            crown_id,
            hue,
            ParametricRecipe::Sphere {
                center: [x, 2.08, z],
                radius: 0.85,
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    if orchard_active {
        scene.push(asset(
            "rc:asset/orchard-fruit",
            [0.98, 0.38, 0.26, 1.0],
            vec![
                block([17.2, 0.30, 3.6], [0.16, 0.17, 0.16]),
                block([18.1, 0.32, 3.6], [0.16, 0.17, 0.16]),
                block([19.0, 0.31, 3.6], [0.16, 0.17, 0.16]),
                block([17.5, 0.32, 5.3], [0.16, 0.17, 0.16]),
                block([18.6, 0.30, 5.3], [0.16, 0.17, 0.16]),
            ],
        ));
    }
    scene
}

fn wobble(tint: [f32; 4], p: [f32; 3], amount: f32) -> [f32; 4] {
    let n = (p[0] * 12.7 + p[1] * 7.3 + p[2] * 5.1).sin() * amount;
    let m = (p[0] * 3.1 - p[2] * 4.4).cos() * amount * 0.5;
    [
        (tint[0] + n).clamp(0.0, 1.0),
        (tint[1] + m).clamp(0.0, 1.0),
        (tint[2] + n * 0.4).clamp(0.0, 1.0),
        tint[3],
    ]
}

fn paint_figure(mesh: &Mesh, kind: u32, cloth: [f32; 4]) -> Vec<[f32; 4]> {
    let skin = match kind % 6 {
        0 => [0.72, 0.50, 0.36, 1.0],
        1 => [0.40, 0.28, 0.20, 1.0],
        2 => [0.86, 0.70, 0.55, 1.0],
        3 => [0.55, 0.36, 0.26, 1.0],
        4 => [0.78, 0.58, 0.42, 1.0],
        _ => [0.48, 0.34, 0.26, 1.0],
    };
    let hair = [skin[0] * 0.35, skin[1] * 0.28, skin[2] * 0.22, 1.0];
    let pants = [
        (cloth[0] * 0.55).clamp(0.0, 1.0),
        (cloth[1] * 0.5).clamp(0.0, 1.0),
        (cloth[2] * 0.5).clamp(0.0, 1.0),
        1.0,
    ];
    let span = (mesh.max[1] - mesh.min[1]).max(0.001);
    mesh.positions
        .iter()
        .map(|p| {
            let t = (p[1] - mesh.min[1]) / span;
            let tint = if t > 0.93 {
                hair
            } else if t > 0.78 {
                skin
            } else if t > 0.42 {
                cloth
            } else {
                pants
            };
            wobble(tint, *p, 0.04)
        })
        .collect()
}

/// Per-vertex SRD1 reading. One organ albedo is not this.
pub fn surface_reading(recipe: &AssetRecipe, mesh: &Mesh) -> Vec<[f32; 4]> {
    if let Some(ParametricRecipe::Figure { kind, .. }) = &recipe.parametric {
        return paint_figure(mesh, *kind, recipe.color);
    }
    if let Some(ParametricRecipe::Rig { kind, span, .. }) = &recipe.parametric {
        if let Ok(parts) = rig_pieces(*kind, *span) {
            let mut colors = Vec::new();
            for part in parts {
                for v in &part.mesh.positions {
                    colors.push(wobble(part.tint, *v, 0.045));
                }
            }
            if colors.len() == mesh.positions.len() {
                return colors;
            }
        }
    }
    let leafy = recipe.id.contains("canopy")
        || recipe.id.contains("crown")
        || recipe.id.contains("tree");
    let ground = recipe.id.contains("ground") || recipe.id.contains("grass") || recipe.id.contains("verge");
    let amount = if leafy { 0.12 } else if ground { 0.08 } else { 0.05 };
    mesh.positions
        .iter()
        .map(|p| wobble(recipe.color, *p, amount))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_tank_colour_is_hdpe_reading_not_sugar() {
        let colour = hdpe_shell_colour();
        assert_eq!(colour[3], 1.0, "opaque so both zooms keep a silhouette");
        let sugar = vibe::physics::MaterialSignature::lookup("sugar_cube").unwrap();
        let sugar_albedo = sugar.optical.unwrap().albedo as f32;
        assert!(colour[0] < sugar_albedo);
        assert!(colour[1] < sugar_albedo);
        assert!(colour[2] < sugar_albedo);
        // Old water-online cyan swap.
        assert!(colour[1] < 0.5, "not the baked cyan {colour:?}");
        let tank = kestrel_flats(0, 0, false, false, false, false)
            .into_iter()
            .find(|a| a.id == "rc:asset/water-tank")
            .expect("tank");
        assert_eq!(tank.color, colour);
        assert_eq!(
            tank.shell_signature,
            Some("did:q42:material:hdpe-tank-shell-v1")
        );
        assert_eq!(tank.spectrum_reading, Some("colour"));
        let mesh = tank.parametric.unwrap().compile().unwrap();
        assert!(mesh.triangle_count() > 64, "rim must hold at close zoom");
        let span_y = mesh.max[1] - mesh.min[1];
        let span_x = mesh.max[0] - mesh.min[0];
        assert!(span_y > 1.0 && span_x > 0.8, "belly reads zoomed out {span_x} {span_y}");
        let rec = tank.vibe_source.unwrap();
        assert!(rec.contains("baked_clip false"));
        assert!(rec.contains("motion t=0"));
    }

    #[test]
    fn authored_camp_parts_are_distinct_records_and_empty_kinds_stay_out() {
        let scene = kestrel_flats(0, 0, false, false, false, false);
        let want = [
            "rc:asset/tent",
            "rc:asset/car",
            "rc:asset/four-wd",
            "rc:asset/passenger-van",
            "rc:asset/bus",
            "rc:asset/camper-van",
            "rc:asset/camper-trailer",
            "rc:asset/caravan",
            "rc:asset/stove",
            "rc:asset/fire-pit",
            "rc:asset/solar-panel",
            "rc:asset/battery",
            "rc:asset/inverter",
            "rc:asset/participant",
        ];
        for id in want {
            let part = scene.iter().find(|a| a.id == id).unwrap_or_else(|| panic!("missing {id}"));
            assert!(part.shell_signature.is_some(), "{id}");
            assert_eq!(part.spectrum_reading, Some("colour"));
            let rec = part.vibe_source.as_deref().unwrap_or("");
            assert!(rec.contains("baked_clip false"), "{id}");
            assert!(rec.contains("sound_absorption"), "{id}");
            assert!(rec.contains("construct editable"), "{id}");
            assert!(!rec.contains("sucrose"), "{id}");
        }
        let stove = scene.iter().find(|a| a.id == "rc:asset/stove").unwrap();
        let pit = scene.iter().find(|a| a.id == "rc:asset/fire-pit").unwrap();
        assert_ne!(stove.color, pit.color);
        assert!(scene.iter().all(|a| {
            !a.id.contains("shower") && !a.id.contains("flora") && !a.id.contains("fauna") && !a.id.contains("funga")
        }));
        for id in ["rc:asset/car", "rc:asset/passenger-van", "rc:asset/bus", "rc:asset/caravan", "rc:asset/solar-panel", "rc:asset/battery"] {
            let part = scene.iter().find(|a| a.id == id).unwrap();
            assert!(part.parts.is_empty(), "{id} still a box record");
            let mesh = part.parametric.as_ref().unwrap().compile().unwrap();
            assert!(mesh.triangle_count() > 80, "{id} tris {}", mesh.triangle_count());
        }
        let car = scene.iter().find(|a| a.id == "rc:asset/car").unwrap().parametric.as_ref().unwrap().compile().unwrap();
        let van = scene.iter().find(|a| a.id == "rc:asset/passenger-van").unwrap().parametric.as_ref().unwrap().compile().unwrap();
        let bus = scene.iter().find(|a| a.id == "rc:asset/bus").unwrap().parametric.as_ref().unwrap().compile().unwrap();
        let van_h = van.max[1] - van.min[1];
        let car_h = car.max[1] - car.min[1];
        let bus_l = bus.max[0] - bus.min[0];
        let van_l = van.max[0] - van.min[0];
        assert!(van_h > car_h + 0.25, "van taller than car {van_h} {car_h}");
        assert!(bus_l > van_l + 0.6, "bus longer than van {bus_l} {van_l}");
        let solar = scene.iter().find(|a| a.id == "rc:asset/solar-panel").unwrap().parametric.as_ref().unwrap().compile().unwrap();
        assert!(solar.max[1] - solar.min[1] < 1.2, "panel is not a vehicle");
    }

    #[test]
    fn figure_sealed_mesh_is_a_person_not_a_box() {
        let party = "0,0,0.8,0.4,0.2,0,0,0,0,0;1.5,0.2,0.3,0.45,0.7,1,0,1.1,0.4,1";
        let figs = participant_markers(party);
        assert_eq!(figs.len(), 2);
        let a = figs[0].parametric.as_ref().unwrap().compile().unwrap();
        let b = figs[1].parametric.as_ref().unwrap().compile().unwrap();
        assert!(a.triangle_count() > 200, "not a 12-tri box {}", a.triangle_count());
        assert_eq!(a.positions.len(), b.positions.len(), "pose keeps one topology");
        let tall = a.max[1] - a.min[1];
        let wide_a = a.max[0] - a.min[0];
        let wide_b = b.max[0] - b.min[0];
        assert!(tall > 1.4, "upright {tall}");
        assert!(tall > wide_a, "not a crate");
        assert!((wide_a - wide_b).abs() > 0.15, "silhouettes differ {wide_a} {wide_b}");
        let moved = a.positions.iter().zip(b.positions.iter()).filter(|(p, q)| (p[2] - q[2]).abs() > 0.08).count();
        assert!(moved > 20, "phase moves limbs, not a rigid box");
        assert!(figs[0].parts.is_empty(), "no box parts");
    }

    #[test]
    fn surface_reading_is_not_one_flat_albedo() {
        let scene = kestrel_flats(0, 0, false, false, false, false);
        let car = scene.iter().find(|a| a.id == "rc:asset/car").unwrap();
        let mesh = car.parametric.as_ref().unwrap().compile().unwrap();
        let reading = surface_reading(car, &mesh);
        assert_eq!(reading.len(), mesh.vertex_count());
        let q = |c: [f32; 4]| {
            (
                (c[0] * 16.0) as u8,
                (c[1] * 16.0) as u8,
                (c[2] * 16.0) as u8,
            )
        };
        let mut kinds = std::collections::BTreeSet::new();
        for sample in &reading {
            kinds.insert(q(*sample));
        }
        assert!(kinds.len() > 3, "car reading collapsed {}", kinds.len());
        let fig = participant_markers("0,0,0.8,0.3,0.2,0,0,0,0,0");
        let fm = fig[0].parametric.as_ref().unwrap().compile().unwrap();
        let fr = surface_reading(&fig[0], &fm);
        assert_eq!(fr.len(), fm.positions.len());
        assert!(fr.iter().any(|c| c[0] > 0.6), "skin or cloth must read");
        assert!(fr.iter().any(|c| (c[0] - fr[0][0]).abs() > 0.08), "figure is not one albedo");
    }
}
