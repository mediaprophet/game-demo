//! Hand-held work objects authored in the resident's shared bind coordinates.
use crate::asset_catalog::{
    AssetRecipe, EllipsoidInstance, LimbSegment, MaterialFamily, PanelPart, ParametricRecipe,
    CLOTH_AND_SKIN, GALVANISED_METAL, PAINTED_METAL, WEATHERED_TIMBER,
};

fn at(hand: [f32; 3], delta: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|a| hand[a] + delta[a])
}
fn model(
    resident: &str,
    side: &str,
    name: &str,
    color: [f32; 4],
    family: MaterialFamily,
    spec: ParametricRecipe,
) -> AssetRecipe {
    AssetRecipe {
        id: format!("rc:asset/resident-{resident}-attachment-{side}-{name}"),
        parts: Vec::new(),
        parametric: Some(spec),
        color,
        material_family: family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}
fn panel(offset: [f32; 3], size: [f32; 3]) -> PanelPart {
    PanelPart {
        offset,
        size,
        pitch_deg: 0.0,
        roll_deg: 0.0,
        yaw_deg: 0.0,
    }
}
fn ring(center: [f32; 3], radius: f32, tube: f32) -> ParametricRecipe {
    // Vertical circular handles use authored stem endpoints rather than a
    // rectangular approximation. Native tapered revolutions build the tube.
    let limbs = (0..24)
        .map(|i| {
            let a = i as f32 * std::f32::consts::TAU / 24.0;
            let b = (i + 1) as f32 * std::f32::consts::TAU / 24.0;
            LimbSegment {
                start: [0.0, a.cos() * radius, a.sin() * radius],
                end: [0.0, b.cos() * radius, b.sin() * radius],
                start_radius: tube,
                end_radius: tube,
            }
        })
        .collect();
    ParametricRecipe::LimbCluster {
        center,
        limbs,
        radial_segments: 8,
    }
}
fn crate_body(center: [f32; 3]) -> ParametricRecipe {
    let mut parts = vec![panel([0.0, -0.18, 0.0], [0.45, 0.035, 0.30])];
    for x in [-0.21, 0.21] {
        for z in [-0.135, 0.135] {
            parts.push(panel([x, -0.07, z], [0.028, 0.22, 0.028]));
        }
    }
    for y in [-0.135, -0.06, 0.015] {
        for z in [-0.145, 0.145] {
            parts.push(panel([0.0, y, z], [0.45, 0.045, 0.018]));
        }
        for x in [-0.22, 0.22] {
            parts.push(panel([x, y, 0.0], [0.018, 0.045, 0.30]));
        }
    }
    ParametricRecipe::PanelGroup { center, parts }
}
fn fruit(center: [f32; 3]) -> ParametricRecipe {
    let instances = [
        [-0.13, -0.055, -0.07],
        [0.0, -0.065, -0.07],
        [0.13, -0.055, -0.07],
        [-0.08, -0.05, 0.065],
        [0.07, -0.04, 0.065],
    ]
    .into_iter()
    .map(|p| EllipsoidInstance {
        center: at(center, p),
        radii: [0.055, 0.051, 0.054],
        pitch_deg: 0.0,
        yaw_deg: 0.0,
        roll_deg: 0.0,
    })
    .collect();
    ParametricRecipe::EllipsoidCluster {
        instances,
        latitude: 8,
        longitude: 12,
    }
}

pub fn assets(resident: &str, left: [f32; 3], right: [f32; 3]) -> Vec<AssetRecipe> {
    let steel = [0.62, 0.68, 0.67, 1.0];
    let wood = [0.55, 0.35, 0.18, 1.0];
    match resident {
        "ada" => {
            let mut sections = Vec::new();
            for (z, width, height) in [
                (0.14, 0.025, 0.008),
                (0.22, 0.070, 0.018),
                (0.31, 0.057, 0.020),
                (0.38, 0.007, 0.004),
            ] {
                sections.push(vec![
                    at(right, [-width, -height, z]),
                    at(right, [width, -height, z]),
                    at(right, [width, height, z]),
                    at(right, [-width, height, z]),
                ]);
            }
            vec![
                model(
                    resident,
                    "right",
                    "trowel-wood-handle",
                    wood,
                    WEATHERED_TIMBER,
                    ParametricRecipe::Stem {
                        start: at(right, [0.0, 0.0, -0.05]),
                        end: at(right, [0.0, 0.0, 0.14]),
                        radius: 0.025,
                        segments: 16,
                    },
                ),
                model(
                    resident,
                    "right",
                    "trowel-steel-blade",
                    steel,
                    GALVANISED_METAL,
                    ParametricRecipe::LoftedBody { sections },
                ),
            ]
        }
        "bo" => vec![
            model(
                resident,
                "right",
                "spanner-steel-shaft",
                steel,
                GALVANISED_METAL,
                ParametricRecipe::Stem {
                    start: at(right, [0.0, 0.0, -0.08]),
                    end: at(right, [0.0, 0.0, 0.20]),
                    radius: 0.017,
                    segments: 12,
                },
            ),
            model(
                resident,
                "right",
                "spanner-steel-head",
                steel,
                GALVANISED_METAL,
                ParametricRecipe::Torus {
                    center: at(right, [0.0, 0.0, 0.24]),
                    major: 0.043,
                    minor: 0.013,
                    segments: 24,
                    tube_segments: 8,
                },
            ),
        ],
        "cam" => {
            let center = at(left, [-0.10, -0.12, 0.02]);
            vec![
                model(
                    resident,
                    "left",
                    "harvest-timber-basket",
                    wood,
                    WEATHERED_TIMBER,
                    crate_body(center),
                ),
                model(
                    resident,
                    "left",
                    "basket-metal-handle",
                    steel,
                    GALVANISED_METAL,
                    ring(at(center, [0.0, 0.03, 0.0]), 0.13, 0.012),
                ),
                model(
                    resident,
                    "left",
                    "harvest-fruit",
                    [0.73, 0.30, 0.11, 1.0],
                    CLOTH_AND_SKIN,
                    fruit(center),
                ),
            ]
        }
        "dev" => vec![
            model(
                resident,
                "left",
                "clipboard-timber-board",
                [0.39, 0.25, 0.13, 1.0],
                WEATHERED_TIMBER,
                ParametricRecipe::TiltedBox {
                    center: at(left, [0.0, 0.01, 0.10]),
                    size: [0.21, 0.018, 0.30],
                    pitch_deg: 14.0,
                    roll_deg: 0.0,
                    yaw_deg: 0.0,
                },
            ),
            model(
                resident,
                "left",
                "clipboard-paper",
                [0.94, 0.90, 0.76, 1.0],
                CLOTH_AND_SKIN,
                ParametricRecipe::TiltedBox {
                    center: at(left, [0.0, 0.026, 0.10]),
                    size: [0.18, 0.003, 0.26],
                    pitch_deg: 14.0,
                    roll_deg: 0.0,
                    yaw_deg: 0.0,
                },
            ),
            model(
                resident,
                "left",
                "clipboard-metal-clip",
                steel,
                GALVANISED_METAL,
                ParametricRecipe::TiltedBox {
                    center: at(left, [0.0, 0.055, -0.015]),
                    size: [0.065, 0.012, 0.035],
                    pitch_deg: 14.0,
                    roll_deg: 0.0,
                    yaw_deg: 0.0,
                },
            ),
        ],
        "eve" => {
            const PROFILE: &[[f32; 2]] = &[
                [0.0, -0.11],
                [0.085, -0.11],
                [0.11, -0.08],
                [0.11, 0.075],
                [0.095, 0.10],
                [0.084, 0.10],
                [0.091, 0.065],
                [0.085, -0.085],
                [0.0, -0.085],
            ];
            let center = at(right, [0.0, -0.16, 0.06]);
            vec![
                model(
                    resident,
                    "right",
                    "watering-can-metal-body",
                    [0.17, 0.39, 0.42, 1.0],
                    PAINTED_METAL,
                    ParametricRecipe::Revolve {
                        center,
                        profile: PROFILE,
                        segments: 28,
                    },
                ),
                model(
                    resident,
                    "right",
                    "watering-can-metal-handle",
                    steel,
                    GALVANISED_METAL,
                    ring(at(center, [0.0, 0.10, 0.0]), 0.09, 0.013),
                ),
                model(
                    resident,
                    "right",
                    "watering-can-metal-spout",
                    steel,
                    GALVANISED_METAL,
                    ParametricRecipe::LimbCluster {
                        center,
                        limbs: vec![LimbSegment {
                            start: [0.0, -0.04, 0.08],
                            end: [0.0, 0.04, 0.32],
                            start_radius: 0.030,
                            end_radius: 0.015,
                        }],
                        radial_segments: 12,
                    },
                ),
                model(
                    resident,
                    "right",
                    "watering-can-metal-rose",
                    steel,
                    GALVANISED_METAL,
                    ParametricRecipe::Ellipsoid {
                        center: at(center, [0.0, 0.05, 0.34]),
                        radii: [0.042, 0.025, 0.034],
                        latitude: 8,
                        longitude: 12,
                    },
                ),
            ]
        }
        "finn" => {
            let center = at(right, [-0.23, 0.0, 0.01]);
            vec![
                model(
                    resident,
                    "right",
                    "produce-timber-crate",
                    wood,
                    WEATHERED_TIMBER,
                    crate_body(center),
                ),
                model(
                    resident,
                    "right",
                    "crate-produce",
                    [0.55, 0.67, 0.17, 1.0],
                    CLOTH_AND_SKIN,
                    fruit(center),
                ),
            ]
        }
        _ => Vec::new(),
    }
}
