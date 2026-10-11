//! Modular Australian touring rigs assembled from independent Qualia `.10d` assets.
//!
//! Cosmetic rig selection changes presentation only. The campaign world and
//! its replayable actions remain the source of game-state truth.

use qualia_core_db::render::scene_primitives::Primitive;

use crate::asset_catalog::{AssetRecipe, ParametricRecipe};

const VEHICLE_X: f32 = 2.35;
const VEHICLE_Z: f32 = 4.65;

// A long, weather-sealed touring body. Qualia revolves this profile into a
// rounded shell so the campervan and caravan catch broad daylight gradients
// instead of reading as stacks of hard-edged boxes.
const TOURING_BODY_PROFILE: &[[f32; 2]] = &[
    [0.0, 0.0],
    [0.72, 0.0],
    [0.91, 0.035],
    [0.98, 0.12],
    [1.0, 0.24],
    [1.0, 0.78],
    [0.97, 0.90],
    [0.88, 0.97],
    [0.70, 1.0],
    [0.0, 1.0],
];

fn box_part(center: [f32; 3], size: [f32; 3]) -> Primitive {
    Primitive::Box { center, size }
}

fn asset(id: String, color: [f32; 4], parts: Vec<Primitive>) -> AssetRecipe {
    AssetRecipe {
        id,
        parts,
        parametric: None,
        color,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn mesh_part(id: String, color: [f32; 4], recipe: ParametricRecipe) -> AssetRecipe {
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(recipe),
        color,
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
        }
    }
    out
}

fn four_wheel_drive(prefix: &str, color: [f32; 4], cab_color: [f32; 4]) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z;
    let body = asset(
        format!("rc:asset/rig/{prefix}/body"),
        color,
        vec![
            box_part([x, 0.66, z], [1.64, 0.63, 2.48]),
            box_part([x, 1.03, z - 0.77], [1.52, 0.29, 0.83]),
            box_part([x, 0.91, z + 0.80], [1.48, 0.17, 0.69]),
        ],
    );
    let cabin = mesh_part(
        format!("rc:asset/rig/{prefix}/cabin"),
        cab_color,
        ParametricRecipe::Pod {
            center: [x, 0.90, z + 0.13],
            profile: TOURING_BODY_PROFILE,
            segments: 32,
            stretch: [0.70, 0.78, 0.58],
        },
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
            box_part([x - 0.69, 1.34, z - 0.06], [0.028, 0.40, 0.51]),
            box_part([x + 0.69, 1.34, z - 0.06], [0.028, 0.40, 0.51]),
            box_part([x - 0.69, 1.34, z + 0.45], [0.028, 0.40, 0.44]),
            box_part([x + 0.69, 1.34, z + 0.45], [0.028, 0.40, 0.44]),
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
        ParametricRecipe::Pod {
            center: [x, 0.31, z],
            profile: TOURING_BODY_PROFILE,
            segments: 32,
            stretch: [0.92, 1.42, 1.62],
        },
    );
    let running_gear = asset(
        format!("rc:asset/rig/{prefix}/caravan-running-gear"),
        [0.18, 0.19, 0.17, 1.0],
        vec![
            box_part([x, 0.34, z], [1.72, 0.16, 3.35]),
            box_part([x, 0.36, z - 1.86], [0.11, 0.11, 1.04]),
        ],
    );
    let glazing = asset(
        format!("rc:asset/rig/{prefix}/caravan-glazing"),
        [0.25, 0.42, 0.47, 1.0],
        vec![
            box_part([x - 0.82, 1.26, z - 0.80], [0.035, 0.40, 0.69]),
            box_part([x - 0.82, 1.26, z + 0.60], [0.035, 0.40, 0.78]),
            box_part([x + 0.82, 1.26, z - 0.80], [0.035, 0.40, 0.69]),
            box_part([x + 0.82, 1.26, z + 0.60], [0.035, 0.40, 0.78]),
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
            box_part([x, 0.39, z - 2.05], [0.11, 0.10, 0.76]),
            box_part([x, 0.35, z - 2.42], [0.20, 0.12, 0.13]),
        ],
    );
    let mut out = vec![body, running_gear, glazing, details, drawbar];
    out.extend(wheels(prefix, 0.93, &[z - 0.13, z + 0.42], 0.31));
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
            vec![box_part([x, 0.36, z - 1.20], [0.10, 0.08, 0.66])],
        ),
    ]
}

fn sedan(prefix: &str) -> Vec<AssetRecipe> {
    let x = VEHICLE_X;
    let z = VEHICLE_Z;
    let mut out = vec![
        asset(
            format!("rc:asset/rig/{prefix}/body"),
            [0.56, 0.65, 0.65, 1.0],
            vec![
                box_part([x, 0.59, z], [1.69, 0.53, 2.80]),
                box_part([x, 0.89, z - 0.81], [1.57, 0.23, 0.79]),
                box_part([x, 0.85, z + 0.93], [1.48, 0.20, 0.60]),
            ],
        ),
        asset(
            format!("rc:asset/rig/{prefix}/cabin"),
            [0.67, 0.73, 0.70, 1.0],
            vec![
                box_part([x, 1.02, z + 0.06], [1.34, 0.47, 1.30]),
                box_part([x, 1.27, z + 0.06], [1.10, 0.07, 1.08]),
            ],
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
            ParametricRecipe::Pod {
                center: [x, 0.30, z],
                profile: TOURING_BODY_PROFILE,
                segments: 32,
                stretch: [0.92, 1.30, 1.78],
            },
        ),
        asset(
            format!("rc:asset/rig/{prefix}/glazing"),
            [0.19, 0.36, 0.43, 1.0],
            vec![
                box_part([x, 1.10, z - 1.70], [1.16, 0.61, 0.035]),
                box_part([x - 0.90, 1.25, z - 0.72], [0.035, 0.52, 0.68]),
                box_part([x + 0.90, 1.25, z - 0.72], [0.035, 0.52, 0.68]),
                box_part([x - 0.90, 1.25, z + 0.40], [0.035, 0.52, 0.72]),
                box_part([x + 0.90, 1.25, z + 0.40], [0.035, 0.52, 0.72]),
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
                box_part([x, 0.48, z - 2.28], [1.82, 0.17, 0.14]),
                box_part([x, 0.48, z + 1.80], [1.76, 0.16, 0.14]),
                box_part([x, 0.72, z - 2.17], [0.24, 0.15, 0.045]),
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
    match rig.min(3) {
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
    }
}

pub const RIG_LABELS: [&str; 4] = [
    "Campervan",
    "4WD + caravan",
    "Car + camper trailer",
    "4WD + roof-top tent",
];
