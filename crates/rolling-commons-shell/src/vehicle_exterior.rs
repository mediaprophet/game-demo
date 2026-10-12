//! Original touring exterior details and native circular guard profiles.
use crate::asset_catalog::{
    AssetRecipe, LimbSegment, MaterialFamily, PanelPart, ParametricRecipe as Shape, FRAMED_GLASS,
    GALVANISED_METAL, PAINTED_METAL, RUBBER,
};
fn part(id: String, shape: Shape, color: [f32; 4], family: MaterialFamily) -> AssetRecipe {
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(shape),
        color,
        material_family: family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}
fn panel(at: [f32; 3], size: [f32; 3]) -> Shape {
    Shape::PanelGroup {
        center: [0.0; 3],
        parts: vec![PanelPart {
            offset: at,
            size,
            pitch_deg: 0.0,
            roll_deg: 0.0,
            yaw_deg: 0.0,
        }],
    }
}
fn axle_data(rig: u8) -> (Vec<f32>, f32, f32) {
    match rig {
        0 => (vec![4.65 + 0.12 - 1.23, 4.65 + 0.12 + 1.12], 0.37, 0.94),
        2 => (vec![4.65 - 0.84, 4.65 + 0.82], 0.30, 0.86),
        _ => (vec![4.65 - 0.77, 4.65 + 0.78], 0.34, 0.86),
    }
}
pub(crate) fn refine(rig: u8, models: &mut Vec<AssetRecipe>) {
    let rig = rig.min(3);
    let (axles, radius, wheel_x) = axle_data(rig);
    let stem = match rig {
        0 => "campervan",
        1 => "4wd-caravan",
        2 => "car-trailer",
        _ => "4wd-roof-tent",
    };
    let base = format!("rc:asset/rig/{stem}");
    let mut additions = Vec::new();
    // Narrow the lower central volume and retain a broad curved bonnet/trunk
    // cap. Native guard strips occupy the sides, leaving real tyre clearance.
    for model in models.iter_mut().filter(|model| {
        model.id.ends_with("/body")
            || model.id.ends_with("/rear-body")
            || model.id.ends_with("/caravan-body")
    }) {
        let Some(Shape::LoftedBody { sections }) = model.parametric.as_mut() else {
            continue;
        };
        if rig == 0 || model.id.ends_with("/caravan-body") {
            for ring in sections {
                for point in ring {
                    point[0] = 2.35 + (point[0] - 2.35) * 0.75;
                }
            }
            continue;
        }
        let original = sections.clone();
        let mut caps = Vec::new();
        for ring in &original {
            let top = ring.iter().map(|p| p[1]).fold(f32::NEG_INFINITY, f32::max);
            let width = ring.iter().map(|p| (p[0] - 2.35).abs()).fold(0.0, f32::max);
            caps.push([ring[0][2] - 4.65, width, top - 0.08, 0.08, 0.026]);
        }
        for ring in sections {
            for point in ring {
                point[0] = 2.35 + (point[0] - 2.35) * 0.71;
            }
        }
        additions.push(part(
            format!("{}-cap", model.id),
            crate::vehicles::coachwork(2.35, 4.65, &caps),
            model.color,
            PAINTED_METAL,
        ));
        for (segment, pair) in original.windows(2).enumerate() {
            let top = |ring: &Vec<[f32; 3]>| {
                ring.iter().map(|p| p[1]).fold(f32::NEG_INFINITY, f32::max) - 0.08
            };
            for side in [-1.0, 1.0] {
                additions.push(part(
                    format!(
                        "{}-guard-{segment}-{}",
                        model.id,
                        if side < 0.0 { "left" } else { "right" }
                    ),
                    Shape::WheelClearancePanel {
                        center_x: 2.35 + side * 0.82,
                        thickness: 0.055,
                        z: [pair[0][0][2], pair[1][0][2]],
                        bottom: 0.42,
                        top: [top(&pair[0]), top(&pair[1])],
                        wheel_y: 0.29,
                        clearance_radius: radius + 0.045,
                        axles: axles.clone(),
                        segments: 48,
                    },
                    model.color,
                    PAINTED_METAL,
                ));
            }
        }
    }
    for (index, axle) in axles.iter().enumerate() {
        for side in [-1.0, 1.0] {
            let mut limbs = Vec::new();
            let r = radius + 0.065;
            for step in 0..32 {
                let a = step as f32 * std::f32::consts::PI / 32.0;
                let b = (step + 1) as f32 * std::f32::consts::PI / 32.0;
                limbs.push(LimbSegment {
                    start: [0.0, r * a.sin(), r * a.cos()],
                    end: [0.0, r * b.sin(), r * b.cos()],
                    start_radius: 0.019,
                    end_radius: 0.019,
                });
            }
            additions.push(part(
                format!(
                    "{base}/wheel-arch-{index}-{}",
                    if side < 0.0 { "left" } else { "right" }
                ),
                Shape::LimbCluster {
                    center: [2.35 + side * (wheel_x - 0.06), 0.29, *axle],
                    limbs,
                    radial_segments: 8,
                },
                [0.15, 0.19, 0.17, 1.0],
                RUBBER,
            ));
            additions.push(part(
                format!(
                    "{base}/mud-flap-{index}-{}",
                    if side < 0.0 { "left" } else { "right" }
                ),
                panel(
                    [2.35 + side * wheel_x, 0.26, *axle + radius + 0.03],
                    [0.20, 0.25, 0.025],
                ),
                [0.13, 0.17, 0.15, 1.0],
                RUBBER,
            ));
        }
    }
    let front = if rig == 0 {
        4.65 + 0.12 - 1.80
    } else if rig == 2 {
        4.65 - 1.44
    } else {
        4.65 - 1.29
    };
    let back = if rig == 0 {
        4.65 + 0.12 + 1.80
    } else if rig == 2 {
        4.65 + 1.43
    } else {
        4.65 + 1.26
    };
    let lamp_y = if rig == 0 { 0.73 } else { 0.70 };
    let mirror_y = if rig == 0 {
        1.29
    } else if rig == 2 {
        1.03
    } else {
        1.29
    };
    // Retain the old fittings identity/bumpers while replacing its block lamp
    // stand-ins, so two different headlight designs do not occupy the nose.
    for model in models
        .iter_mut()
        .filter(|model| model.id == format!("{base}/fittings"))
    {
        model.parts.retain(|shape| match shape {
            qualia_core_db::render::scene_primitives::Primitive::Box { center, .. } => {
                !((center[2] - front).abs() < 0.16 && center[1] > 0.60 && center[1] < 0.91)
            }
            _ => true,
        });
    }
    let mut grille = Vec::new();
    for index in 0..9 {
        grille.push(PanelPart {
            offset: [2.35 - 0.20 + index as f32 * 0.05, lamp_y, front - 0.01],
            size: [0.021, 0.13, 0.022],
            pitch_deg: 0.0,
            roll_deg: 0.0,
            yaw_deg: 0.0,
        });
    }
    additions.push(part(
        format!("{base}/grille"),
        Shape::PanelGroup {
            center: [0.0; 3],
            parts: grille,
        },
        [0.16, 0.20, 0.18, 1.0],
        GALVANISED_METAL,
    ));
    for side in [-1.0, 1.0] {
        let label = if side < 0.0 { "left" } else { "right" };
        let mx = 2.35 + side * (wheel_x + 0.17);
        additions.push(part(
            format!("{base}/mirror-arm-{label}"),
            Shape::Stem {
                start: [2.35 + side * 0.78, mirror_y - 0.03, 4.65 - 0.27],
                end: [mx, mirror_y, 4.65 - 0.21],
                radius: 0.015,
                segments: 16,
            },
            [0.35, 0.39, 0.37, 1.0],
            GALVANISED_METAL,
        ));
        additions.push(part(
            format!("{base}/mirror-housing-{label}"),
            Shape::Ellipsoid {
                center: [mx, mirror_y, 4.65 - 0.20],
                radii: [0.075, 0.09, 0.038],
                latitude: 16,
                longitude: 24,
            },
            [0.14, 0.19, 0.18, 1.0],
            PAINTED_METAL,
        ));
        additions.push(part(
            format!("{base}/mirror-glass-{label}"),
            panel([mx, mirror_y, 4.65 - 0.157], [0.115, 0.13, 0.010]),
            [0.59, 0.71, 0.70, 1.0],
            FRAMED_GLASS,
        ));
        additions.push(part(
            format!("{base}/headlamp-{label}"),
            Shape::Ellipsoid {
                center: [2.35 + side * 0.53, lamp_y, front],
                radii: [0.13, 0.095, 0.026],
                latitude: 16,
                longitude: 24,
            },
            [0.91, 0.89, 0.69, 1.0],
            PAINTED_METAL,
        ));
        additions.push(part(
            format!("{base}/indicator-{label}"),
            panel(
                [2.35 + side * 0.70, lamp_y - 0.04, front],
                [0.10, 0.055, 0.018],
            ),
            [0.88, 0.46, 0.12, 1.0],
            PAINTED_METAL,
        ));
        additions.push(part(
            format!("{base}/tail-lamp-{label}"),
            panel([2.35 + side * 0.61, lamp_y, back], [0.13, 0.19, 0.018]),
            [0.68, 0.18, 0.12, 1.0],
            PAINTED_METAL,
        ));
        additions.push(part(
            format!("{base}/door-handle-{label}"),
            Shape::Stem {
                start: [
                    2.35 + side * 0.79,
                    if rig == 2 { 0.77 } else { 0.97 },
                    4.65 + 0.02,
                ],
                end: [
                    2.35 + side * 0.79,
                    if rig == 2 { 0.77 } else { 0.97 },
                    4.65 + 0.16,
                ],
                radius: 0.018,
                segments: 16,
            },
            [0.65, 0.68, 0.62, 1.0],
            GALVANISED_METAL,
        ));
    }
    models.extend(additions);
}
