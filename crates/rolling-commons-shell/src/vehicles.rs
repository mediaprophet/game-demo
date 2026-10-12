//! Modular Australian touring rigs assembled from independent Qualia `.10d` assets.
//!
//! Cosmetic rig selection changes presentation only. The campaign world and
//! its replayable actions remain the source of game-state truth.

use qualia_core_db::render::scene_primitives::Primitive;

use crate::asset_catalog::{material_family_for, AssetRecipe, ParametricRecipe, GALVANISED_METAL};

const VEHICLE_X: f32 = 2.35;
const VEHICLE_Z: f32 = 4.65;

/// Each station records Z, half-width, sill, roof height above sill and
/// corner radius. This authors coachwork rather than stretching a revolved
/// capsule: bonnet, cab, shoulder and rear overhang have distinct profiles.
pub(crate) fn coachwork(x: f32, z: f32, stations: &[[f32; 5]]) -> ParametricRecipe {
    let sections = stations.iter().map(|s| {
        let [station, width, sill, height, radius] = *s;
        let r = radius.min(width * 0.45).min(height * 0.45);
        let mut ring = Vec::with_capacity(20);
        for (cx, cy, start) in [
            (width - r, sill + r, -90.0_f32),
            (width - r, sill + height - r, 0.0),
            (-width + r, sill + height - r, 90.0),
            (-width + r, sill + r, 180.0),
        ] {
            for step in 0..5 {
                let angle = (start + step as f32 * 22.5).to_radians();
                ring.push([x + cx + r * angle.cos(), cy + r * angle.sin(), z + station]);
            }
        }
        ring
    }).collect();
    ParametricRecipe::LoftedBody { sections }
}

fn box_part(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Box { center, size }
}

