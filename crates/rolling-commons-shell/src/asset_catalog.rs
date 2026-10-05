//! Original Kestrel Flats source recipes. QualiaDB owns mesh assembly and `.10d`.
//! These are authored game assets, separate from the engine and its licence.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::scene_primitives::Primitive;
use qualia_core_db::specialized_libs::computational_geometry::{authoring, parametric_cad, Point3};

/// Game-owned source parameters; Qualia's public computational geometry
/// builds the mesh before the shared `.10d` compiler seals it.
#[derive(Debug, Clone)]
pub enum ParametricRecipe {
    Ant {
        center: [f32; 3],
        heading: f32,
    },
    AntEyes {
        center: [f32; 3],
        heading: f32,
    },
    TerrainPatch {
        center_x: f32,
        center_z: f32,
    },
    SettlementPatch {
        center_x: f32,
    },
    Ribbon {
        control: &'static [[f32; 2]],
        width: f32,
        elevation: f32,
    },
    Cylinder {
        center: [f32; 3],
        radius: f32,
        height: f32,
        segments: u32,
    },
    /// A woody stem between two points. Qualia builds and transforms the cylinder.
    Stem {
        start: [f32; 3],
        end: [f32; 3],
        radius: f32,
        segments: u32,
    },
    Sphere {
        center: [f32; 3],
        radius: f32,
        latitude: u32,
        longitude: u32,
    },
    Ellipsoid {
        center: [f32; 3],
        radii: [f32; 3],
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
            Self::Ant { center, heading } => {
                ([0.0, 0.0, 0.0], crate::ants::ant(*center, *heading)?)
            }
            Self::AntEyes { center, heading } => {
                ([0.0, 0.0, 0.0], crate::ants::eyes(*center, *heading)?)
            }
            Self::TerrainPatch { center_x, center_z } => {
                ([0.0, 0.0, 0.0], crate::terrain::patch(*center_x, *center_z))
            }
            Self::SettlementPatch { center_x } => {
                ([0.0, 0.0, 0.0], crate::terrain::settlement_patch(*center_x))
            }
            Self::Ribbon {
                control,
                width,
                elevation,
            } => {
                let curve: Vec<Point3> = control
                    .iter()
                    .map(|p| Point3::new(p[0] as f64, p[1] as f64, 0.0))
                    .collect();
                let path: Vec<Point3> = (0..=24)
                    .map(|step| parametric_cad::bspline_eval(&curve, 3, step as f64 / 24.0))
                    .collect::<Result<_, _>>()
                    .map_err(|e| format!("road curve: {e:?}"))?;
                let mut left = vec![Point3::new(0.0, 0.0, 0.0); path.len()];
                let mut right = left.clone();
                parametric_cad::offset_polyline(&path, *width as f64 * 0.5, &mut left)
                    .map_err(|e| format!("road left edge: {e:?}"))?;
                parametric_cad::offset_polyline(&path, -*width as f64 * 0.5, &mut right)
                    .map_err(|e| format!("road right edge: {e:?}"))?;
                let mut verts = vec![Point3::new(0.0, 0.0, 0.0); path.len() * 2];
                let mut triangles = vec![[0u32; 3]; (path.len() - 1) * 2];
                let (nv, nt) =
                    parametric_cad::loft_profiles(&left, &right, &mut verts, &mut triangles)
                        .map_err(|e| format!("road loft: {e:?}"))?;
                let positions: Vec<[f32; 3]> = verts[..nv]
                    .iter()
                    .map(|p| [p.x as f32, *elevation, p.y as f32])
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
                    [0.0, 0.0, 0.0],
                    Mesh {
                        positions,
                        triangles: triangles[..nt].to_vec(),
                        min,
                        max,
                    },
                )
            }
            Self::Cylinder {
                center,
                radius,
                height,
                segments,
            } => (
                *center,
                authoring::cylinder(*radius, *height, *segments).map_err(|e| e.to_string())?,
            ),
            Self::Stem {
                start,
                end,
                radius,
                segments,
            } => {
                let delta = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
                let length =
                    (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
                if length < 0.001 {
                    return Err("stem endpoints coincide".into());
                }
                let y = [delta[0] / length, delta[1] / length, delta[2] / length];
                let horizontal = (y[0] * y[0] + y[1] * y[1]).sqrt();
                let x = if horizontal < 0.001 {
                    [1.0, 0.0, 0.0]
                } else {
                    [y[1] / horizontal, -y[0] / horizontal, 0.0]
                };
                let z = [
                    x[1] * y[2] - x[2] * y[1],
                    x[2] * y[0] - x[0] * y[2],
                    x[0] * y[1] - x[1] * y[0],
                ];
                let rotation = [
                    [x[0] as f64, x[1] as f64, x[2] as f64, 0.0],
                    [y[0] as f64, y[1] as f64, y[2] as f64, 0.0],
                    [z[0] as f64, z[1] as f64, z[2] as f64, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];
                let mesh =
                    authoring::cylinder(*radius, length, *segments).map_err(|e| e.to_string())?;
                let midpoint = [
                    (start[0] + end[0]) * 0.5,
                    (start[1] + end[1]) * 0.5,
                    (start[2] + end[2]) * 0.5,
                ];
                (midpoint, authoring::transform_mesh(&mesh, &rotation))
            }
            Self::Sphere {
                center,
                radius,
                latitude,
                longitude,
            } => (
                *center,
                authoring::uv_sphere(*radius, *latitude, *longitude).map_err(|e| e.to_string())?,
            ),
            Self::Ellipsoid {
                center,
                radii,
                latitude,
                longitude,
            } => {
                let sphere =
                    authoring::uv_sphere(1.0, *latitude, *longitude).map_err(|e| e.to_string())?;
                (
                    *center,
                    authoring::transform_mesh(
                        &sphere,
                        &authoring::scale(radii[0] as f64, radii[1] as f64, radii[2] as f64),
                    ),
                )
            }
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
    signal_online: bool,
) -> Vec<AssetRecipe> {
    const TANK_PROFILE: &[[f32; 2]] = &[
        [0.0, -0.66],
        [0.48, -0.66],
        [0.62, -0.49],
        [0.62, 0.42],
        [0.51, 0.61],
        [0.0, 0.61],
    ];
    let mut scene = Vec::with_capacity(160);
    scene.push(asset(
        "rc:asset/kestrel-earth-skirt",
        [0.46, 0.31, 0.24, 1.0],
        vec![block([0.0, -0.49, 0.0], [14.18, 0.38, 14.18])],
    ));
    scene.push(parametric(
        "rc:asset/ground",
        [0.58, 0.82, 0.43, 1.0],
        ParametricRecipe::SettlementPatch { center_x: 0.0 },
    ));
    scene.push(parametric(
        "rc:asset/main-road",
        [0.78, 0.55, 0.37, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[-6.1, 0.45], [-2.8, 0.16], [2.5, 0.86], [6.0, 0.48]],
            width: 1.15,
            elevation: 0.046,
        },
    ));
    scene.push(parametric(
        "rc:asset/main-road-workshop-spur",
        [0.77, 0.56, 0.39, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[1.7, -4.35], [1.38, -2.60], [1.88, -0.45], [1.38, 0.42]],
            width: 0.82,
            elevation: 0.048,
        },
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
            block([4.62, 0.49, -2.47], [0.9, 0.98, 1.15]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-roof",
        [0.72, 0.34, 0.25, 1.0],
        vec![
            roof([3.2, 1.6, -2.2], [2.75, 0.62, 2.12]),
            roof([4.62, 0.98, -2.47], [1.18, 0.38, 1.42]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-fascia",
        [0.27, 0.42, 0.46, 1.0],
        vec![
            block([1.96, 0.81, -1.17], [0.12, 1.62, 0.13]),
            block([4.44, 0.81, -1.17], [0.12, 1.62, 0.13]),
            block([2.43, 1.25, -1.18], [0.48, 0.08, 0.11]),
            block([3.97, 1.25, -1.18], [0.48, 0.08, 0.11]),
            block([3.2, 1.35, -1.17], [1.3, 0.11, 0.12]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-chimney",
        [0.69, 0.34, 0.28, 1.0],
        vec![
            block([2.34, 2.02, -2.64], [0.38, 1.08, 0.38]),
            block([2.34, 2.57, -2.64], [0.51, 0.11, 0.51]),
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
            block([0.0, 1.77, -5.25], [0.86, 0.96, 0.82]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hall-roof",
        [0.29, 0.49, 0.54, 1.0],
        vec![
            roof([0.0, 1.3, -5.25], [2.52, 0.63, 1.51]),
            roof([0.0, 2.27, -5.25], [1.09, 0.47, 1.06]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hall-bell-tower",
        [0.66, 0.36, 0.28, 1.0],
        vec![
            block([-0.24, 1.82, -4.82], [0.14, 0.44, 0.05]),
            block([0.24, 1.82, -4.82], [0.14, 0.44, 0.05]),
            block([0.0, 2.08, -4.82], [0.63, 0.09, 0.05]),
            block([0.0, 2.45, -5.25], [1.08, 0.11, 1.06]),
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
        "rc:asset/hall-banner",
        [0.94, 0.47, 0.38, 1.0],
        vec![
            block([0.0, 1.76, -4.77], [0.35, 0.55, 0.04]),
            block([0.0, 1.46, -4.76], [0.42, 0.09, 0.04]),
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
        if signal_online {
            [0.24, 0.84, 0.64, 1.0]
        } else {
            [0.25, 0.48, 0.52, 1.0]
        },
        vec![block([5.75, 2.55, 5.3], [0.26, 0.32, 0.26])],
    ));
    if signal_online {
        scene.push(parametric(
            "rc:asset/mast-signal",
            [0.42, 0.98, 0.72, 1.0],
            ParametricRecipe::Sphere {
                center: [5.75, 2.83, 5.3],
                radius: 0.19,
                latitude: 12,
                longitude: 18,
            },
        ));
    }
    // Landscape and architectural detail remain game-owned source recipes.
    // The rounded forms below use Qualia's authoring and parametric CAD APIs.
    scene.push(asset(
        "rc:asset/grass-verges",
        [0.36, 0.74, 0.39, 1.0],
        vec![
            block([-3.8, 0.012, 5.6], [4.0, 0.035, 1.35]),
            block([3.7, 0.012, 5.6], [4.5, 0.035, 1.35]),
            block([-3.5, 0.012, -6.2], [5.2, 0.035, 0.75]),
            block([4.0, 0.012, -6.2], [4.5, 0.035, 0.75]),
        ],
    ));
    // Broad low mounds break up the rectangular turf without obscuring roads.
    // Each is an authored ellipsoid from Qualia's sphere and transform APIs.
    for (id, center, radii, color) in [
        (
            "rc:asset/kestrel-meadow-nw",
            [-5.25, 0.025, -5.78],
            [1.35, 0.12, 0.65],
            [0.43, 0.78, 0.39, 1.0],
        ),
        (
            "rc:asset/kestrel-meadow-ne",
            [5.35, 0.025, -5.75],
            [1.38, 0.13, 0.67],
            [0.38, 0.73, 0.35, 1.0],
        ),
        (
            "rc:asset/kestrel-meadow-sw",
            [-5.37, 0.025, 5.72],
            [1.45, 0.13, 0.75],
            [0.40, 0.76, 0.36, 1.0],
        ),
        (
            "rc:asset/kestrel-meadow-se",
            [5.33, 0.025, 5.70],
            [1.42, 0.12, 0.72],
            [0.46, 0.80, 0.37, 1.0],
        ),
        (
            "rc:asset/kestrel-meadow-west",
            [-6.40, 0.025, 0.82],
            [0.51, 0.10, 1.12],
            [0.47, 0.77, 0.39, 1.0],
        ),
        (
            "rc:asset/kestrel-meadow-east",
            [6.37, 0.025, -0.55],
            [0.50, 0.09, 1.02],
            [0.41, 0.76, 0.37, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 7,
                longitude: 12,
            },
        ));
    }
    for (id, center, radii, color) in [
        (
            "rc:asset/kestrel-shrub-hall",
            [-1.23, 0.27, -4.27],
            [0.46, 0.28, 0.37],
            [0.20, 0.58, 0.31, 1.0],
        ),
        (
            "rc:asset/kestrel-shrub-workshop",
            [4.98, 0.31, -1.05],
            [0.55, 0.33, 0.39],
            [0.27, 0.64, 0.38, 1.0],
        ),
        (
            "rc:asset/kestrel-shrub-camp",
            [-4.61, 0.28, 3.75],
            [0.50, 0.30, 0.43],
            [0.25, 0.61, 0.33, 1.0],
        ),
        (
            "rc:asset/kestrel-shrub-garden",
            [-2.39, 0.22, -5.54],
            [0.43, 0.24, 0.38],
            [0.29, 0.68, 0.36, 1.0],
        ),
        (
            "rc:asset/kestrel-shrub-salvage",
            [6.42, 0.25, 2.26],
            [0.39, 0.27, 0.48],
            [0.23, 0.58, 0.34, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 8,
                longitude: 12,
            },
        ));
    }
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
        [0.32, 0.28, 0.31, 1.0],
        vec![
            block([3.2, 1.62, -1.11], [2.78, 0.08, 0.10]),
            block([3.2, 1.62, -3.29], [2.78, 0.08, 0.10]),
            block([3.2, 2.24, -2.2], [2.78, 0.09, 0.12]),
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
    scene.push(asset(
        "rc:asset/workshop-entry-awning",
        [0.22, 0.50, 0.58, 1.0],
        vec![
            roof([3.2, 1.66, -0.89], [1.70, 0.34, 0.78]),
            block([3.2, 0.035, -0.62], [1.80, 0.07, 0.69]),
        ],
    ));
    scene.push(asset(
        "rc:asset/workshop-door-glow",
        if online {
            [0.99, 0.80, 0.37, 1.0]
        } else {
            [0.34, 0.48, 0.53, 1.0]
        },
        vec![
            block([3.2, 1.05, -1.235], [0.54, 0.16, 0.035]),
            block([3.2, 0.22, -1.235], [0.54, 0.09, 0.035]),
        ],
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
        "rc:asset/market-canopy-cream",
        [0.99, 0.89, 0.65, 1.0],
        vec![
            block([-3.18, 1.36, -3.4], [0.34, 0.07, 1.50]),
            block([-2.32, 1.36, -3.4], [0.34, 0.07, 1.50]),
            block([-1.45, 1.36, -3.4], [0.34, 0.07, 1.50]),
        ],
    ));
    scene.push(asset(
        "rc:asset/market-display-baskets",
        [0.62, 0.37, 0.22, 1.0],
        vec![
            block([-2.74, 0.67, -2.60], [0.44, 0.20, 0.43]),
            block([-2.08, 0.67, -2.60], [0.44, 0.20, 0.43]),
            block([-1.43, 0.67, -2.60], [0.44, 0.20, 0.43]),
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
        for (id, x, z, hue) in [
            (
                "rc:asset/veggie-leaves-kale-a",
                -4.70,
                -4.20,
                [0.22, 0.52, 0.26, 1.0],
            ),
            (
                "rc:asset/veggie-leaves-kale-b",
                -3.02,
                -4.20,
                [0.31, 0.62, 0.32, 1.0],
            ),
            (
                "rc:asset/veggie-leaves-chard-a",
                -4.70,
                -3.30,
                [0.40, 0.65, 0.34, 1.0],
            ),
            (
                "rc:asset/veggie-leaves-chard-b",
                -3.02,
                -3.30,
                [0.28, 0.55, 0.31, 1.0],
            ),
        ] {
            scene.push(parametric(
                id,
                hue,
                ParametricRecipe::Ellipsoid {
                    center: [x, 0.42, z],
                    radii: [0.66, 0.20, 0.28],
                    latitude: 7,
                    longitude: 11,
                },
            ));
        }
        for (id, x, z) in [
            ("rc:asset/veggie-pumpkin-a", -5.13, -4.20),
            ("rc:asset/veggie-pumpkin-b", -4.08, -4.20),
            ("rc:asset/veggie-pumpkin-c", -3.57, -3.30),
        ] {
            scene.push(parametric(
                id,
                [0.93, 0.49, 0.20, 1.0],
                ParametricRecipe::Ellipsoid {
                    center: [x, 0.36, z],
                    radii: [0.17, 0.14, 0.16],
                    latitude: 7,
                    longitude: 10,
                },
            ));
        }
    }
    // The herb border is present before the player plants the vegetable beds.
    for (id, x, z, hue) in [
        (
            "rc:asset/herb-rosemary-a",
            -5.46,
            -4.95,
            [0.36, 0.57, 0.46, 1.0],
        ),
        (
            "rc:asset/herb-rosemary-b",
            -4.82,
            -4.95,
            [0.42, 0.63, 0.49, 1.0],
        ),
        (
            "rc:asset/herb-sage-a",
            -4.14,
            -4.95,
            [0.53, 0.67, 0.57, 1.0],
        ),
        (
            "rc:asset/herb-sage-b",
            -3.47,
            -4.95,
            [0.46, 0.62, 0.55, 1.0],
        ),
        ("rc:asset/herb-thyme", -2.80, -4.95, [0.36, 0.59, 0.39, 1.0]),
    ] {
        scene.push(parametric(
            id,
            hue,
            ParametricRecipe::Ellipsoid {
                center: [x, 0.25, z],
                radii: [0.23, 0.20, 0.19],
                latitude: 7,
                longitude: 10,
            },
        ));
    }
    scene.push(parametric(
        "rc:asset/garden-ant-mound",
        [0.57, 0.40, 0.27, 1.0],
        ParametricRecipe::Ellipsoid {
            center: [-5.82, 0.11, -5.49],
            radii: [0.38, 0.19, 0.35],
            latitude: 9,
            longitude: 14,
        },
    ));
    for (body_id, eyes_id, center, heading, color) in [
        (
            "rc:asset/garden-ant-scout",
            "rc:asset/garden-ant-scout-eyes",
            [-5.30, 0.20, -5.57],
            -0.25,
            [0.32, 0.17, 0.12, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-a",
            "rc:asset/garden-ant-worker-a-eyes",
            [-4.62, 0.20, -5.46],
            0.18,
            [0.39, 0.21, 0.14, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-b",
            "rc:asset/garden-ant-worker-b-eyes",
            [-3.96, 0.20, -5.62],
            -0.34,
            [0.29, 0.17, 0.13, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-c",
            "rc:asset/garden-ant-worker-c-eyes",
            [-5.02, 0.20, -5.38],
            0.35,
            [0.35, 0.19, 0.13, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-d",
            "rc:asset/garden-ant-worker-d-eyes",
            [-4.86, 0.20, -5.69],
            -0.16,
            [0.31, 0.17, 0.12, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-e",
            "rc:asset/garden-ant-worker-e-eyes",
            [-4.37, 0.20, -5.31],
            0.41,
            [0.38, 0.20, 0.14, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-f",
            "rc:asset/garden-ant-worker-f-eyes",
            [-4.21, 0.20, -5.69],
            -0.18,
            [0.30, 0.16, 0.12, 1.0],
        ),
        (
            "rc:asset/garden-ant-worker-g",
            "rc:asset/garden-ant-worker-g-eyes",
            [-3.71, 0.20, -5.49],
            0.24,
            [0.35, 0.18, 0.12, 1.0],
        ),
    ] {
        scene.push(parametric(
            body_id,
            color,
            ParametricRecipe::Ant { center, heading },
        ));
        scene.push(parametric(
            eyes_id,
            [0.97, 0.88, 0.67, 1.0],
            ParametricRecipe::AntEyes { center, heading },
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
    // Eucalypts need a pale, slightly leaning trunk, open fork and fine
    // blue-green leaf masses. Keep the established tree IDs for scene picks.
    for (trunk_id, crown_id, fork_id, bark_id, x, z, lean, tint) in [
        (
            "rc:asset/tree-trunk-nw",
            "rc:asset/tree-crown-nw",
            "rc:asset/gum-fork-nw",
            "rc:asset/gum-bark-nw",
            -6.15,
            -5.5,
            -0.22,
            [0.34, 0.57, 0.52, 1.0],
        ),
        (
            "rc:asset/tree-trunk-ne",
            "rc:asset/tree-crown-ne",
            "rc:asset/gum-fork-ne",
            "rc:asset/gum-bark-ne",
            6.1,
            -5.2,
            0.25,
            [0.40, 0.64, 0.54, 1.0],
        ),
        (
            "rc:asset/tree-trunk-sw",
            "rc:asset/tree-crown-sw",
            "rc:asset/gum-fork-sw",
            "rc:asset/gum-bark-sw",
            -6.1,
            5.2,
            -0.26,
            [0.38, 0.59, 0.50, 1.0],
        ),
        (
            "rc:asset/tree-trunk-se",
            "rc:asset/tree-crown-se",
            "rc:asset/gum-fork-se",
            "rc:asset/gum-bark-se",
            6.1,
            4.8,
            0.21,
            [0.35, 0.61, 0.55, 1.0],
        ),
    ] {
        scene.push(parametric(
            trunk_id,
            [0.83, 0.79, 0.67, 1.0],
            ParametricRecipe::Stem {
                start: [x, 0.02, z],
                end: [x + lean, 2.00, z - 0.12],
                radius: 0.16,
                segments: 9,
            },
        ));
        scene.push(parametric(
            fork_id,
            [0.72, 0.65, 0.52, 1.0],
            ParametricRecipe::Stem {
                start: [x + lean * 0.70, 1.43, z - 0.08],
                end: [x + lean + 0.68, 2.52, z + 0.26],
                radius: 0.105,
                segments: 8,
            },
        ));
        scene.push(parametric(
            bark_id,
            [0.61, 0.48, 0.36, 1.0],
            ParametricRecipe::Stem {
                start: [x + lean * 0.28, 0.55, z - 0.02],
                end: [x + lean * 0.45, 0.96, z - 0.04],
                radius: 0.169,
                segments: 9,
            },
        ));
        scene.push(parametric(
            crown_id,
            tint,
            ParametricRecipe::Ellipsoid {
                center: [x + lean, 2.75, z - 0.14],
                radii: [0.78, 0.36, 0.58],
                latitude: 8,
                longitude: 12,
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
            [0.43, 0.67, 0.57, 1.0],
        ),
        (
            "rc:asset/canopy-nw-b",
            [-5.83, 2.31, -5.62],
            0.56,
            [0.57, 0.73, 0.59, 1.0],
        ),
        (
            "rc:asset/canopy-ne-a",
            [5.75, 2.22, -5.13],
            0.59,
            [0.40, 0.64, 0.55, 1.0],
        ),
        (
            "rc:asset/canopy-ne-b",
            [6.45, 2.30, -5.28],
            0.55,
            [0.58, 0.75, 0.61, 1.0],
        ),
        (
            "rc:asset/canopy-sw-a",
            [-6.48, 2.20, 5.28],
            0.56,
            [0.43, 0.65, 0.54, 1.0],
        ),
        (
            "rc:asset/canopy-sw-b",
            [-5.85, 2.29, 5.05],
            0.59,
            [0.57, 0.72, 0.58, 1.0],
        ),
        (
            "rc:asset/canopy-se-a",
            [5.76, 2.17, 4.74],
            0.58,
            [0.39, 0.64, 0.52, 1.0],
        ),
        (
            "rc:asset/canopy-se-b",
            [6.44, 2.26, 4.88],
            0.56,
            [0.53, 0.72, 0.59, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center: [center[0], center[1] + 0.38, center[2]],
                radii: [radius, radius * 0.60, radius * 0.84],
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    // Narrow pendent sprays keep the eucalypts distinct from the round fruit
    // trees and stay visible in the territory camera.
    for (id, center, color) in [
        (
            "rc:asset/gum-spray-nw-west",
            [-6.77, 2.44, -5.38],
            [0.30, 0.56, 0.48, 1.0],
        ),
        (
            "rc:asset/gum-spray-nw-east",
            [-5.52, 2.56, -5.54],
            [0.48, 0.69, 0.54, 1.0],
        ),
        (
            "rc:asset/gum-spray-ne-west",
            [5.50, 2.48, -5.12],
            [0.34, 0.59, 0.52, 1.0],
        ),
        (
            "rc:asset/gum-spray-ne-east",
            [6.76, 2.57, -5.26],
            [0.50, 0.71, 0.59, 1.0],
        ),
        (
            "rc:asset/gum-spray-sw-west",
            [-6.75, 2.43, 5.26],
            [0.33, 0.56, 0.49, 1.0],
        ),
        (
            "rc:asset/gum-spray-sw-east",
            [-5.48, 2.52, 5.02],
            [0.51, 0.70, 0.56, 1.0],
        ),
        (
            "rc:asset/gum-spray-se-west",
            [5.49, 2.43, 4.75],
            [0.33, 0.57, 0.48, 1.0],
        ),
        (
            "rc:asset/gum-spray-se-east",
            [6.76, 2.52, 4.87],
            [0.49, 0.70, 0.56, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii: [0.22, 0.47, 0.23],
                latitude: 8,
                longitude: 10,
            },
        ));
    }
    for (id, center, color) in [
        (
            "rc:asset/wattle-bloom-nw",
            [-5.45, 0.46, -5.87],
            [0.96, 0.76, 0.28, 1.0],
        ),
        (
            "rc:asset/wattle-bloom-sw",
            [-5.48, 0.43, 5.82],
            [0.94, 0.70, 0.25, 1.0],
        ),
        (
            "rc:asset/wattle-bloom-ne",
            [5.47, 0.41, -5.80],
            [0.98, 0.79, 0.31, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii: [0.30, 0.12, 0.20],
                latitude: 7,
                longitude: 10,
            },
        ));
    }
    // Distinct, rounded character silhouettes. Separate limbs, faces and hair
    // retain semantic identities for later Qualia animation clips.
    const COAT_PROFILE: &[[f32; 2]] = &[
        [0.0, -0.34],
        [0.23, -0.34],
        [0.34, -0.20],
        [0.30, 0.15],
        [0.19, 0.35],
        [0.0, 0.35],
    ];
    for (body_id, head_id, feet_id, arm_id, face_id, hair_id, x, z, coat) in [
        (
            "rc:asset/resident-ada-coat",
            "rc:asset/resident-ada-head",
            "rc:asset/resident-ada-boots",
            "rc:asset/resident-ada-arms",
            "rc:asset/resident-ada-face",
            "rc:asset/resident-ada-hair",
            -2.2,
            1.8,
            [0.95, 0.35, 0.38, 1.0],
        ),
        (
            "rc:asset/resident-bo-coat",
            "rc:asset/resident-bo-head",
            "rc:asset/resident-bo-boots",
            "rc:asset/resident-bo-arms",
            "rc:asset/resident-bo-face",
            "rc:asset/resident-bo-hair",
            1.0,
            0.4,
            [0.35, 0.66, 0.85, 1.0],
        ),
        (
            "rc:asset/resident-cam-coat",
            "rc:asset/resident-cam-head",
            "rc:asset/resident-cam-boots",
            "rc:asset/resident-cam-arms",
            "rc:asset/resident-cam-face",
            "rc:asset/resident-cam-hair",
            -2.0,
            -2.0,
            [0.91, 0.67, 0.29, 1.0],
        ),
        (
            "rc:asset/resident-dev-coat",
            "rc:asset/resident-dev-head",
            "rc:asset/resident-dev-boots",
            "rc:asset/resident-dev-arms",
            "rc:asset/resident-dev-face",
            "rc:asset/resident-dev-hair",
            3.5,
            -0.6,
            [0.62, 0.46, 0.83, 1.0],
        ),
    ] {
        scene.push(parametric(
            body_id,
            coat,
            ParametricRecipe::Revolve {
                center: [x, 0.58, z],
                profile: COAT_PROFILE,
                segments: 16,
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
        scene.push(asset(
            arm_id,
            coat,
            vec![
                block([x - 0.35, 0.65, z], [0.16, 0.48, 0.19]),
                block([x + 0.35, 0.65, z], [0.16, 0.48, 0.19]),
            ],
        ));
        scene.push(asset(
            face_id,
            [0.12, 0.21, 0.26, 1.0],
            vec![
                block([x - 0.085, 1.07, z + 0.215], [0.035, 0.045, 0.025]),
                block([x + 0.085, 1.07, z + 0.215], [0.035, 0.045, 0.025]),
            ],
        ));
        scene.push(parametric(
            hair_id,
            [0.20, 0.22, 0.24, 1.0],
            ParametricRecipe::Sphere {
                center: [x, 1.23, z - 0.035],
                radius: 0.22,
                latitude: 8,
                longitude: 12,
            },
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
    orchard_harvested: bool,
) -> Vec<AssetRecipe> {
    let mut scene = Vec::with_capacity(78);
    scene.push(asset(
        "rc:asset/saltwind-earth-skirt",
        [0.54, 0.38, 0.27, 1.0],
        vec![block([16.0, -0.49, 0.0], [14.18, 0.38, 14.18])],
    ));
    scene.push(parametric(
        "rc:asset/saltwind-ground",
        [0.73, 0.81, 0.48, 1.0],
        ParametricRecipe::SettlementPatch { center_x: 16.0 },
    ));
    for (id, center, radii, color) in [
        (
            "rc:asset/saltwind-meadow-nw",
            [11.1, 0.02, -5.8],
            [1.55, 0.13, 0.70],
            [0.64, 0.77, 0.42, 1.0],
        ),
        (
            "rc:asset/saltwind-meadow-ne",
            [20.7, 0.02, -5.7],
            [1.58, 0.14, 0.76],
            [0.59, 0.75, 0.42, 1.0],
        ),
        (
            "rc:asset/saltwind-meadow-sw",
            [11.1, 0.02, 5.8],
            [1.50, 0.13, 0.69],
            [0.61, 0.79, 0.43, 1.0],
        ),
        (
            "rc:asset/saltwind-meadow-se",
            [20.7, 0.02, 5.7],
            [1.47, 0.12, 0.77],
            [0.65, 0.80, 0.40, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 7,
                longitude: 12,
            },
        ));
    }
    for (trunk_id, branch_id, crown_id, crown_side_id, x, z, lean) in [
        (
            "rc:asset/saltwind-gum-west-trunk",
            "rc:asset/saltwind-gum-west-branch",
            "rc:asset/saltwind-gum-west-crown",
            "rc:asset/saltwind-gum-west-leaves",
            10.6,
            -5.7,
            -0.32,
        ),
        (
            "rc:asset/saltwind-gum-east-trunk",
            "rc:asset/saltwind-gum-east-branch",
            "rc:asset/saltwind-gum-east-crown",
            "rc:asset/saltwind-gum-east-leaves",
            22.15,
            5.68,
            0.29,
        ),
    ] {
        scene.push(parametric(
            trunk_id,
            [0.82, 0.79, 0.69, 1.0],
            ParametricRecipe::Stem {
                start: [x, 0.02, z],
                end: [x + lean, 2.30, z - 0.16],
                radius: 0.17,
                segments: 9,
            },
        ));
        scene.push(parametric(
            branch_id,
            [0.69, 0.62, 0.52, 1.0],
            ParametricRecipe::Stem {
                start: [x + lean * 0.70, 1.65, z - 0.10],
                end: [x + lean + 0.77, 2.77, z + 0.29],
                radius: 0.10,
                segments: 8,
            },
        ));
        scene.push(parametric(
            crown_id,
            [0.37, 0.60, 0.53, 1.0],
            ParametricRecipe::Ellipsoid {
                center: [x + lean, 3.03, z - 0.20],
                radii: [0.95, 0.42, 0.66],
                latitude: 8,
                longitude: 12,
            },
        ));
        scene.push(parametric(
            crown_side_id,
            [0.56, 0.72, 0.60, 1.0],
            ParametricRecipe::Ellipsoid {
                center: [x + lean + 0.74, 2.88, z + 0.20],
                radii: [0.71, 0.34, 0.57],
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    for (id, center, color) in [
        (
            "rc:asset/saltwind-gum-west-spray-a",
            [9.96, 2.66, -5.89],
            [0.32, 0.55, 0.50, 1.0],
        ),
        (
            "rc:asset/saltwind-gum-west-spray-b",
            [11.13, 2.75, -5.47],
            [0.52, 0.69, 0.58, 1.0],
        ),
        (
            "rc:asset/saltwind-gum-east-spray-a",
            [21.60, 2.63, 5.53],
            [0.33, 0.56, 0.51, 1.0],
        ),
        (
            "rc:asset/saltwind-gum-east-spray-b",
            [22.87, 2.72, 5.85],
            [0.53, 0.70, 0.60, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii: [0.25, 0.50, 0.23],
                latitude: 8,
                longitude: 10,
            },
        ));
    }
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
        "rc:asset/canal-reeds",
        [0.55, 0.72, 0.35, 1.0],
        [-5.7, -4.4, -2.8, 2.1, 3.5, 5.2]
            .into_iter()
            .flat_map(|z| {
                [
                    block([6.83, 0.22, z], [0.06, 0.44, 0.08]),
                    block([9.18, 0.22, z + 0.3], [0.06, 0.44, 0.08]),
                ]
            })
            .collect(),
    ));
    for (id, z, major) in [
        ("rc:asset/canal-ripple-a", -5.0, 0.37),
        ("rc:asset/canal-ripple-b", -2.7, 0.48),
        ("rc:asset/canal-ripple-c", 2.3, 0.42),
        ("rc:asset/canal-ripple-d", 5.3, 0.34),
    ] {
        scene.push(parametric(
            id,
            [0.69, 0.92, 0.91, 1.0],
            ParametricRecipe::Torus {
                center: [8.0, if high_tide { 0.16 } else { 0.0 }, z],
                major,
                minor: 0.019,
                segments: 24,
                tube_segments: 6,
            },
        ));
    }
    for (id, center, radii) in [
        (
            "rc:asset/canal-stone-a",
            [6.72, 0.08, -3.55],
            [0.29, 0.18, 0.39],
        ),
        (
            "rc:asset/canal-stone-b",
            [9.30, 0.08, -4.70],
            [0.34, 0.20, 0.31],
        ),
        (
            "rc:asset/canal-stone-c",
            [6.73, 0.08, 3.12],
            [0.25, 0.17, 0.36],
        ),
        (
            "rc:asset/canal-stone-d",
            [9.27, 0.08, 5.04],
            [0.37, 0.21, 0.28],
        ),
    ] {
        scene.push(parametric(
            id,
            [0.77, 0.73, 0.60, 1.0],
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 7,
                longitude: 10,
            },
        ));
    }
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
        if bridge_open {
            vec![
                block([8.0, 0.42, -0.28], [2.6, 0.08, 0.09]),
                block([8.0, 0.42, 1.27], [2.6, 0.08, 0.09]),
                block([6.85, 0.24, -0.28], [0.10, 0.48, 0.10]),
                block([9.15, 0.24, -0.28], [0.10, 0.48, 0.10]),
                block([6.85, 0.24, 1.27], [0.10, 0.48, 0.10]),
                block([9.15, 0.24, 1.27], [0.10, 0.48, 0.10]),
            ]
        } else {
            vec![
                block([7.18, 0.42, -0.28], [0.70, 0.08, 0.09]),
                block([8.82, 0.42, -0.28], [0.70, 0.08, 0.09]),
                block([7.18, 0.42, 1.27], [0.70, 0.08, 0.09]),
                block([8.82, 0.42, 1.27], [0.70, 0.08, 0.09]),
            ]
        },
    ));
    if bridge_open {
        scene.push(asset(
            "rc:asset/bridge-deck-planks",
            [0.96, 0.77, 0.51, 1.0],
            (-5..=5)
                .map(|i| block([8.0 + i as f32 * 0.20, 0.225, 0.5], [0.16, 0.035, 1.43]))
                .collect(),
        ));
    }
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
    scene.push(parametric(
        "rc:asset/saltwind-main-road",
        [0.89, 0.72, 0.50, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[9.6, 0.55], [13.0, 0.25], [18.8, 0.83], [22.3, 0.50]],
            width: 1.25,
            elevation: 0.049,
        },
    ));
    scene.push(parametric(
        "rc:asset/saltwind-road-orchard-spur",
        [0.88, 0.71, 0.51, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[16.0, -4.20], [15.65, -2.6], [16.25, -0.55], [16.0, 0.48]],
            width: 0.88,
            elevation: 0.050,
        },
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
        vec![block([15.3, 0.79, 3.3], [2.4, 1.58, 1.85])],
    ));
    scene.push(asset(
        "rc:asset/barn-roof",
        [0.34, 0.51, 0.55, 1.0],
        vec![roof([15.3, 1.59, 3.3], [2.86, 0.69, 2.22])],
    ));
    scene.push(asset(
        "rc:asset/barn-eaves",
        [0.99, 0.80, 0.53, 1.0],
        vec![
            block([15.3, 1.6, 2.17], [2.92, 0.10, 0.12]),
            block([15.3, 1.6, 4.43], [2.92, 0.10, 0.12]),
            block([15.3, 2.29, 3.3], [2.84, 0.08, 0.10]),
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
    scene.push(asset(
        "rc:asset/barn-hay-bales",
        [0.94, 0.73, 0.36, 1.0],
        vec![
            block([13.83, 0.27, 2.06], [0.82, 0.50, 0.60]),
            block([16.76, 0.27, 2.06], [0.82, 0.50, 0.60]),
            block([13.83, 0.77, 2.06], [0.74, 0.47, 0.55]),
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
            block([19.0, 3.62, -1.42], [0.48, 0.47, 0.06]),
            block([19.0, 2.02, -1.42], [0.48, 0.47, 0.06]),
            block([18.20, 2.82, -1.42], [0.47, 0.48, 0.06]),
            block([19.80, 2.82, -1.42], [0.47, 0.48, 0.06]),
        ],
    ));
    scene.push(asset(
        "rc:asset/wind-pump-vane",
        [0.27, 0.54, 0.59, 1.0],
        vec![
            block([19.0, 3.49, -2.43], [0.10, 0.83, 0.10]),
            block([19.37, 3.74, -2.43], [0.75, 0.08, 0.10]),
            block([19.70, 3.84, -2.43], [0.17, 0.28, 0.10]),
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
        "rc:asset/saltwind-market-stripes",
        [0.99, 0.88, 0.62, 1.0],
        vec![
            block([11.14, 1.34, -1.8], [0.32, 0.07, 0.80]),
            block([11.80, 1.34, -1.8], [0.32, 0.07, 0.80]),
            block([12.46, 1.34, -1.8], [0.32, 0.07, 0.80]),
        ],
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
        "rc:asset/canal-boat-rail",
        [0.98, 0.82, 0.56, 1.0],
        vec![
            block([7.54, 0.29, 3.8], [0.07, 0.12, 1.92]),
            block([8.46, 0.29, 3.8], [0.07, 0.12, 1.92]),
            block([8.0, 0.28, 2.86], [0.98, 0.12, 0.08]),
            block([8.0, 0.28, 4.74], [0.98, 0.12, 0.08]),
            block([8.0, 0.32, 4.24], [0.80, 0.08, 0.22]),
        ],
    ));
    scene.push(asset(
        "rc:asset/canal-boat-awning",
        [0.96, 0.77, 0.48, 1.0],
        vec![
            roof([8.0, 0.81, 4.03], [0.89, 0.27, 0.92]),
            block([7.62, 0.55, 3.67], [0.06, 0.54, 0.06]),
            block([8.38, 0.55, 3.67], [0.06, 0.54, 0.06]),
            block([7.62, 0.55, 4.39], [0.06, 0.54, 0.06]),
            block([8.38, 0.55, 4.39], [0.06, 0.54, 0.06]),
        ],
    ));
    scene.push(asset(
        "rc:asset/canal-boat-hull-stripe",
        [0.29, 0.64, 0.71, 1.0],
        vec![
            block([7.54, 0.15, 3.8], [0.025, 0.08, 1.64]),
            block([8.46, 0.15, 3.8], [0.025, 0.08, 1.64]),
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
    scene.push(asset(
        "rc:asset/orchard-furrows",
        [0.68, 0.48, 0.30, 1.0],
        [3.24_f32, 3.60, 3.96, 4.94, 5.30, 5.66]
            .into_iter()
            .map(|z| block([18.3, 0.215, z], [3.43, 0.055, 0.105]))
            .collect(),
    ));
    if orchard_active {
        scene.push(asset(
            "rc:asset/orchard-crop-rows",
            [0.29, 0.68, 0.34, 1.0],
            [3.60_f32, 5.30]
                .into_iter()
                .flat_map(|z| {
                    (0..9)
                        .map(move |i| block([16.79 + i as f32 * 0.37, 0.36, z], [0.16, 0.30, 0.21]))
                })
                .collect(),
        ));
    }
    for (id, z, color) in [
        ("rc:asset/lavender-row-back", -5.25, [0.57, 0.47, 0.73, 1.0]),
        ("rc:asset/lavender-row-mid", -4.85, [0.66, 0.55, 0.79, 1.0]),
        (
            "rc:asset/lavender-row-front",
            -4.45,
            [0.51, 0.46, 0.70, 1.0],
        ),
    ] {
        scene.push(asset(
            id,
            [0.32, 0.55, 0.35, 1.0],
            (0..8)
                .map(|i| block([13.4 + i as f32 * 0.32, 0.14, z], [0.055, 0.28, 0.055]))
                .collect(),
        ));
        let flower_id = match id {
            "rc:asset/lavender-row-back" => "rc:asset/lavender-bloom-back",
            "rc:asset/lavender-row-mid" => "rc:asset/lavender-bloom-mid",
            _ => "rc:asset/lavender-bloom-front",
        };
        scene.push(asset(
            flower_id,
            color,
            (0..8)
                .map(|i| block([13.4 + i as f32 * 0.32, 0.34, z], [0.16, 0.17, 0.15]))
                .collect(),
        ));
    }
    for (id, center, radii, color) in [
        (
            "rc:asset/saltwind-windbreak-a",
            [21.80, 0.32, -2.50],
            [0.52, 0.36, 0.68],
            [0.31, 0.62, 0.39, 1.0],
        ),
        (
            "rc:asset/saltwind-windbreak-b",
            [21.94, 0.34, -0.90],
            [0.49, 0.37, 0.60],
            [0.27, 0.59, 0.36, 1.0],
        ),
        (
            "rc:asset/saltwind-windbreak-c",
            [21.85, 0.31, 1.04],
            [0.56, 0.34, 0.63],
            [0.34, 0.64, 0.38, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 8,
                longitude: 12,
            },
        ));
    }
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
    for (id, center, radii, color) in [
        (
            "rc:asset/orchard-canopy-a-lobe",
            [11.78, 2.23, 4.70],
            [0.64, 0.53, 0.60],
            [0.64, 0.81, 0.42, 1.0],
        ),
        (
            "rc:asset/orchard-canopy-b-lobe",
            [20.13, 2.26, 4.77],
            [0.63, 0.57, 0.56],
            [0.56, 0.78, 0.44, 1.0],
        ),
        (
            "rc:asset/orchard-canopy-c-lobe",
            [21.38, 2.20, -5.12],
            [0.65, 0.56, 0.57],
            [0.72, 0.84, 0.44, 1.0],
        ),
        (
            "rc:asset/saltwind-shrub-market",
            [12.93, 0.25, -3.67],
            [0.48, 0.28, 0.44],
            [0.31, 0.63, 0.38, 1.0],
        ),
        (
            "rc:asset/saltwind-shrub-barn",
            [17.02, 0.28, 4.25],
            [0.52, 0.31, 0.41],
            [0.29, 0.62, 0.34, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center,
                radii,
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    if orchard_active && !orchard_harvested {
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
    if orchard_harvested {
        scene.push(asset(
            "rc:asset/harvest-crates",
            [0.64, 0.39, 0.22, 1.0],
            vec![
                block([16.85, 0.22, 2.10], [0.75, 0.42, 0.72]),
                block([17.72, 0.22, 2.10], [0.75, 0.42, 0.72]),
                block([18.59, 0.22, 2.10], [0.75, 0.42, 0.72]),
                block([16.85, 0.47, 2.10], [0.82, 0.07, 0.79]),
                block([17.72, 0.47, 2.10], [0.82, 0.07, 0.79]),
                block([18.59, 0.47, 2.10], [0.82, 0.07, 0.79]),
            ],
        ));
        scene.push(asset(
            "rc:asset/harvest-produce",
            [0.97, 0.56, 0.23, 1.0],
            (0..3)
                .flat_map(|crate_index| {
                    (0..3).map(move |fruit_index| {
                        block(
                            [
                                16.6 + crate_index as f32 * 0.87 + fruit_index as f32 * 0.24,
                                0.55,
                                2.1,
                            ],
                            [0.18, 0.18, 0.18],
                        )
                    })
                })
                .collect(),
        ));
    }
    scene
}

/// Two deterministic heightfield tiles create a northern ridge with a low
/// crossing. They are exploration scenery rather than a second game engine.
pub fn northern_highlands() -> Vec<AssetRecipe> {
    let mut scene = Vec::with_capacity(20);
    for (id, base_id, x, color) in [
        (
            "rc:asset/terrain-west-ridge",
            "rc:asset/ridge-west-earth",
            0.0,
            [0.45, 0.72, 0.40, 1.0],
        ),
        (
            "rc:asset/terrain-east-ridge",
            "rc:asset/ridge-east-earth",
            16.0,
            [0.48, 0.70, 0.42, 1.0],
        ),
    ] {
        scene.push(asset(
            base_id,
            [0.47, 0.34, 0.26, 1.0],
            vec![block([x, -0.49, -14.0], [14.0, 0.38, 14.0])],
        ));
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::TerrainPatch {
                center_x: x,
                center_z: -14.0,
            },
        ));
    }
    scene.push(asset(
        "rc:asset/highland-creek",
        [0.28, 0.63, 0.73, 1.0],
        vec![block([8.0, 0.025, -14.0], [2.0, 0.07, 14.0])],
    ));
    for (id, x, z, radii) in [
        (
            "rc:asset/ridge-boulder-west",
            -4.9,
            -13.9,
            [0.49, 0.36, 0.61],
        ),
        (
            "rc:asset/ridge-boulder-east",
            19.8,
            -16.6,
            [0.61, 0.39, 0.46],
        ),
        ("rc:asset/valley-boulder", 8.9, -17.6, [0.39, 0.25, 0.45]),
    ] {
        scene.push(parametric(
            id,
            [0.60, 0.57, 0.48, 1.0],
            ParametricRecipe::Ellipsoid {
                center: [x, crate::terrain::height(x, z) + radii[1] - 0.02, z],
                radii,
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    for (id, x, z, color) in [
        (
            "rc:asset/ridge-shrub-west-a",
            -4.2,
            -17.4,
            [0.26, 0.55, 0.38, 1.0],
        ),
        (
            "rc:asset/ridge-shrub-west-b",
            1.9,
            -12.4,
            [0.37, 0.62, 0.42, 1.0],
        ),
        (
            "rc:asset/ridge-shrub-east-a",
            14.1,
            -17.2,
            [0.29, 0.56, 0.39, 1.0],
        ),
        (
            "rc:asset/ridge-shrub-east-b",
            19.1,
            -11.6,
            [0.40, 0.64, 0.43, 1.0],
        ),
    ] {
        scene.push(parametric(
            id,
            color,
            ParametricRecipe::Ellipsoid {
                center: [x, crate::terrain::height(x, z) + 0.28, z],
                radii: [0.46, 0.28, 0.42],
                latitude: 8,
                longitude: 12,
            },
        ));
    }
    scene
}
