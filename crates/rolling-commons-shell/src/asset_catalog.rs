//! Original Kestrel Flats source recipes. QualiaDB owns mesh assembly and `.10d`.
//! These are authored game assets, separate from the engine and its licence.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::scene_primitives::Primitive;
use qualia_core_db::specialized_libs::computational_geometry::botanical_hulls::{
    asymmetric_hull_capacity, asymmetric_hull_into, AsymmetricHullParams,
};
use qualia_core_db::specialized_libs::computational_geometry::math_geometry::AffineFrame3;
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::{
    extrude_profile_along_path, ExtrusionStation,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    authoring, parametric_cad, Point2, Point3,
};

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
    TerrainApron {
        center_x: f32,
        center_z: f32,
    },
    /// Authored model placement, evaluated by Qualia's mesh transform API.
    /// Pivot and target are explicit so scaling a resident does not move its
    /// saved site or turn a model part into a new semantic identity.
    Transform {
        source: Box<ParametricRecipe>,
        pivot: [f32; 3],
        target: [f32; 3],
        scale: [f32; 3],
    },
    /// Authored orientation through Qualia's public mesh transform surface.
    Yawed {
        source: Box<ParametricRecipe>,
        pivot: [f32; 3],
        degrees: f32,
        target: [f32; 3],
    },
    /// Authored three-axis placement through Qualia's public transform surface.
    Oriented {
        source: Box<ParametricRecipe>,
        pivot: [f32; 3],
        target: [f32; 3],
        degrees: [f32; 3],
    },
    /// Station-authored coachwork compiled by Qualia's closed convex loft.
    LoftedBody {
        sections: Vec<Vec<[f32; 3]>>,
    },
    WheelClearancePanel {
        center_x: f32,
        thickness: f32,
        z: [f32; 2],
        bottom: f32,
        top: [f32; 2],
        wheel_y: f32,
        clearance_radius: f32,
        axles: Vec<f32>,
        segments: u16,
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
    /// A deterministic cluster of narrow leaf forms compiled into one mesh.
    /// This gives gum and wattle silhouettes readable foliage detail without
    /// uploading one draw/scene object for every leaf.
    EllipsoidCluster {
        instances: Vec<EllipsoidInstance>,
        latitude: u32,
        longitude: u32,
    },
    /// A merged set of tapered, articulated limbs authored from Qualia's
    /// parametric revolve primitive. Endpoints are local to `center`.
    LimbCluster {
        center: [f32; 3],
        limbs: Vec<LimbSegment>,
        radial_segments: usize,
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
    /// A revolved profile stretched into an elongated rounded body, such as a
    /// home-on-wheels pod. Qualia revolves, scales and transforms the mesh.
    Pod {
        center: [f32; 3],
        profile: &'static [[f32; 2]],
        segments: usize,
        stretch: [f32; 3],
    },
    /// A wheel: a Qualia cylinder turned onto its axle and placed.
    Wheel {
        center: [f32; 3],
        radius: f32,
        width: f32,
        segments: u32,
    },
    /// A wheel mounted on an X axle. The van faces along the scene's Z axis,
    /// so this keeps its paired wheels on the two visible sides of the body.
    WheelX {
        center: [f32; 3],
        radius: f32,
        width: f32,
        segments: u32,
    },
    /// A barrel roof or canopy: a Qualia cylinder laid along X so only the
    /// upper arc shows above the eave line.
    Barrel {
        center: [f32; 3],
        radius: f32,
        length: f32,
        segments: u32,
    },
    /// A rotated flat panel (solar glass, awning fabric). Pitch tips around X,
    /// roll around Z, then yaw around Y; Qualia boxes and transforms it.
    TiltedBox {
        center: [f32; 3],
        size: [f32; 3],
        pitch_deg: f32,
        roll_deg: f32,
        yaw_deg: f32,
    },
    /// Several rotated boxes baked into one mesh: panel arrays, mounting
    /// rails, shade structures. Each part is placed relative to the group.
    PanelGroup {
        center: [f32; 3],
        parts: Vec<PanelPart>,
    },
    /// A procedural water plane with shoreline and depth falloff grid.
    WaterPlane {
        center: [f32; 3],
        size: [f32; 2],
        subdivisions: u32,
    },
    /// A continuous, naturally curving water surface lofted along a cubic
    /// Qualia B-spline. The renderer supplies the animated wave shading.
    Waterway {
        control: &'static [[f32; 2]],
        width: f32,
        elevation: f32,
        segments: u32,
        across: u32,
    },
    /// Tapered, curved margins that join a waterway into its surrounding
    /// ground instead of leaving a rectangular blue cutout.
    WaterwayBanks {
        control: &'static [[f32; 2]],
        water_width: f32,
        bank_width: f32,
        elevation: f32,
        segments: u32,
    },
    /// A procedural vegetation cluster with dynamic blade triangles.
    VegetationPatch {
        center: [f32; 3],
        radius: f32,
        density: u32,
        blade_height: f32,
    },
    /// A 6-point asymmetric botanical tree crown envelope (Cescatti 1997).
    /// Qualia's native computational geometry compiles the watertight canopy mesh.
    BotanicalHull {
        center: [f32; 3],
        radii: [f32; 4],   // -X, +X, -Z, +Z in world space
        heights: [f32; 4], // -X, +X, -Z, +Z in world space
        bottom_elevation: f32,
        top_elevation: f32,
        bottom_shape: f32,
        top_shape: f32,
        slices: usize,
        stacks: usize,
    },
    /// A continuous swept cross-section along an arbitrary 3D path using
    /// Rotation-Minimizing Frames (Bishop double-reflection).
    SweepExtrusion {
        center: [f32; 3],
        axis: Vec<[f32; 3]>,
        profile: Vec<[f32; 2]>,
        stations: Option<Vec<ExtrusionStation>>,
        closed_section: bool,
        cap_ends: bool,
    },
}

/// One deterministic, oriented leaf or petal in a merged foliage mesh.
#[derive(Debug, Clone, Copy)]
pub struct EllipsoidInstance {
    pub center: [f32; 3],
    pub radii: [f32; 3],
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub roll_deg: f32,
}

/// One tapered limb segment. Radius is measured at `start` and `end`.
#[derive(Debug, Clone, Copy)]
pub struct LimbSegment {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub start_radius: f32,
    pub end_radius: f32,
}

fn limb_segment(start: [f32; 3], end: [f32; 3], start_radius: f32, end_radius: f32) -> LimbSegment {
    LimbSegment {
        start,
        end,
        start_radius,
        end_radius,
    }
}

/// Authored elbow/hand offsets shared by resident bind geometry and rigs.
pub(crate) fn community_work_pose(index: usize) -> ([f32; 3], [f32; 3], [f32; 3], [f32; 3]) {
    match index {
        // Planting: one hand stays close while the other reaches to a bed.
        0 => (
            [-0.36, 0.70, 0.02],
            [-0.39, 0.49, 0.12],
            [0.36, 0.71, 0.08],
            [0.27, 0.48, 0.34],
        ),
        // Repairing: a low tool grip and a hand braced near the waist.
        1 => (
            [-0.40, 0.72, 0.01],
            [-0.22, 0.56, 0.08],
            [0.40, 0.76, 0.10],
            [0.17, 0.72, 0.29],
        ),
        // Harvesting: both hands work in front of the body at different heights.
        2 => (
            [-0.39, 0.76, 0.07],
            [-0.28, 0.63, 0.29],
            [0.39, 0.72, 0.07],
            [0.31, 0.54, 0.30],
        ),
        // Welcoming/teaching: arms open so the silhouette reads at overview scale.
        3 => (
            [-0.45, 0.80, 0.02],
            [-0.60, 0.84, 0.04],
            [0.44, 0.81, 0.02],
            [0.61, 0.91, 0.04],
        ),
        // Watering: both hands angle down toward the shared growing area.
        4 => (
            [-0.37, 0.72, 0.08],
            [-0.43, 0.53, 0.25],
            [0.37, 0.71, 0.08],
            [0.30, 0.49, 0.32],
        ),
        // Carrying produce: hands meet around a crate-height load.
        _ => (
            [-0.40, 0.76, 0.07],
            [-0.23, 0.68, 0.21],
            [0.40, 0.76, 0.07],
            [0.23, 0.68, 0.21],
        ),
    }
}

/// One rotated box inside a `PanelGroup`, offset from the group center.
#[derive(Debug, Clone, Copy)]
pub struct PanelPart {
    pub offset: [f32; 3],
    pub size: [f32; 3],
    pub pitch_deg: f32,
    pub roll_deg: f32,
    pub yaw_deg: f32,
}

fn mesh_bounds(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for position in positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(position[axis]);
            max[axis] = max[axis].max(position[axis]);
        }
    }
    (min, max)
}

fn sampled_spline(control: &[[f32; 2]], segments: u32) -> Result<Vec<Point3>, String> {
    if control.len() != 4 || control.iter().flatten().any(|value| !value.is_finite()) {
        return Err("a curved waterway needs four finite cubic control points".into());
    }
    let curve: Vec<Point3> = control
        .iter()
        .map(|point| Point3::new(point[0] as f64, point[1] as f64, 0.0))
        .collect();
    (0..=segments.clamp(8, 256))
        .map(|step| {
            parametric_cad::bspline_eval(&curve, 3, step as f64 / segments.clamp(8, 256) as f64)
                .map_err(|error| format!("waterway curve: {error:?}"))
        })
        .collect()
}