fn asset(id: String, color: [f32; 4], parts: Vec<Primitive>) -> AssetRecipe {
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts,
        parametric: None,
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn mesh_part(id: String, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(recipe),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn wheels(prefix: &str, x: f32, axle_z: &[f32], radius: f32) -> Vec<AssetRecipe> {
    let mut out = Vec::with_capacity(axle_z.len() * 2);
    for (axle, z) in axle_z.iter().enumerate() {
        for (side, offset) in [("left", -x), ("right", x)] {
            out.push(mesh_part(
                format!("rc:asset/rig/{prefix}/wheel-{axle}-{side}"),
                [0.055, 0.065, 0.063, 1.0],
                ParametricRecipe::WheelX {
                    center: [VEHICLE_X + offset, 0.29, *z],
                    radius,
                    width: 0.19,
                    segments: 32,
                },
            ));
            let mut rim = mesh_part(
                format!("rc:asset/rig/{prefix}/rim-{axle}-{side}"),
                [0.70, 0.72, 0.68, 1.0],
                ParametricRecipe::WheelX {
                    center: [VEHICLE_X + offset + offset.signum() * 0.102, 0.29, *z],
                    radius: radius * 0.51, width: 0.025, segments: 24,
                },
            );
            rim.material_family = GALVANISED_METAL;
            out.push(rim);
        }
    }
    out
}

fn four_wheel_drive(prefix: &str, color: [f32; 4], cab_color: [f32; 4]) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z;
    let body = mesh_part(
        format!("rc:asset/rig/{prefix}/body"),
        color,
        coachwork(x, z, &[
            [-1.28, 0.74, 0.46, 0.36, 0.08],
            [-1.08, 0.83, 0.42, 0.45, 0.10],
            [-0.52, 0.84, 0.42, 0.52, 0.10],
            [0.66, 0.84, 0.42, 0.55, 0.10],
            [1.23, 0.79, 0.46, 0.50, 0.08],
        ]),
    );
    let cabin = mesh_part(
        format!("rc:asset/rig/{prefix}/cabin"),
        cab_color,
        coachwork(x, z, &[
            [-0.49, 0.68, 0.91, 0.65, 0.065],
            [-0.34, 0.71, 0.91, 0.76, 0.075],
            [0.60, 0.71, 0.91, 0.76, 0.075],
            [0.73, 0.67, 0.91, 0.65, 0.065],
        ]),
    );
    let chassis = asset(
        format!("rc:asset/rig/{prefix}/chassis"),
        [0.16, 0.17, 0.16, 1.0],
        vec![
            box_part([x, 0.30, z], [1.46, 0.17, 2.70]),
            box_part([x, 0.22, z - 0.72], [1.78, 0.12, 0.13]),
            box_part([x, 0.22, z + 0.72], [1.78, 0.12, 0.13]),
        ],
    );
    let glass = asset(
        format!("rc:asset/rig/{prefix}/glazing"),
        [0.19, 0.34, 0.40, 1.0],
        vec![
            box_part([x, 1.31, z - 0.46], [1.02, 0.47, 0.035]),
            box_part([x, 1.30, z + 0.69], [0.91, 0.43, 0.035]),
            box_part([x - 0.716, 1.34, z - 0.06], [0.028, 0.40, 0.51]),
            box_part([x + 0.716, 1.34, z - 0.06], [0.028, 0.40, 0.51]),
            box_part([x - 0.716, 1.34, z + 0.45], [0.028, 0.40, 0.44]),
            box_part([x + 0.716, 1.34, z + 0.45], [0.028, 0.40, 0.44]),
        ],
    );
    let fittings = asset(
        format!("rc:asset/rig/{prefix}/fittings"),
        [0.73, 0.70, 0.57, 1.0],
        vec![
            box_part([x, 0.48, z - 1.28], [1.78, 0.17, 0.16]),
            box_part([x, 0.47, z + 1.28], [1.72, 0.16, 0.15]),
            box_part([x, 0.83, z - 1.263], [0.26, 0.16, 0.035]),
            box_part([x - 0.52, 0.80, z - 1.266], [0.15, 0.12, 0.035]),
            box_part([x + 0.52, 0.80, z - 1.266], [0.15, 0.12, 0.035]),
            box_part([x - 0.82, 0.94, z + 0.10], [0.17, 0.055, 0.13]),
            box_part([x + 0.82, 0.94, z + 0.10], [0.17, 0.055, 0.13]),
        ],
    );
    let mut out = vec![body, cabin, chassis, glass, fittings];
    out.extend(wheels(prefix, 0.86, &[z - 0.77, z + 0.78], 0.34));
    out
}

fn caravan(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z + 3.39;
    let body = mesh_part(
        format!("rc:asset/rig/{prefix}/caravan-body"),
        [0.88, 0.85, 0.74, 1.0],
        coachwork(x, z, &[
            [-1.58, 0.77, 0.42, 1.10, 0.12],
            [-1.32, 0.90, 0.36, 1.32, 0.13],
            [1.28, 0.90, 0.36, 1.32, 0.13],
            [1.58, 0.80, 0.42, 1.16, 0.12],
        ]),
    );
    let running_gear = asset(
        format!("rc:asset/rig/{prefix}/caravan-running-gear"),
        [0.18, 0.19, 0.17, 1.0],
        vec![
            box_part([x, 0.34, z], [1.72, 0.16, 3.35]),
            box_part([x, 0.36, VEHICLE_Z + 1.77], [0.11, 0.11, 0.82]),
        ],
    );
    let glazing = asset(
        format!("rc:asset/rig/{prefix}/caravan-glazing"),
        [0.25, 0.42, 0.47, 1.0],
        vec![
            box_part([x - 0.915, 1.26, z - 0.80], [0.035, 0.40, 0.69]),
            box_part([x - 0.915, 1.26, z + 0.60], [0.035, 0.40, 0.78]),
            box_part([x + 0.915, 1.26, z - 0.80], [0.035, 0.40, 0.69]),
            box_part([x + 0.915, 1.26, z + 0.60], [0.035, 0.40, 0.78]),
            box_part([x, 1.28, z - 1.40], [0.87, 0.41, 0.035]),
        ],
    );
    let details = asset(
        format!("rc:asset/rig/{prefix}/caravan-details"),
        [0.77, 0.49, 0.25, 1.0],
        vec![
            box_part([x + 0.86, 0.88, z + 0.11], [0.04, 0.84, 0.085]),
            box_part([x, 0.61, z + 1.56], [1.36, 0.09, 0.05]),
            box_part([x, 0.78, z - 0.96], [1.76, 0.07, 0.035]),
        ],
    );
    let drawbar = asset(
        format!("rc:asset/rig/{prefix}/drawbar"),
        [0.32, 0.34, 0.31, 1.0],
        vec![
            box_part([x, 0.39, VEHICLE_Z + 1.74], [0.11, 0.10, 0.76]),
            box_part([x, 0.35, VEHICLE_Z + 1.36], [0.20, 0.12, 0.13]),
        ],
    );
    let mut out = vec![body, running_gear, glazing, details, drawbar];
    out.extend(wheels(&format!("{prefix}/caravan"), 0.93, &[z - 0.13, z + 0.42], 0.31));
    out
}

fn camper_trailer(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z + 2.65;
    vec![
        asset(
            format!("rc:asset/rig/{prefix}/trailer-body"),
            [0.65, 0.61, 0.47, 1.0],
            vec![
                box_part([x, 0.62, z], [1.52, 0.72, 1.80]),
                box_part([x, 1.06, z + 0.03], [1.40, 0.20, 1.64]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/trailer-chassis"),
            [0.18, 0.19, 0.17, 1.0],
            vec![
                box_part([x, 0.29, z], [1.40, 0.12, 2.03]),
                box_part([x, 0.30, z - 1.08], [0.10, 0.10, 0.77]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/trailer-canvas"),
            [0.83, 0.72, 0.52, 1.0],
            vec![
                box_part([x, 1.30, z], [1.18, 0.31, 1.30]),
                box_part([x - 0.60, 1.28, z], [0.075, 0.28, 1.40]),
                box_part([x + 0.60, 1.28, z], [0.075, 0.28, 1.40]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/trailer-drawbar"),
            [0.35, 0.36, 0.33, 1.0],
            vec![box_part([x, 0.36, VEHICLE_Z + 1.83], [0.10, 0.08, 0.56])],
        ),
    ]
}

fn sedan(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z;
    let mut out = vec![
        mesh_part(
            format!("rc:asset/rig/{prefix}/body"),
            [0.56, 0.65, 0.65, 1.0],
            coachwork(x, z, &[
                [-1.43, 0.72, 0.44, 0.31, 0.09],
                [-1.04, 0.83, 0.40, 0.38, 0.10],
                [-0.56, 0.84, 0.40, 0.43, 0.10],
                [0.70, 0.84, 0.40, 0.43, 0.10],
                [1.40, 0.73, 0.44, 0.30, 0.09],
            ]),
        ),
        mesh_part(
            format!("rc:asset/rig/{prefix}/cabin"),
            [0.67, 0.73, 0.70, 1.0],
            coachwork(x, z, &[
                [-0.63, 0.65, 0.80, 0.23, 0.06],
                [-0.33, 0.67, 0.80, 0.53, 0.07],
                [0.39, 0.67, 0.80, 0.53, 0.07],
                [0.75, 0.63, 0.80, 0.21, 0.06],
            ]),
        ),
        asset(
            format!("rc:asset/rig/{prefix}/chassis"),
            [0.16, 0.17, 0.16, 1.0],
            vec![
                box_part([x, 0.31, z], [1.47, 0.16, 2.94]),
                box_part([x, 0.23, z - 0.84], [1.77, 0.12, 0.13]),
                box_part([x, 0.23, z + 0.84], [1.70, 0.12, 0.13]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/glazing"),
            [0.20, 0.39, 0.43, 1.0],
            vec![
                box_part([x, 1.03, z - 0.60], [1.08, 0.30, 0.035]),
                box_part([x, 1.03, z + 0.72], [1.02, 0.30, 0.035]),
                box_part([x - 0.68, 1.06, z - 0.04], [0.03, 0.30, 0.51]),
                box_part([x + 0.68, 1.06, z - 0.04], [0.03, 0.30, 0.51]),
                box_part([x - 0.68, 1.06, z + 0.49], [0.03, 0.30, 0.42]),
                box_part([x + 0.68, 1.06, z + 0.49], [0.03, 0.30, 0.42]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/fittings"),
            [0.72, 0.70, 0.59, 1.0],
            vec![
                box_part([x, 0.44, z - 1.45], [1.74, 0.13, 0.13]),
                box_part([x, 0.43, z + 1.45], [1.68, 0.13, 0.12]),
                box_part([x, 0.75, z - 1.40], [0.27, 0.12, 0.035]),
            ],
        ),
    ];
    out.extend(wheels(prefix, 0.86, &[z - 0.84, z + 0.82], 0.30));
    out
}

fn roof_tent(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z;
    vec![
        asset(
            format!("rc:asset/rig/{prefix}/roof-rack"),
            [0.20, 0.22, 0.21, 1.0],
            vec![
                box_part([x, 1.77, z - 0.38], [1.35, 0.055, 0.10]),
                box_part([x, 1.77, z + 0.43], [1.35, 0.055, 0.10]),
                box_part([x - 0.58, 1.80, z + 0.03], [0.07, 0.06, 0.92]),
                box_part([x + 0.58, 1.80, z + 0.03], [0.07, 0.06, 0.92]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/roof-tent"),
            [0.67, 0.51, 0.34, 1.0],
            vec![
                box_part([x, 1.98, z + 0.02], [1.24, 0.31, 1.13]),
                Primitive::Roof {
                    center: [x, 2.13, z + 0.02],
                    size: [1.14, 0.18, 1.03],
                },
                box_part([x, 1.83, z - 0.57], [1.33, 0.06, 0.30]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/awning"),
            [0.82, 0.74, 0.56, 1.0],
            vec![box_part([x, 1.81, z + 0.81], [1.18, 0.08, 0.45])],
        ),
    ]
}

fn campervan(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z + 0.12;
    let mut out = vec![
        mesh_part(
            format!("rc:asset/rig/{prefix}/body"),
            [0.88, 0.84, 0.73, 1.0],
            coachwork(x, z, &[
                [-1.78, 0.78, 0.35, 1.12, 0.12],
                [-1.50, 0.91, 0.30, 1.34, 0.14],
                [-0.90, 0.92, 0.30, 1.40, 0.14],
                [1.49, 0.92, 0.30, 1.40, 0.14],
                [1.78, 0.81, 0.35, 1.26, 0.12],
            ]),
        ),
        asset(
            format!("rc:asset/rig/{prefix}/glazing"),
            [0.19, 0.36, 0.43, 1.0],
            vec![
                box_part([x, 1.10, z - 1.70], [1.16, 0.61, 0.035]),
                box_part([x - 0.933, 1.25, z - 0.72], [0.035, 0.52, 0.68]),
                box_part([x + 0.933, 1.25, z - 0.72], [0.035, 0.52, 0.68]),
                box_part([x - 0.933, 1.25, z + 0.40], [0.035, 0.52, 0.72]),
                box_part([x + 0.933, 1.25, z + 0.40], [0.035, 0.52, 0.72]),
                box_part([x, 1.19, z + 1.70], [1.04, 0.46, 0.035]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/chassis"),
            [0.17, 0.18, 0.17, 1.0],
            vec![box_part([x, 0.31, z], [1.65, 0.18, 3.54])],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/fittings"),
            [0.75, 0.47, 0.23, 1.0],
            vec![
                box_part([x, 0.48, z - 1.85], [1.82, 0.17, 0.14]),
                box_part([x, 0.48, z + 1.80], [1.76, 0.16, 0.14]),
                box_part([x, 0.72, z - 1.79], [0.24, 0.15, 0.045]),
                box_part([x - 0.82, 0.96, z - 0.14], [0.22, 0.06, 0.13]),
                box_part([x + 0.82, 0.96, z - 0.14], [0.22, 0.06, 0.13]),
            ],
        ),
    ];
    out.extend(wheels(prefix, 0.94, &[z - 1.23, z + 1.12], 0.37));
    out
}

/// Return one complete road-ready combination: campervan, tow vehicle with
/// caravan, car with camper trailer, or 4WD with roof-top tent.
pub fn rig_assets(rig: u8) -> Vec<AssetRecipe> {
    let mut assets = match rig.min(3) {
        0 => campervan("campervan"),
        1 => {
            let mut out = four_wheel_drive(
                "4wd-caravan",
                [0.73, 0.66, 0.49, 1.0],
                [0.77, 0.72, 0.58, 1.0],
            );
            out.extend(caravan("4wd-caravan"));
            out
        }
        2 => {
            let mut out = sedan("car-trailer");
            out.extend(camper_trailer("car-trailer"));
            out.extend(wheels("car-trailer/camper-trailer",0.78,&[VEHICLE_Z+2.65],0.31));
            out
        }
        _ => {
            let mut out = four_wheel_drive(
                "4wd-roof-tent",
                [0.65, 0.70, 0.52, 1.0],
                [0.75, 0.77, 0.63, 1.0],
            );
            out.extend(roof_tent("4wd-roof-tent"));
            out
        }
    };
    crate::vehicle_cabins::refine(rig,&mut assets);
    crate::vehicle_exterior::refine(rig,&mut assets);
    let mut frames = Vec::new();
    for glazing in assets.iter().filter(|asset| asset.id.ends_with("glazing")) {
        let mut parts = Vec::new();
        for pane in &glazing.parts {
            let Primitive::Box { center, size } = pane else { continue };
            let axis = if size[0] < size[2] { 0 } else { 2 };
            let horizontal = if axis == 0 { 2 } else { 0 };
            let thickness = 0.036;
            for sign in [-1.0, 1.0] {
                let mut at = *center;
                at[1] += sign * (size[1] * 0.5 + thickness * 0.5);
                let mut extent = *size;
                extent[axis] = 0.048;
                extent[1] = thickness;
                extent[horizontal] += thickness * 2.0;
                parts.push(box_part(at, extent));
                let mut at = *center;
                at[horizontal] += sign * (size[horizontal] * 0.5 + thickness * 0.5);
                let mut extent = *size;
                extent[axis] = 0.048;
                extent[horizontal] = thickness;
                parts.push(box_part(at, extent));
            }
        }
        if !parts.is_empty() {
            frames.push(asset(glazing.id.replace("glazing", "window-frame"),
                [0.26, 0.29, 0.27, 1.0], parts));
        }
    }
    assets.extend(frames);
    assets
}

pub const RIG_LABELS: [&str; 4] = [
    "Campervan",
    "4WD + caravan",
    "Car + camper trailer",
    "4WD + roof-top tent",
];
