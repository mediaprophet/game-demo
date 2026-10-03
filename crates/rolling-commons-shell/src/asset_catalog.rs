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
    }
}

fn parametric(id: &'static str, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(recipe),
        color,
        vibe_source: None,
    }
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
    scene.push(asset(
        "rc:asset/camp-shelter",
        [0.96, 0.53, 0.35, 1.0],
        vec![
            roof(
                [-3.0, 0.0, 2.6],
                [1.6 + upgrades.min(20) as f32 * 0.25, 0.9, 1.2],
            ),
            block([-3.0, 0.02, 3.6], [1.8, 0.06, 0.5]),
        ],
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
    scene.push(parametric(
        "rc:asset/water-tank",
        if water_online {
            [0.26, 0.70, 0.78, 1.0]
        } else {
            [0.48, 0.57, 0.57, 1.0]
        },
        ParametricRecipe::Revolve {
            center: [-5.0, 1.02, -0.5],
            profile: TANK_PROFILE,
            segments: 20,
        },
    ));
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
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    // Four tiny residents make the town feel inhabited. Each body part is a
    // stable semantic asset so future Qualia animation clips can replace it.
    for (body_id, head_id, feet_id, x, z, coat) in [
        (
            "rc:asset/resident-ada-coat",
            "rc:asset/resident-ada-head",
            "rc:asset/resident-ada-boots",
            -2.2,
            1.8,
            [0.95, 0.35, 0.38, 1.0],
        ),
        (
            "rc:asset/resident-bo-coat",
            "rc:asset/resident-bo-head",
            "rc:asset/resident-bo-boots",
            1.0,
            0.4,
            [0.35, 0.66, 0.85, 1.0],
        ),
        (
            "rc:asset/resident-cam-coat",
            "rc:asset/resident-cam-head",
            "rc:asset/resident-cam-boots",
            -2.0,
            -2.0,
            [0.91, 0.67, 0.29, 1.0],
        ),
        (
            "rc:asset/resident-dev-coat",
            "rc:asset/resident-dev-head",
            "rc:asset/resident-dev-boots",
            3.5,
            -0.6,
            [0.62, 0.46, 0.83, 1.0],
        ),
    ] {
        scene.push(parametric(
            body_id,
            coat,
            ParametricRecipe::Sphere {
                center: [x, 0.55, z],
                radius: 0.28,
                latitude: 8,
                longitude: 12,
            },
        ));
        scene.push(parametric(
            head_id,
            [0.94, 0.68, 0.47, 1.0],
            ParametricRecipe::Sphere {
                center: [x, 1.04, z],
                radius: 0.23,
                latitude: 8,
                longitude: 12,
            },
        ));
        scene.push(asset(
            feet_id,
            [0.31, 0.35, 0.47, 1.0],
            vec![
                block([x - 0.12, 0.16, z], [0.14, 0.32, 0.17]),
                block([x + 0.12, 0.16, z], [0.14, 0.32, 0.17]),
            ],
        ));
    }
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
        vec![
            block([8.0, 0.42, -0.28], [2.6, 0.08, 0.09]),
            block([8.0, 0.42, 1.27], [2.6, 0.08, 0.09]),
        ],
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