fn mesh_from_positions(positions: Vec<[f32; 3]>, triangles: Vec<[u32; 3]>) -> Mesh {
    let (min, max) = mesh_bounds(&positions);
    Mesh {
        positions,
        triangles,
        min,
        max,
    }
}

fn tapered_limb_mesh(limb: LimbSegment, radial_segments: usize) -> Result<Mesh, String> {
    let delta = [
        limb.end[0] - limb.start[0],
        limb.end[1] - limb.start[1],
        limb.end[2] - limb.start[2],
    ];
    let length = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
    if !length.is_finite()
        || length < 0.005
        || !limb.start_radius.is_finite()
        || !limb.end_radius.is_finite()
        || limb.start_radius <= 0.0
        || limb.end_radius <= 0.0
        || limb.start_radius > 2.0
        || limb.end_radius > 2.0
    {
        return Err(format!(
            "invalid tapered limb dimensions: length={length}, start={:?}, end={:?}, r0={}, r1={}",
            limb.start, limb.end, limb.start_radius, limb.end_radius
        ));
    }

    let half = length * 0.5;
    let profile = [
        Point3::new(0.0, -half as f64, 0.0),
        Point3::new(limb.start_radius as f64, -half as f64, 0.0),
        Point3::new(limb.end_radius as f64, half as f64, 0.0),
        Point3::new(0.0, half as f64, 0.0),
    ];
    let sides = radial_segments.clamp(8, 24);
    let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); profile.len() * sides];
    let mut triangles = vec![[0u32; 3]; (profile.len() - 1) * sides * 2];
    let (vertex_count, triangle_count) =
        parametric_cad::revolve_profile(&profile, sides, &mut vertices, &mut triangles)
            .map_err(|error| format!("limb revolve: {error:?}"))?;
    let positions: Vec<[f32; 3]> = vertices[..vertex_count]
        .iter()
        .map(|point| [point.x as f32, point.y as f32, point.z as f32])
        .collect();
    let (min, max) = mesh_bounds(&positions);
    let local = Mesh {
        positions,
        triangles: triangles[..triangle_count].to_vec(),
        min,
        max,
    };

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
    let rotated = authoring::transform_mesh(&local, &rotation);
    let midpoint = [
        (limb.start[0] + limb.end[0]) * 0.5,
        (limb.start[1] + limb.end[1]) * 0.5,
        (limb.start[2] + limb.end[2]) * 0.5,
    ];
    Ok(authoring::transform_mesh(
        &rotated,
        &authoring::translation(midpoint[0] as f64, midpoint[1] as f64, midpoint[2] as f64),
    ))
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
            Self::TerrainApron { center_x, center_z } => {
                ([0.0; 3], crate::terrain::apron(*center_x, *center_z))
            }
            Self::Transform {
                source,
                pivot,
                target,
                scale,
            } => {
                if pivot
                    .iter()
                    .chain(target.iter())
                    .chain(scale.iter())
                    .any(|v| !v.is_finite())
                    || scale.iter().any(|v| *v <= 0.0 || *v > 64.0)
                {
                    return Err("invalid authored model transform".into());
                }
                let mesh = source.compile()?;
                let local = authoring::transform_mesh(
                    &mesh,
                    &authoring::translation(-pivot[0] as f64, -pivot[1] as f64, -pivot[2] as f64),
                );
                let scaled = authoring::transform_mesh(
                    &local,
                    &authoring::scale(scale[0] as f64, scale[1] as f64, scale[2] as f64),
                );
                return Ok(authoring::transform_mesh(
                    &scaled,
                    &authoring::translation(target[0] as f64, target[1] as f64, target[2] as f64),
                ));
            }
            Self::WheelClearancePanel {
                center_x,
                thickness,
                z,
                bottom,
                top,
                wheel_y,
                clearance_radius,
                axles,
                segments,
            } => {
                use qualia_core_db::specialized_libs::computational_geometry::wheel_panel::{
                    wheel_clearance_panel_into, WheelPanel,
                };
                let count = (*segments as usize + 1).min(193);
                let mut positions = vec![[0.0; 3]; count * 10];
                let mut triangles = vec![[0; 3]; count * 8];
                let receipt = wheel_clearance_panel_into(
                    WheelPanel {
                        center_x: *center_x,
                        thickness: *thickness,
                        z: *z,
                        bottom: *bottom,
                        top: *top,
                        wheel_y: *wheel_y,
                        clearance_radius: *clearance_radius,
                        axles,
                        segments: *segments,
                    },
                    &mut positions,
                    &mut triangles,
                )
                .map_err(|e| e.to_string())?;
                positions.truncate(receipt.vertex_count);
                triangles.truncate(receipt.triangle_count);
                return Ok(Mesh {
                    positions,
                    triangles,
                    min: receipt.min,
                    max: receipt.max,
                });
            }
            Self::LoftedBody { sections } => {
                let count = sections.first().map_or(0, Vec::len);
                if !(2..=256).contains(&sections.len()) || !(3..=128).contains(&count) {
                    return Err("coachwork exceeds bounded loft topology".into());
                }
                let mut positions = vec![[0.0; 3]; sections.len() * (count + 1) * 2];
                let mut triangles = vec![[0; 3]; sections.len() * count * 2];
                let receipt =
                    authoring::loft_convex_sections_into(sections, &mut positions, &mut triangles)
                        .map_err(|e| e.to_string())?;
                positions.truncate(receipt.vertex_count);
                triangles.truncate(receipt.triangle_count);
                return Ok(Mesh {
                    positions,
                    triangles,
                    min: receipt.min,
                    max: receipt.max,
                });
            }
            Self::Oriented {
                source,
                pivot,
                target,
                degrees,
            } => {
                if pivot
                    .iter()
                    .chain(target.iter())
                    .chain(degrees.iter())
                    .any(|value| !value.is_finite())
                {
                    return Err("invalid authored orientation".into());
                }
                let mesh = source.compile()?;
                let local = authoring::transform_mesh(
                    &mesh,
                    &authoring::translation(-pivot[0] as f64, -pivot[1] as f64, -pivot[2] as f64),
                );
                let yaw = authoring::transform_mesh(
                    &local,
                    &authoring::rotation_y(degrees[1].to_radians() as f64),
                );
                let roll = authoring::transform_mesh(
                    &yaw,
                    &authoring::rotation_z(degrees[2].to_radians() as f64),
                );
                let pitch = authoring::transform_mesh(
                    &roll,
                    &authoring::rotation_x(degrees[0].to_radians() as f64),
                );
                return Ok(authoring::transform_mesh(
                    &pitch,
                    &authoring::translation(target[0] as f64, target[1] as f64, target[2] as f64),
                ));
            }
            Self::Yawed {
                source,
                pivot,
                degrees,
                target,
            } => {
                if !degrees.is_finite() || pivot.iter().chain(target.iter()).any(|v| !v.is_finite())
                {
                    return Err("invalid authored model orientation".into());
                }
                let mesh = source.compile()?;
                let local = authoring::transform_mesh(
                    &mesh,
                    &authoring::translation(-pivot[0] as f64, -pivot[1] as f64, -pivot[2] as f64),
                );
                let rotated = authoring::transform_mesh(
                    &local,
                    &authoring::rotation_y((*degrees as f64).to_radians()),
                );
                return Ok(authoring::transform_mesh(
                    &rotated,
                    &authoring::translation(target[0] as f64, target[1] as f64, target[2] as f64),
                ));
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
            Self::EllipsoidCluster {
                instances,
                latitude,
                longitude,
            } => {
                let unit =
                    authoring::uv_sphere(1.0, *latitude, *longitude).map_err(|e| e.to_string())?;
                let mut positions = Vec::with_capacity(instances.len() * unit.positions.len());
                let mut triangles = Vec::with_capacity(instances.len() * unit.triangles.len());
                for instance in instances {
                    let scale = authoring::scale(
                        instance.radii[0] as f64,
                        instance.radii[1] as f64,
                        instance.radii[2] as f64,
                    );
                    let mut rotation = authoring::identity();
                    if instance.yaw_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_y(instance.yaw_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    if instance.roll_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_z(instance.roll_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    if instance.pitch_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_x(instance.pitch_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    let translate = authoring::translation(
                        instance.center[0] as f64,
                        instance.center[1] as f64,
                        instance.center[2] as f64,
                    );
                    let transform =
                        authoring::mat_mul(&translate, &authoring::mat_mul(&rotation, &scale));
                    let leaf = authoring::transform_mesh(&unit, &transform);
                    let base = positions.len() as u32;
                    positions.extend_from_slice(&leaf.positions);
                    triangles.extend(
                        leaf.triangles
                            .iter()
                            .map(|tri| [base + tri[0], base + tri[1], base + tri[2]]),
                    );
                }
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for point in &positions {
                    for axis in 0..3 {
                        min[axis] = min[axis].min(point[axis]);
                        max[axis] = max[axis].max(point[axis]);
                    }
                }
                (
                    [0.0; 3],
                    Mesh {
                        positions,
                        triangles,
                        min,
                        max,
                    },
                )
            }
            Self::LimbCluster {
                center,
                limbs,
                radial_segments,
            } => {
                if limbs.is_empty() || limbs.len() > 64 {
                    return Err(format!(
                        "limb cluster must contain 1–64 segments (had {})",
                        limbs.len()
                    ));
                }
                let sides = (*radial_segments).clamp(8, 24);
                let mut positions = Vec::with_capacity(limbs.len() * sides * 4);
                let mut triangles = Vec::with_capacity(limbs.len() * sides * 6);
                for limb in limbs {
                    let segment = tapered_limb_mesh(*limb, *radial_segments)?;
                    let base = positions.len() as u32;
                    positions.extend_from_slice(&segment.positions);
                    triangles.extend(
                        segment
                            .triangles
                            .iter()
                            .map(|tri| [base + tri[0], base + tri[1], base + tri[2]]),
                    );
                }
                let (min, max) = mesh_bounds(&positions);
                (
                    *center,
                    Mesh {
                        positions,
                        triangles,
                        min,
                        max,
                    },
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
            Self::Pod {
                center,
                profile,
                segments,
                stretch,
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
                .map_err(|e| format!("pod revolve: {e:?}"))?;
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
                let mesh = authoring::transform_mesh(
                    &Mesh {
                        positions,
                        triangles: triangles[..nt].to_vec(),
                        min,
                        max,
                    },
                    &authoring::scale(stretch[0] as f64, stretch[1] as f64, stretch[2] as f64),
                );
                (*center, mesh)
            }
            Self::Wheel {
                center,
                radius,
                width,
                segments,
            } => {
                let wheel =
                    authoring::cylinder(*radius, *width, *segments).map_err(|e| e.to_string())?;
                let axle = authoring::rotation_x(std::f64::consts::FRAC_PI_2);
                (*center, authoring::transform_mesh(&wheel, &axle))
            }
            Self::WheelX {
                center,
                radius,
                width,
                segments,
            } => {
                let wheel =
                    authoring::cylinder(*radius, *width, *segments).map_err(|e| e.to_string())?;
                let axle = authoring::rotation_z(std::f64::consts::FRAC_PI_2);
                (*center, authoring::transform_mesh(&wheel, &axle))
            }
            Self::Barrel {
                center,
                radius,
                length,
                segments,
            } => {
                let barrel =
                    authoring::cylinder(*radius, *length, *segments).map_err(|e| e.to_string())?;
                let laid_flat = authoring::rotation_z(std::f64::consts::FRAC_PI_2);
                (*center, authoring::transform_mesh(&barrel, &laid_flat))
            }
            Self::TiltedBox {
                center,
                size,
                pitch_deg,
                roll_deg,
                yaw_deg,
            } => {
                let panel =
                    authoring::box_mesh(size[0], size[1], size[2]).map_err(|e| e.to_string())?;
                let mut rotation = authoring::identity();
                if *yaw_deg != 0.0 {
                    rotation = authoring::mat_mul(
                        &authoring::rotation_y(yaw_deg.to_radians() as f64),
                        &rotation,
                    );
                }
                if *roll_deg != 0.0 {
                    rotation = authoring::mat_mul(
                        &authoring::rotation_z(roll_deg.to_radians() as f64),
                        &rotation,
                    );
                }
                if *pitch_deg != 0.0 {
                    rotation = authoring::mat_mul(
                        &authoring::rotation_x(pitch_deg.to_radians() as f64),
                        &rotation,
                    );
                }
                (*center, authoring::transform_mesh(&panel, &rotation))
            }
            Self::PanelGroup { center, parts } => {
                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut triangles: Vec<[u32; 3]> = Vec::new();
                for part in parts {
                    let panel = authoring::box_mesh(part.size[0], part.size[1], part.size[2])
                        .map_err(|e| e.to_string())?;
                    let mut rotation = authoring::identity();
                    if part.yaw_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_y(part.yaw_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    if part.roll_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_z(part.roll_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    if part.pitch_deg != 0.0 {
                        rotation = authoring::mat_mul(
                            &authoring::rotation_x(part.pitch_deg.to_radians() as f64),
                            &rotation,
                        );
                    }
                    let local = authoring::transform_mesh(&panel, &rotation);
                    let placed = authoring::transform_mesh(
                        &local,
                        &authoring::translation(
                            part.offset[0] as f64,
                            part.offset[1] as f64,
                            part.offset[2] as f64,
                        ),
                    );
                    let base = positions.len() as u32;
                    for triangle in placed.triangles {
                        triangles.push([
                            triangle[0] + base,
                            triangle[1] + base,
                            triangle[2] + base,
                        ]);
                    }
                    positions.extend_from_slice(&placed.positions);
                }
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
                        triangles,
                        min,
                        max,
                    },
                )
            }
            Self::WaterPlane {
                center,
                size,
                subdivisions,
            } => {
                let subs = (*subdivisions).clamp(1, 32);
                let w = size[0];
                let l = size[1];
                let mut positions = Vec::new();
                let mut triangles = Vec::new();
                let dx = w / subs as f32;
                let dz = l / subs as f32;
                let half_w = w * 0.5;
                let half_l = l * 0.5;
                for i in 0..=subs {
                    for j in 0..=subs {
                        positions.push([-half_w + i as f32 * dx, 0.0, -half_l + j as f32 * dz]);
                    }
                }
                for i in 0..subs {
                    for j in 0..subs {
                        let row1 = i * (subs + 1) + j;
                        let row2 = (i + 1) * (subs + 1) + j;
                        triangles.push([row1, row1 + 1, row2]);
                        triangles.push([row1 + 1, row2 + 1, row2]);
                    }
                }
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for p in &positions {
                    for a in 0..3 {
                        min[a] = min[a].min(p[a]);
                        max[a] = max[a].max(p[a]);
                    }
                }
                (
                    *center,
                    Mesh {
                        positions,
                        triangles,
                        min,
                        max,
                    },
                )
            }
            Self::Waterway {
                control,
                width,
                elevation,
                segments,
                across,
            } => {
                if !width.is_finite() || *width <= 0.1 || !elevation.is_finite() {
                    return Err("curved waterway dimensions are invalid".into());
                }
                let path = sampled_spline(control, *segments)?;
                let longitudinal = path.len() - 1;
                let across = (*across).clamp(2, 16) as usize;
                let mut left = vec![Point3::new(0.0, 0.0, 0.0); path.len()];
                let mut right = left.clone();
                parametric_cad::offset_polyline(&path, *width as f64 * 0.5, &mut left)
                    .map_err(|error| format!("waterway left edge: {error:?}"))?;
                parametric_cad::offset_polyline(&path, -*width as f64 * 0.5, &mut right)
                    .map_err(|error| format!("waterway right edge: {error:?}"))?;
                let mut positions = Vec::with_capacity(path.len() * (across + 1));
                for index in 0..path.len() {
                    for column in 0..=across {
                        let t = column as f64 / across as f64;
                        let ripple =
                            ((index as f32 / longitudinal as f32) * std::f32::consts::TAU * 2.0)
                                .sin()
                                * 0.0025;
                        positions.push([
                            (left[index].x * (1.0 - t) + right[index].x * t) as f32,
                            *elevation + ripple,
                            (left[index].y * (1.0 - t) + right[index].y * t) as f32,
                        ]);
                    }
                }
                let mut triangles = Vec::with_capacity(longitudinal * across * 6);
                let row = across + 1;
                for index in 0..longitudinal {
                    for column in 0..across {
                        let a = (index * row + column) as u32;
                        let b = a + row as u32;
                        let c = a + 1;
                        let d = b + 1;
                        triangles.extend([[a, b, c], [c, b, d]]);
                    }
                }
                ([0.0; 3], mesh_from_positions(positions, triangles))
            }
            Self::WaterwayBanks {
                control,
                water_width,
                bank_width,
                elevation,
                segments,
            } => {
                if !water_width.is_finite()
                    || *water_width <= 0.1
                    || !bank_width.is_finite()
                    || *bank_width <= 0.1
                    || !elevation.is_finite()
                {
                    return Err("curved waterway bank dimensions are invalid".into());
                }
                let path = sampled_spline(control, *segments)?;
                let longitudinal = path.len() - 1;
                const ACROSS: usize = 8;
                let row = ACROSS + 1;
                let mut positions = Vec::with_capacity(path.len() * row * 2);
                let mut triangles = Vec::with_capacity(longitudinal * ACROSS * 12);
                for side in [1.0_f64, -1.0] {
                    let mut offsets = Vec::with_capacity(ACROSS + 1);
                    for column in 0..=ACROSS {
                        let fraction = column as f64 / ACROSS as f64;
                        let distance = *water_width as f64 * 0.5 + *bank_width as f64 * fraction;
                        let mut edge = vec![Point3::new(0.0, 0.0, 0.0); path.len()];
                        parametric_cad::offset_polyline(&path, side * distance, &mut edge)
                            .map_err(|error| format!("waterway bank edge: {error:?}"))?;
                        offsets.push(edge);
                    }
                    let base = positions.len() as u32;
                    for index in 0..path.len() {
                        let t = index as f32 / longitudinal as f32;
                        for column in 0..=ACROSS {
                            let fraction = column as f32 / ACROSS as f32;
                            let crest = (fraction * std::f32::consts::PI).sin();
                            let along = (t * std::f32::consts::TAU * 1.7 + side as f32).sin();
                            let y = *elevation - 0.028 + crest * 0.082 + along * crest * 0.008;
                            let point = offsets[column][index];
                            positions.push([point.x as f32, y, point.y as f32]);
                        }
                    }
                    for index in 0..longitudinal {
                        for column in 0..ACROSS {
                            let a = base + (index * row + column) as u32;
                            let b = a + row as u32;
                            let c = a + 1;
                            let d = b + 1;
                            if side > 0.0 {
                                triangles.extend([[a, c, b], [b, c, d]]);
                            } else {
                                triangles.extend([[a, b, c], [b, d, c]]);
                            }
                        }
                    }
                }
                ([0.0; 3], mesh_from_positions(positions, triangles))
            }
            Self::VegetationPatch {
                center,
                radius,
                density,
                blade_height,
            } => {
                let count = (*density).clamp(4, 128);
                let h = (*blade_height).max(0.1);
                let mut positions = Vec::new();
                let mut triangles = Vec::new();
                let r = *radius;
                for idx in 0..count {
                    let angle = idx as f32 * 2.3999632;
                    let dist = (idx as f32 / count as f32).sqrt() * r;
                    let bx = angle.cos() * dist;
                    let bz = angle.sin() * dist;
                    let blade_angle = angle * 1.7;
                    let half_w = 0.04;
                    let dx = blade_angle.cos() * half_w;
                    let dz = blade_angle.sin() * half_w;
                    let base_idx = positions.len() as u32;
                    positions.push([bx - dx, 0.0, bz - dz]);
                    positions.push([bx + dx, 0.0, bz + dz]);
                    positions.push([bx + dx * 0.3, h, bz + dz * 0.3]);
                    triangles.push([base_idx, base_idx + 1, base_idx + 2]);
                }
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for p in &positions {
                    for a in 0..3 {
                        min[a] = min[a].min(p[a]);
                        max[a] = max[a].max(p[a]);
                    }
                }
                (
                    *center,
                    Mesh {
                        positions,
                        triangles,
                        min,
                        max,
                    },
                )
            }
            Self::BotanicalHull {
                center,
                radii,
                heights,
                bottom_elevation,
                top_elevation,
                bottom_shape,
                top_shape,
                slices,
                stacks,
            } => {
                let sl = (*slices).clamp(6, 48);
                let st = (*stacks).clamp(3, 24);
                let (req_v, req_t) = asymmetric_hull_capacity(sl, st);
                let mut out_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
                let mut out_t = vec![[0u32; 3]; req_t];
                let bot = *bottom_elevation as f64;
                let top = (*top_elevation as f64).max(bot + 0.05);
                let params = AsymmetricHullParams {
                    neg_x_radius: (radii[0] as f64).max(0.01),
                    pos_x_radius: (radii[1] as f64).max(0.01),
                    neg_y_radius: (radii[2] as f64).max(0.01),
                    pos_y_radius: (radii[3] as f64).max(0.01),
                    neg_x_height: (heights[0] as f64).clamp(bot, top),
                    pos_x_height: (heights[1] as f64).clamp(bot, top),
                    neg_y_height: (heights[2] as f64).clamp(bot, top),
                    pos_y_height: (heights[3] as f64).clamp(bot, top),
                    bottom: Point3::new(0.0, 0.0, bot),
                    top: Point3::new(0.0, 0.0, top),
                    bottom_shape: (*bottom_shape as f64).max(0.1),
                    top_shape: (*top_shape as f64).max(0.1),
                    slices: sl,
                    stacks: st,
                };
                let (nv, nt) = asymmetric_hull_into(&params, &mut out_v, &mut out_t)
                    .map_err(|e| format!("botanical hull: {e:?}"))?;
                // Map Z (elevation in hull) -> Y (elevation in world), Y (horizontal) -> Z (horizontal).
                // Reverse triangle winding [0, 2, 1] to compensate for the Y-Z axis swap reflection.
                let positions: Vec<[f32; 3]> = out_v[..nv]
                    .iter()
                    .map(|p| [p.x as f32, p.z as f32, p.y as f32])
                    .collect();
                let triangles: Vec<[u32; 3]> = out_t[..nt]
                    .iter()
                    .map(|tri| [tri[0], tri[2], tri[1]])
                    .collect();
                let (min, max) = mesh_bounds(&positions);
                (
                    *center,
                    Mesh {
                        positions,
                        triangles,
                        min,
                        max,
                    },
                )
            }
            Self::SweepExtrusion {
                center,
                axis,
                profile,
                stations,
                closed_section,
                cap_ends,
            } => {
                if axis.len() < 2 || profile.len() < 2 {
                    return Err(
                        "sweep extrusion requires at least 2 axis points and 2 profile points"
                            .into(),
                    );
                }
                let path: Vec<Point3> = axis
                    .iter()
                    .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                    .collect();
                let cross_sec: Vec<Point2> = profile
                    .iter()
                    .map(|p| Point2::new(p[0] as f64, p[1] as f64))
                    .collect();
                let n_axis = path.len();
                let m_sec = cross_sec.len();
                let cap_verts = if *closed_section && *cap_ends { 2 } else { 0 };
                let req_v = n_axis * m_sec + cap_verts;
                let segs_per_ring = if *closed_section { m_sec } else { m_sec - 1 };
                let cap_tris = if *closed_section && *cap_ends {
                    2 * m_sec
                } else {
                    0
                };
                let req_t = (n_axis - 1) * segs_per_ring * 2 + cap_tris;
                let identity_frame = AffineFrame3 {
                    origin: Point3::new(0.0, 0.0, 0.0),
                    e0: Point3::new(1.0, 0.0, 0.0),
                    e1: Point3::new(0.0, 1.0, 0.0),
                    e2: Point3::new(0.0, 0.0, 1.0),
                };
                let mut frames_scratch = vec![identity_frame; n_axis];
                let mut out_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
                let mut out_t = vec![[0u32; 3]; req_t];
                let (nv, nt) = extrude_profile_along_path(
                    &path,
                    &cross_sec,
                    stations.as_deref(),
                    *closed_section,
                    *cap_ends,
                    &mut frames_scratch,
                    &mut out_v,
                    &mut out_t,
                )
                .map_err(|e| format!("sweep extrusion: {e:?}"))?;
                let positions: Vec<[f32; 3]> = out_v[..nv]
                    .iter()
                    .map(|p| [p.x as f32, p.y as f32, p.z as f32])
                    .collect();
                let (min, max) = mesh_bounds(&positions);
                (
                    *center,
                    Mesh {
                        positions,
                        triangles: out_t[..nt].to_vec(),
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

#[derive(Debug, Clone, Copy)]
pub struct MaterialFamily {
    pub id: &'static str,
    pub metallic: f32,
    pub roughness: f32,
}

pub const SOLAR_GLASS: MaterialFamily = MaterialFamily {
    id: "solar-glass",
    metallic: 0.0,
    roughness: 0.23,
};
pub const RUBBER: MaterialFamily = MaterialFamily {
    id: "rubber",
    metallic: 0.0,
    roughness: 0.84,
};
pub const FRAMED_GLASS: MaterialFamily = MaterialFamily {
    id: "framed-glass",
    metallic: 0.0,
    roughness: 0.24,
};
pub const GALVANISED_METAL: MaterialFamily = MaterialFamily {
    id: "galvanised-metal",
    metallic: 0.62,
    roughness: 0.34,
};
pub const WEATHERED_TIMBER: MaterialFamily = MaterialFamily {
    id: "weathered-timber",
    metallic: 0.0,
    roughness: 0.78,
};
pub const LIVING_FOLIAGE: MaterialFamily = MaterialFamily {
    id: "living-foliage",
    metallic: 0.0,
    roughness: 0.88,
};
pub const SOIL_AND_GRAVEL: MaterialFamily = MaterialFamily {
    id: "soil-and-gravel",
    metallic: 0.0,
    roughness: 0.96,
};
pub const FUR_AND_HIDE: MaterialFamily = MaterialFamily {
    id: "fur-and-hide",
    metallic: 0.0,
    roughness: 0.91,
};
pub const CLOTH_AND_SKIN: MaterialFamily = MaterialFamily {
    id: "cloth-and-skin",
    metallic: 0.0,
    roughness: 0.82,
};
pub const WEATHERED_CONCRETE: MaterialFamily = MaterialFamily {
    id: "weathered-concrete",
    metallic: 0.0,
    roughness: 0.93,
};
pub const PAINTED_METAL: MaterialFamily = MaterialFamily {
    id: "painted-metal",
    metallic: 0.12,
    roughness: 0.52,
};
pub const LIME_RENDER: MaterialFamily = MaterialFamily {
    id: "lime-render",
    metallic: 0.0,
    roughness: 0.91,
};
pub const TERRACOTTA: MaterialFamily = MaterialFamily {
    id: "terracotta",
    metallic: 0.0,
    roughness: 0.82,
};
pub const SHADE_CANVAS: MaterialFamily = MaterialFamily {
    id: "shade-canvas",
    metallic: 0.0,
    roughness: 0.89,
};

/// Existing recipe IDs get a first shared family; hero recipes can override
/// it at construction time. Material assignment is stored on the recipe and
/// travels with its compiled asset instead of being redetected at render time.
pub(crate) fn material_family_for(id: &str) -> MaterialFamily {
    let component = id.rsplit('/').next().unwrap_or(id);
    let has = |words: &[&str]| {
        words.iter().any(|word| {
            component
                .split(|character: char| character == '-' || character == '_')
                .any(|token| token == *word)
        })
    };

    if component.contains("solar-cell") || component.contains("solar-panel") {
        SOLAR_GLASS
    } else if has(&["wheel", "tyre", "tire", "rubber", "boot"])
        || id.contains("/wheel-")
        || id.contains("/tyre-")
    {
        RUBBER
    } else if has(&["glass", "glazing"]) || (has(&["window", "windows"]) && !has(&["frame"])) {
        FRAMED_GLASS
    } else if id.contains("/flora/") && has(&["trunk", "stem", "branch", "bark", "wood"]) {
        WEATHERED_TIMBER
    } else if id.contains("/flora/") {
        LIVING_FOLIAGE
    } else if has(&["trunk", "stem", "branch", "bark", "fork"]) {
        WEATHERED_TIMBER
    } else if has(&["foliage", "crown", "leaf", "spray"]) || component.starts_with("canopy-") {
        LIVING_FOLIAGE
    } else if has(&["shade", "awning", "canvas", "tent"]) && !has(&["frame", "metal"]) {
        SHADE_CANVAS
    } else if has(&[
        "metal", "mast", "steel", "pipe", "tank", "battery", "gutter", "rail", "frame", "vice",
        "press", "fascia", "chassis", "array", "grid",
    ]) {
        GALVANISED_METAL
    } else if has(&[
        "wood", "timber", "trunk", "stem", "branch", "bark", "bed", "bench", "crate", "fence",
        "deck", "board", "plank", "shelf", "tool", "door", "brace",
    ]) {
        WEATHERED_TIMBER
    } else if has(&[
        "foliage",
        "canopy",
        "crown",
        "leaf",
        "shoot",
        "flower",
        "gum",
        "wattle",
        "grass",
        "crop",
        "cone",
        "bottlebrush",
        "saltbush",
        "kangaroo-paw",
        "shrub",
    ]) {
        LIVING_FOLIAGE
    } else if component == "workshop-roof-panels" {
        GALVANISED_METAL
    } else if has(&[
        "ground", "terrain", "soil", "path", "road", "hill", "earth", "mound", "rock", "boulder",
        "bank", "canal", "creek", "meadow",
    ]) {
        SOIL_AND_GRAVEL
    } else if id.contains("/fauna/") {
        FUR_AND_HIDE
    } else if id.contains("resident")
        || has(&[
            "hand", "head", "face", "hair", "pants", "coat", "shirt", "dress",
        ])
    {
        CLOTH_AND_SKIN
    } else if has(&["floor", "concrete", "masonry", "cement", "slab"]) {
        WEATHERED_CONCRETE
    } else if has(&["roof", "tile", "terracotta", "chimney"]) {
        TERRACOTTA
    } else if has(&["wall", "workshop", "hall", "shelter"]) {
        LIME_RENDER
    } else {
        PAINTED_METAL
    }
}

pub struct AssetRecipe {
    pub id: String,
    pub parts: Vec<Primitive>,
    pub parametric: Option<ParametricRecipe>,
    pub color: [f32; 4],
    pub material_family: MaterialFamily,
    pub vibe_source: Option<String>,
    /// Renderer-owned pass role carried beside the `.10d` payload in HMC.
    pub render_role: String,
}

pub(crate) fn block(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Box { center, size }
}

fn roof(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Roof { center, size }
}

pub(crate) fn asset(id: &str, color: [f32; 4], parts: Vec<Primitive>) -> AssetRecipe {
    asset_with_family(id, color, parts, material_family_for(id))
}

fn asset_with_family(
    id: &str,
    color: [f32; 4],
    parts: Vec<Primitive>,
    material_family: MaterialFamily,
) -> AssetRecipe {
    AssetRecipe {
        id: id.to_string(),
        parts,
        parametric: None,
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn parametric(id: &str, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    AssetRecipe {
        id: id.to_string(),
        parts: Vec::new(),
        parametric: Some(recipe),
        color,
        material_family: material_family_for(id),
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn water_surface(id: &str, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    let mut result = parametric(id, color, recipe);
    result.render_role = "water".to_string();
    result
}

/// Authored scene recipes in metres; QualiaDB compiles each into its shared
/// mesh, material and `.10d` path. Stable IDs preserve picks and project links.
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
    const SERVICE_VAN_PROFILE: &[[f32; 2]] = &[
        [0.0, 0.0],
        [0.42, 0.0],
        [0.58, 0.045],
        [0.72, 0.18],
        [0.78, 0.34],
        [0.78, 0.94],
        [0.71, 1.15],
        [0.58, 1.32],
        [0.0, 1.36],
    ];
    let mut scene = Vec::with_capacity(160);
    scene.push(parametric(
        "rc:asset/kestrel-earth-skirt",
        [0.51, 0.52, 0.33, 1.0],
        ParametricRecipe::TerrainApron {
            center_x: 0.0,
            center_z: 0.0,
        },
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
            width: 2.05,
            elevation: 0.046,
        },
    ));
    scene.push(parametric(
        "rc:asset/main-road-workshop-spur",
        [0.77, 0.56, 0.39, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[0.84, -4.35], [0.72, -2.60], [0.80, -0.45], [0.80, 0.42]],
            width: 1.46,
            elevation: 0.048,
        },
    ));
    // Human-scale caravan coachwork, chassis, glazing and canvas annex.
    // Shelter upgrades retain the site's original pick and action identities.
    scene.extend(crate::camp_home::assets(upgrades));
    scene.push(asset(
        "rc:asset/camp-platform",
        [0.62, 0.39, 0.25, 1.0],
        vec![
            block([-3.0, -0.01, 2.7], [2.8, 0.12, 2.5]),
            block([-4.2, 0.25, 2.7], [0.08, 0.5, 0.08]),
            block([-1.8, 0.25, 2.7], [0.08, 0.5, 0.08]),
        ],
    ));
    // A long solar shade structure shelters the home-on-wheels bay, as on
    // the community-ground concept illustration: cream posts, a steel frame
    // and two tilted panel rows running the full bay. Fixed site
    // infrastructure, like the lights.
    const SHADE_PANEL_PARTS: &[PanelPart] = &[
        PanelPart {
            offset: [0.0, 0.06, -0.55],
            size: [4.56, 0.055, 0.94],
            pitch_deg: 20.0,
            roll_deg: 0.0,
            yaw_deg: 0.0,
        },
        PanelPart {
            offset: [0.0, -0.05, 0.55],
            size: [4.56, 0.055, 0.94],
            pitch_deg: 20.0,
            roll_deg: 0.0,
            yaw_deg: 0.0,
        },
        PanelPart {
            offset: [0.0, 0.10, 0.0],
            size: [4.6, 0.05, 0.10],
            pitch_deg: 0.0,
            roll_deg: 0.0,
            yaw_deg: 0.0,
        },
    ];
    for (id, x, z) in [
        ("rc:asset/camp-shade-post-a", -4.45, 1.65),
        ("rc:asset/camp-shade-post-b", -4.45, 3.75),
        ("rc:asset/camp-shade-post-c", -1.55, 1.65),
        ("rc:asset/camp-shade-post-d", -1.55, 3.75),
        ("rc:asset/camp-shade-post-e", 0.35, 1.65),
        ("rc:asset/camp-shade-post-f", 0.35, 3.75),
    ] {
        scene.push(parametric(
            id,
            [0.93, 0.91, 0.86, 1.0],
            ParametricRecipe::Cylinder {
                center: [x, 1.3, z],
                radius: 0.07,
                height: 2.6,
                segments: 10,
            },
        ));
    }
    scene.push(asset(
        "rc:asset/camp-shade-frame",
        [0.35, 0.36, 0.37, 1.0],
        vec![
            block([-2.05, 2.6, 1.65], [4.8, 0.08, 0.09]),
            block([-2.05, 2.6, 3.75], [4.8, 0.08, 0.09]),
            block([-4.45, 2.6, 2.7], [0.09, 0.08, 2.2]),
            block([-1.55, 2.6, 2.7], [0.09, 0.08, 2.2]),
            block([0.35, 2.6, 2.7], [0.09, 0.08, 2.2]),
        ],
    ));
    scene.push(parametric(
        "rc:asset/camp-shade-panels",
        [0.13, 0.22, 0.36, 1.0],
        ParametricRecipe::PanelGroup {
            center: [-2.05, 2.72, 2.7],
            parts: SHADE_PANEL_PARTS.to_vec(),
        },
    ));
    scene.extend(crate::workshop::assets(online, panels));
    scene.push(asset(
        "rc:asset/market-stall",
        [0.98, 0.64, 0.31, 1.0],
        vec![
            block([-2.2, 0.52, -3.4], [1.6, 1.0, 1.0]),
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
    let mut garden_bed_frame = Vec::new();
    for (x, z) in [(-4.8_f32, -4.2_f32), (-4.8, -3.3), (-3.0, -4.2)] {
        garden_bed_frame.extend([
            block([x, 0.16, z - 0.235], [1.42, 0.16, 0.085]),
            block([x, 0.16, z + 0.235], [1.42, 0.16, 0.085]),
            block([x - 0.665, 0.16, z], [0.085, 0.16, 0.48]),
            block([x + 0.665, 0.16, z], [0.085, 0.16, 0.48]),
        ]);
        for dx in [-0.66_f32, 0.66] {
            for dz in [-0.23_f32, 0.23] {
                garden_bed_frame.push(block([x + dx, 0.14, z + dz], [0.095, 0.24, 0.095]));
            }
        }
    }
    scene.push(asset(
        "rc:asset/garden-beds",
        if garden_active {
            [0.56, 0.40, 0.25, 1.0]
        } else {
            [0.43, 0.31, 0.20, 1.0]
        },
        garden_bed_frame,
    ));
    // One raised plot is a small water-connected growing channel. Keep its
    // frame in the ordinary scene so offline states still show the hardware.
    scene.push(asset(
        "rc:asset/hydroponic-channel-frame",
        [0.43, 0.39, 0.29, 1.0],
        vec![
            block([-3.62, 0.20, -3.51], [0.08, 0.34, 0.08]),
            block([-2.38, 0.20, -3.51], [0.08, 0.34, 0.08]),
            block([-3.62, 0.20, -3.09], [0.08, 0.34, 0.08]),
            block([-2.38, 0.20, -3.09], [0.08, 0.34, 0.08]),
            block([-3.0, 0.36, -3.54], [1.38, 0.10, 0.07]),
            block([-3.0, 0.36, -3.06], [1.38, 0.10, 0.07]),
            block([-3.67, 0.36, -3.30], [0.07, 0.10, 0.48]),
            block([-2.33, 0.36, -3.30], [0.07, 0.10, 0.48]),
            block([-3.0, 0.30, -3.30], [1.26, 0.035, 0.42]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hydroponic-channel-dividers",
        [0.78, 0.79, 0.70, 1.0],
        vec![
            block([-3.0, 0.405, -3.375], [1.20, 0.05, 0.025]),
            block([-3.0, 0.405, -3.225], [1.20, 0.05, 0.025]),
        ],
    ));
    scene.push(asset(
        "rc:asset/hydroponic-channel-liner",
        if water_online {
            [0.18, 0.37, 0.34, 1.0]
        } else {
            [0.34, 0.32, 0.27, 1.0]
        },
        vec![block([-3.0, 0.33, -3.30], [1.21, 0.025, 0.40])],
    ));
    if water_online {
        for (suffix, z) in [("west", -3.45), ("centre", -3.30), ("east", -3.15)] {
            scene.push(water_surface(
                &format!("rc:asset/hydroponic-water-{suffix}"),
                [0.20, 0.64, 0.76, 1.0],
                ParametricRecipe::WaterPlane {
                    center: [-3.0, 0.408, z],
                    size: [1.16, 0.105],
                    subdivisions: 3,
                },
            ));
        }
    }
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
    scene.push(parametric(
        "rc:asset/repair-van",
        [0.84, 0.81, 0.70, 1.0],
        ParametricRecipe::Pod {
            center: [0.0, 0.16, 4.55],
            profile: SERVICE_VAN_PROFILE,
            segments: 28,
            stretch: [1.08, 1.0, 0.74],
        },
    ));
    scene.push(asset(
        "rc:asset/van-windows",
        [0.12, 0.27, 0.34, 1.0],
        vec![
            block([0.73, 1.10, 4.20], [0.04, 0.35, 0.40]),
            block([-0.73, 1.10, 4.20], [0.04, 0.35, 0.40]),
            block([0.73, 0.91, 4.78], [0.04, 0.40, 0.42]),
            block([-0.73, 0.91, 4.78], [0.04, 0.40, 0.42]),
            block([0.0, 1.20, 4.02], [0.72, 0.30, 0.04]),
            block([0.0, 0.83, 5.06], [0.65, 0.27, 0.04]),
        ],
    ));
    scene.push(asset(
        "rc:asset/van-bumpers",
        [0.45, 0.47, 0.44, 1.0],
        vec![
            block([0.0, 0.34, 3.89], [1.52, 0.13, 0.12]),
            block([0.0, 0.34, 5.20], [1.42, 0.13, 0.12]),
            block([0.0, 0.16, 4.06], [0.94, 0.08, 0.08]),
        ],
    ));
    scene.push(asset(
        "rc:asset/van-door-trim",
        [0.56, 0.52, 0.43, 1.0],
        vec![
            block([0.75, 0.78, 4.68], [0.04, 0.08, 0.11]),
            block([-0.75, 0.78, 4.68], [0.04, 0.08, 0.11]),
            block([0.75, 0.63, 4.48], [0.04, 0.05, 0.38]),
            block([-0.75, 0.63, 4.48], [0.04, 0.05, 0.38]),
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
            [-4.61, 0.28, 4.42],
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
    for (id, control, width, elevation) in [
        (
            "rc:asset/path-to-garden",
            &[[-4.1_f32, 1.2], [-4.55, -0.7], [-4.72, -2.8], [-4.05, -5.1]][..],
            0.66,
            0.046,
        ),
        (
            "rc:asset/path-garden-to-workshop",
            &[
                [-3.75_f32, -2.25],
                [-2.10, -1.53],
                [0.65, -0.50],
                [2.82, -0.45],
            ][..],
            0.54,
            0.052,
        ),
        (
            "rc:asset/path-commons-arc",
            &[[-4.1_f32, 1.18], [-2.02, 1.76], [0.96, 1.55], [3.18, 0.58]][..],
            0.48,
            0.053,
        ),
    ] {
        scene.push(parametric(
            id,
            [0.88, 0.73, 0.53, 1.0],
            ParametricRecipe::Ribbon {
                control,
                width,
                elevation,
            },
        ));
    }
    scene.push(parametric(
        "rc:asset/road-markings",
        [0.88, 0.79, 0.48, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[-6.1, 0.45], [-2.8, 0.16], [2.5, 0.86], [6.0, 0.48]],
            width: 0.035,
            elevation: 0.052,
        },
    ));
    scene.push(parametric(
        "rc:asset/market-canopy",
        [0.96, 0.35, 0.32, 1.0],
        ParametricRecipe::Barrel {
            center: [-2.2, 1.04, -3.4],
            radius: 0.72,
            length: 2.2,
            segments: 16,
        },
    ));
    scene.push(asset(
        "rc:asset/market-canopy-cream",
        [0.99, 0.89, 0.65, 1.0],
        vec![
            block([-2.75, 1.02, -2.69], [0.34, 0.15, 0.05]),
            block([-2.2, 1.02, -2.69], [0.34, 0.15, 0.05]),
            block([-1.65, 1.02, -2.69], [0.34, 0.15, 0.05]),
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
                [-4.8, -3.0].into_iter().filter_map(move |x| {
                    (x == -4.8 || z == -4.2).then(|| block([x, 0.195, z], [1.24, 0.035, 0.43]))
                })
            })
            .collect(),
    ));
    for (name, x, z, length) in [
        ("north", -4.8_f32, -4.2_f32, 1.32_f32),
        ("centre", -4.8, -3.3, 1.32),
        ("east", -3.0, -4.2, 1.32),
    ] {
        scene.push(parametric(
            &format!("rc:asset/garden-dripline-pipe-{name}"),
            [0.26, 0.37, 0.34, 1.0],
            ParametricRecipe::Stem {
                start: [x - length * 0.5, 0.235, z + 0.17],
                end: [x + length * 0.5, 0.235, z + 0.17],
                radius: 0.025,
                segments: 8,
            },
        ));
    }
    scene.push(parametric(
        "rc:asset/garden-irrigation-main-pipe",
        [0.39, 0.43, 0.39, 1.0],
        ParametricRecipe::Stem {
            start: [-5.0, 0.56, -0.55],
            end: [-5.68, 0.245, -0.72],
            radius: 0.045,
            segments: 8,
        },
    ));
    if garden_active {
        let mut seedlings = Vec::new();
        for (plant, (x, z)) in [
            (-5.2_f32, -4.2_f32),
            (-4.7, -4.2),
            (-4.2, -4.2),
            (-3.4, -4.2),
            (-2.9, -4.2),
            (-2.4, -4.2),
            (-5.0, -3.3),
            (-4.55, -3.3),
            (-4.1, -3.3),
        ]
        .into_iter()
        .enumerate()
        {
            for leaf in 0..9 {
                let angle = leaf as f32 * std::f32::consts::TAU / 9.0 + plant as f32 * 0.61;
                let reach = 0.12 + ((leaf * 5 + plant * 3) % 7) as f32 * 0.018;
                seedlings.push(EllipsoidInstance {
                    center: [
                        x + angle.cos() * reach,
                        0.34 + (leaf % 3) as f32 * 0.018,
                        z + angle.sin() * reach * 0.62,
                    ],
                    radii: [0.075, 0.035, 0.18 + (leaf % 4) as f32 * 0.018],
                    pitch_deg: 18.0 + (leaf % 4) as f32 * 7.0,
                    yaw_deg: angle.to_degrees(),
                    roll_deg: (plant as f32 - leaf as f32 * 0.35) * 2.5,
                });
            }
        }
        scene.push(parametric(
            "rc:asset/garden-shoots",
            [0.27, 0.69, 0.27, 1.0],
            ParametricRecipe::EllipsoidCluster {
                instances: seedlings,
                latitude: 8,
                longitude: 11,
            },
        ));
        // Three compact cultivated heads bring a softer silhouette to the
        // trough. They are garden crops, not a claim about native wetland flora.
        let hydroponic_leaves = [-3.43_f32, -3.0, -2.57]
            .into_iter()
            .flat_map(|x| {
                [
                    (0.0_f32, 0.54_f32, [0.16, 0.055, 0.10], 0.0_f32),
                    (-0.11, 0.57, [0.15, 0.05, 0.085], -27.0),
                    (0.11, 0.57, [0.15, 0.05, 0.085], 27.0),
                    (-0.05, 0.64, [0.13, 0.05, 0.08], 58.0),
                    (0.05, 0.64, [0.13, 0.05, 0.08], -58.0),
                ]
                .into_iter()
                .map(move |(dx, y, radii, yaw_deg)| EllipsoidInstance {
                    center: [x + dx, y, -3.30],
                    radii,
                    pitch_deg: 8.0,
                    yaw_deg,
                    roll_deg: 0.0,
                })
            })
            .collect();
        scene.push(parametric(
            "rc:asset/hydroponic-greens",
            [0.29, 0.68, 0.28, 1.0],
            ParametricRecipe::EllipsoidCluster {
                instances: hydroponic_leaves,
                latitude: 7,
                longitude: 10,
            },
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
            let mut leaves = Vec::with_capacity(16);
            for leaf in 0..16 {
                let angle = leaf as f32 * std::f32::consts::TAU / 16.0;
                let reach = 0.16 + ((leaf * 7 + 3) % 9) as f32 * 0.012;
                leaves.push(EllipsoidInstance {
                    center: [
                        x + angle.cos() * reach,
                        0.40 + (leaf % 4) as f32 * 0.025,
                        z + angle.sin() * reach * 0.72,
                    ],
                    radii: [0.12, 0.043, 0.24 + (leaf % 5) as f32 * 0.014],
                    pitch_deg: 16.0 + (leaf % 5) as f32 * 8.0,
                    yaw_deg: angle.to_degrees() + (leaf % 3) as f32 * 7.0,
                    roll_deg: angle.sin() * 13.0,
                });
            }
            scene.push(parametric(
                id,
                hue,
                ParametricRecipe::EllipsoidCluster {
                    instances: leaves,
                    latitude: 7,
                    longitude: 10,
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
    scene.extend(crate::mature_gums::assets());
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
        ("rc:asset/van-wheel-fl", [-0.77, 0.26, 4.08]),
        ("rc:asset/van-wheel-fr", [0.77, 0.26, 4.08]),
        ("rc:asset/van-wheel-rl", [-0.77, 0.26, 5.00]),
        ("rc:asset/van-wheel-rr", [0.77, 0.26, 5.00]),
    ] {
        scene.push(parametric(
            id,
            [0.08, 0.10, 0.10, 1.0],
            ParametricRecipe::WheelX {
                center,
                radius: 0.27,
                width: 0.16,
                segments: 24,
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
    // Resident bind geometry keeps separate limbs, faces and hair identities.
    // The asset compiler attaches their shared Qualia rigs and activity clips.
    const COAT_PROFILE: &[[f32; 2]] = &[
        [0.0, -0.34],
        [0.23, -0.34],
        [0.31, -0.22],
        [0.29, 0.15],
        [0.18, 0.35],
        [0.0, 0.35],
    ];
    let residents = [
        (
            "rc:asset/resident-ada-coat",
            "rc:asset/resident-ada-head",
            "rc:asset/resident-ada-boots",
            "rc:asset/resident-ada-pants",
            "rc:asset/resident-ada-arms",
            "rc:asset/resident-ada-hands",
            "rc:asset/resident-ada-face",
            "rc:asset/resident-ada-hair",
            -2.2,
            1.8,
            [0.95, 0.35, 0.38, 1.0],
            [0.23, 0.27, 0.32, 1.0],
        ),
        (
            "rc:asset/resident-bo-coat",
            "rc:asset/resident-bo-head",
            "rc:asset/resident-bo-boots",
            "rc:asset/resident-bo-pants",
            "rc:asset/resident-bo-arms",
            "rc:asset/resident-bo-hands",
            "rc:asset/resident-bo-face",
            "rc:asset/resident-bo-hair",
            1.0,
            0.4,
            [0.35, 0.66, 0.85, 1.0],
            [0.27, 0.34, 0.39, 1.0],
        ),
        (
            "rc:asset/resident-cam-coat",
            "rc:asset/resident-cam-head",
            "rc:asset/resident-cam-boots",
            "rc:asset/resident-cam-pants",
            "rc:asset/resident-cam-arms",
            "rc:asset/resident-cam-hands",
            "rc:asset/resident-cam-face",
            "rc:asset/resident-cam-hair",
            -2.0,
            -2.0,
            [0.91, 0.67, 0.29, 1.0],
            [0.38, 0.31, 0.20, 1.0],
        ),
        (
            "rc:asset/resident-dev-coat",
            "rc:asset/resident-dev-head",
            "rc:asset/resident-dev-boots",
            "rc:asset/resident-dev-pants",
            "rc:asset/resident-dev-arms",
            "rc:asset/resident-dev-hands",
            "rc:asset/resident-dev-face",
            "rc:asset/resident-dev-hair",
            3.5,
            -0.6,
            [0.62, 0.46, 0.83, 1.0],
            [0.29, 0.28, 0.36, 1.0],
        ),
        (
            "rc:asset/resident-eve-coat",
            "rc:asset/resident-eve-head",
            "rc:asset/resident-eve-boots",
            "rc:asset/resident-eve-pants",
            "rc:asset/resident-eve-arms",
            "rc:asset/resident-eve-hands",
            "rc:asset/resident-eve-face",
            "rc:asset/resident-eve-hair",
            -2.35,
            4.1,
            [0.83, 0.41, 0.34, 1.0],
            [0.34, 0.25, 0.20, 1.0],
        ),
        (
            "rc:asset/resident-finn-coat",
            "rc:asset/resident-finn-head",
            "rc:asset/resident-finn-boots",
            "rc:asset/resident-finn-pants",
            "rc:asset/resident-finn-arms",
            "rc:asset/resident-finn-hands",
            "rc:asset/resident-finn-face",
            "rc:asset/resident-finn-hair",
            -1.6,
            4.4,
            [0.44, 0.56, 0.40, 1.0],
            [0.25, 0.32, 0.23, 1.0],
        ),
    ];
    const SKIN_TONES: [[f32; 4]; 6] = [
        [0.78, 0.52, 0.34, 1.0],
        [0.44, 0.29, 0.20, 1.0],
        [0.64, 0.40, 0.27, 1.0],
        [0.84, 0.67, 0.49, 1.0],
        [0.91, 0.72, 0.53, 1.0],
        [0.55, 0.35, 0.23, 1.0],
    ];
    const HAIR_TONES: [[f32; 4]; 6] = [
        [0.24, 0.10, 0.08, 1.0],
        [0.14, 0.13, 0.12, 1.0],
        [0.31, 0.20, 0.13, 1.0],
        [0.53, 0.48, 0.39, 1.0],
        [0.19, 0.12, 0.09, 1.0],
        [0.59, 0.40, 0.20, 1.0],
    ];
    for (resident_index, resident) in residents.into_iter().enumerate() {
        let model_start = scene.len();
        let (
            body_id,
            head_id,
            feet_id,
            pants_id,
            arm_id,
            hand_id,
            face_id,
            hair_id,
            x,
            z,
            coat,
            pants,
        ) = resident;
        let skin = SKIN_TONES[resident_index];
        let hair_color = HAIR_TONES[resident_index];
        scene.push(parametric(
            body_id,
            coat,
            ParametricRecipe::Revolve {
                center: [x, 0.72, z],
                profile: COAT_PROFILE,
                segments: 24,
            },
        ));
        scene.push(parametric(
            head_id,
            skin,
            ParametricRecipe::Ellipsoid {
                center: [x, 1.23, z],
                radii: [0.17, 0.22, 0.17],
                latitude: 12,
                longitude: 18,
            },
        ));
        let resident_key = body_id
            .strip_prefix("rc:asset/resident-")
            .and_then(|name| name.strip_suffix("-coat"))
            .unwrap_or("community-member");
        scene.push(parametric(
            &format!("rc:asset/resident-{resident_key}-neck"),
            skin,
            ParametricRecipe::Ellipsoid {
                center: [x, 1.035, z - 0.005],
                radii: [0.078, 0.115, 0.078],
                latitude: 10,
                longitude: 14,
            },
        ));
        scene.push(parametric(
            feet_id,
            [0.31, 0.35, 0.47, 1.0],
            ParametricRecipe::EllipsoidCluster {
                instances: vec![
                    EllipsoidInstance {
                        center: [x - 0.12, 0.095, z + 0.075],
                        radii: [0.12, 0.085, 0.18],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x + 0.12, 0.095, z + 0.075],
                        radii: [0.12, 0.085, 0.18],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                ],
                latitude: 9,
                longitude: 12,
            },
        ));
        scene.push(parametric(
            pants_id,
            pants,
            ParametricRecipe::LimbCluster {
                center: [x, 0.0, z],
                limbs: vec![
                    limb_segment([-0.12, 0.48, 0.0], [-0.13, 0.10, 0.04], 0.092, 0.057),
                    limb_segment([0.12, 0.48, 0.0], [0.13, 0.10, 0.04], 0.092, 0.057),
                ],
                radial_segments: 12,
            },
        ));
        let (left_elbow, left_hand, right_elbow, right_hand) = community_work_pose(resident_index);
        let left_shoulder = [-0.29, 0.93, 0.0];
        let right_shoulder = [0.29, 0.93, 0.0];
        scene.push(parametric(
            arm_id,
            coat,
            ParametricRecipe::LimbCluster {
                center: [x, 0.0, z],
                limbs: vec![
                    limb_segment(left_shoulder, left_elbow, 0.078, 0.063),
                    limb_segment(left_elbow, left_hand, 0.063, 0.048),
                    limb_segment(right_shoulder, right_elbow, 0.078, 0.063),
                    limb_segment(right_elbow, right_hand, 0.063, 0.048),
                ],
                radial_segments: 12,
            },
        ));
        scene.push(parametric(
            hand_id,
            skin,
            ParametricRecipe::EllipsoidCluster {
                instances: vec![
                    EllipsoidInstance {
                        center: [x + left_hand[0], left_hand[1], z + left_hand[2]],
                        radii: [0.066, 0.073, 0.061],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x + right_hand[0], right_hand[1], z + right_hand[2]],
                        radii: [0.066, 0.073, 0.061],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                ],
                latitude: 9,
                longitude: 12,
            },
        ));
        scene.push(parametric(
            face_id,
            [0.12, 0.21, 0.26, 1.0],
            ParametricRecipe::EllipsoidCluster {
                instances: vec![
                    EllipsoidInstance {
                        center: [x - 0.055, 1.26, z + 0.154],
                        radii: [0.016, 0.020, 0.012],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x + 0.055, 1.26, z + 0.154],
                        radii: [0.016, 0.020, 0.012],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x - 0.055, 1.30, z + 0.151],
                        radii: [0.033, 0.009, 0.010],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x + 0.055, 1.30, z + 0.151],
                        radii: [0.033, 0.009, 0.010],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x, 1.155, z + 0.170],
                        radii: [0.034, 0.008, 0.009],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                ],
                latitude: 7,
                longitude: 10,
            },
        ));
        scene.push(parametric(
            &format!("rc:asset/resident-{resident_key}-nose"),
            skin,
            ParametricRecipe::Ellipsoid {
                center: [x, 1.205, z + 0.176],
                radii: [0.024, 0.038, 0.028],
                latitude: 8,
                longitude: 10,
            },
        ));
        scene.push(parametric(
            hair_id,
            hair_color,
            ParametricRecipe::EllipsoidCluster {
                instances: vec![
                    EllipsoidInstance {
                        center: [x, 1.405, z - 0.045],
                        radii: [0.176, 0.087, 0.155],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 0.0,
                    },
                    EllipsoidInstance {
                        center: [x - 0.135, 1.315, z - 0.035],
                        radii: [0.061, 0.118, 0.135],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: -8.0,
                    },
                    EllipsoidInstance {
                        center: [x + 0.135, 1.315, z - 0.035],
                        radii: [0.061, 0.118, 0.135],
                        pitch_deg: 0.0,
                        yaw_deg: 0.0,
                        roll_deg: 8.0,
                    },
                ],
                latitude: 10,
                longitude: 14,
            },
        ));
        // Adult proportions: leg length, torso width and head size are authored
        // independently. Qualia transforms the source meshes; stable IDs and
        // the resident's world X/Z location remain unchanged.
        scene.extend(crate::resident_props::assets(
            resident_key,
            [x + left_hand[0], left_hand[1], z + left_hand[2]],
            [x + right_hand[0], right_hand[1], z + right_hand[2]],
        ));
        let ground = crate::terrain::settlement_height(0.0, x, z);
        for part in &mut scene[model_start..] {
            let Some(source) = part.parametric.take() else {
                continue;
            };
            let head_part = part.id == head_id
                || part.id == face_id
                || part.id == hair_id
                || part.id.ends_with("-nose");
            let (pivot, target, scale) = if head_part {
                ([x, 1.23, z], [x, ground + 1.64, z], [0.74, 0.64, 0.74])
            } else if part.id == pants_id {
                ([x, 0.10, z], [x, ground + 0.10, z], [0.74, 1.88, 0.74])
            } else if part.id == feet_id {
                ([x, 0.0, z], [x, ground, z], [0.80, 1.0, 0.85])
            } else {
                ([x, 0.48, z], [x, ground + 0.815, z], [0.74, 1.04, 0.74])
            };
            part.parametric = Some(ParametricRecipe::Transform {
                source: Box::new(source),
                pivot,
                target,
                scale,
            });
        }
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

#[cfg(test)]
mod material_family_tests {
    use super::*;

    #[test]
    fn waterways_and_banks_follow_a_curved_qualia_spline() {
        const CONTROL: &[[f32; 2]] = &[[0.0, 0.0], [1.0, 2.0], [3.0, -2.0], [4.0, 0.0]];
        let water = ParametricRecipe::Waterway {
            control: CONTROL,
            width: 1.8,
            elevation: 0.2,
            segments: 24,
            across: 8,
        }
        .compile()
        .unwrap();
        assert_eq!(water.positions.len(), 25 * 9);
        assert_eq!(water.triangles.len(), 24 * 8 * 2);
        assert!(water
            .positions
            .iter()
            .all(|point| point.iter().all(|v| v.is_finite())));
        let centerline: Vec<[f32; 3]> = water.positions.chunks_exact(9).map(|row| row[4]).collect();
        let z_span = centerline
            .iter()
            .map(|point| point[2])
            .fold(f32::NEG_INFINITY, f32::max)
            - centerline
                .iter()
                .map(|point| point[2])
                .fold(f32::INFINITY, f32::min);
        assert!(
            z_span > 0.2,
            "the water centerline must curve rather than stay a rectangle"
        );

        let banks = ParametricRecipe::WaterwayBanks {
            control: CONTROL,
            water_width: 1.8,
            bank_width: 1.2,
            elevation: 0.2,
            segments: 24,
        }
        .compile()
        .unwrap();
        assert_eq!(banks.positions.len(), 2 * 25 * 9);
        assert!(banks.triangles.len() > water.triangles.len());
        assert!(banks
            .positions
            .iter()
            .all(|point| point.iter().all(|v| v.is_finite())));
    }

    #[test]
    fn botanical_anatomy_keeps_bark_and_foliage_distinct() {
        assert_eq!(
            material_family_for("rc:asset/flora/eucalypt-gum/trunk").id,
            "weathered-timber"
        );
        assert_eq!(
            material_family_for("rc:asset/flora/eucalypt-gum/blue-grey-leaves-0").id,
            "living-foliage"
        );
    }

    #[test]
    fn settlement_parts_use_their_authored_surface_family() {
        assert_eq!(
            material_family_for("rc:asset/solar-cells").id,
            "solar-glass"
        );
        assert_eq!(
            material_family_for("rc:asset/solar-array").id,
            "galvanised-metal"
        );
        assert_eq!(
            material_family_for("rc:asset/workshop-window-glazing").id,
            "framed-glass"
        );
        assert_eq!(
            material_family_for("rc:asset/workshop-window-frame").id,
            "galvanised-metal"
        );
        assert_eq!(
            material_family_for("rc:asset/workshop-roof").id,
            "terracotta"
        );
        assert_eq!(
            material_family_for("rc:asset/camp-shade-panels").id,
            "shade-canvas"
        );
    }
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
    scene.push(parametric(
        "rc:asset/saltwind-earth-skirt",
        [0.55, 0.55, 0.35, 1.0],
        ParametricRecipe::TerrainApron {
            center_x: 16.0,
            center_z: 0.0,
        },
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
    const CANAL_CONTROL: &[[f32; 2]] = &[[8.05, -7.0], [8.72, -2.15], [7.28, 2.20], [8.02, 7.0]];
    scene.push(parametric(
        "rc:asset/channel-banks",
        [0.62, 0.64, 0.43, 1.0],
        ParametricRecipe::WaterwayBanks {
            control: CANAL_CONTROL,
            water_width: 2.15,
            bank_width: 1.05,
            elevation: if high_tide { 0.08 } else { 0.01 },
            segments: 64,
        },
    ));
    scene.push(water_surface(
        "rc:asset/channel",
        if high_tide {
            [0.18, 0.48, 0.66, 1.0]
        } else {
            [0.23, 0.67, 0.75, 1.0]
        },
        ParametricRecipe::Waterway {
            control: CANAL_CONTROL,
            width: 2.15,
            elevation: if high_tide { 0.08 } else { 0.01 },
            segments: 64,
            across: 8,
        },
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
            width: 2.35,
            elevation: 0.049,
        },
    ));
    scene.push(parametric(
        "rc:asset/saltwind-road-orchard-spur",
        [0.88, 0.71, 0.51, 1.0],
        ParametricRecipe::Ribbon {
            control: &[[16.0, -4.20], [15.65, -2.6], [16.25, -0.55], [16.0, 0.48]],
            width: 1.65,
            elevation: 0.050,
        },
    ));
    for (id, control, width) in [
        (
            "rc:asset/saltwind-path-north-meadow",
            &[[11.5_f32, -4.6], [12.8, -3.7], [13.4, -1.7], [15.2, -0.7]][..],
            0.72,
        ),
        (
            "rc:asset/saltwind-path-crossing",
            &[[10.2_f32, 1.2], [12.5, 1.8], [14.6, 2.1], [16.0, 0.9]][..],
            0.66,
        ),
        (
            "rc:asset/saltwind-path-orchard",
            &[[15.3_f32, 2.1], [16.8, 2.8], [18.5, 3.6], [20.4, 4.6]][..],
            0.76,
        ),
    ] {
        scene.push(parametric(
            id,
            [0.91, 0.79, 0.59, 1.0],
            ParametricRecipe::Ribbon {
                control,
                width,
                elevation: 0.052,
            },
        ));
    }
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
        vec![block([11.8, 0.54, -2.6], [1.7, 1.08, 1.15])],
    ));
    scene.push(parametric(
        "rc:asset/saltwind-market-awning",
        [0.43, 0.71, 0.76, 1.0],
        ParametricRecipe::Barrel {
            center: [11.8, 1.10, -2.6],
            radius: 0.76,
            length: 2.2,
            segments: 16,
        },
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
        scene.push(parametric(
            base_id,
            [0.51, 0.53, 0.36, 1.0],
            ParametricRecipe::TerrainApron {
                center_x: x,
                center_z: -14.0,
            },
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
    const CREEK_CONTROL: &[[f32; 2]] = &[[8.02, -21.0], [8.48, -16.4], [7.55, -11.3], [8.08, -7.0]];
    scene.push(parametric(
        "rc:asset/highland-creek-banks",
        [0.57, 0.67, 0.46, 1.0],
        ParametricRecipe::WaterwayBanks {
            control: CREEK_CONTROL,
            water_width: 1.82,
            bank_width: 1.45,
            elevation: 0.02,
            segments: 56,
        },
    ));
    scene.push(water_surface(
        "rc:asset/highland-creek",
        [0.28, 0.63, 0.73, 1.0],
        ParametricRecipe::Waterway {
            control: CREEK_CONTROL,
            width: 1.82,
            elevation: 0.02,
            segments: 56,
            across: 8,
        },
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
