//! Permanent camp caravan, using the same native coachwork family as the rigs.
//! Campaign shelter upgrades retain every existing asset identity and add an
//! attached canvas room; they do not replace or reset the player's rig state.

use crate::asset_catalog::{
    AssetRecipe, MaterialFamily, PanelPart, ParametricRecipe, FRAMED_GLASS,
    GALVANISED_METAL, PAINTED_METAL, RUBBER, SHADE_CANVAS,
};

const X: f32 = -3.05;
const Z: f32 = 2.7;

fn model(id: &str, color: [f32; 4], family: MaterialFamily, source: ParametricRecipe) -> AssetRecipe {
    AssetRecipe {
        id: format!("rc:asset/{id}"), parts: Vec::new(), parametric: Some(source),
        color, material_family: family, vibe_source: None, render_role: "opaque".into(),
    }
}

fn part(center: [f32; 3], size: [f32; 3]) -> PanelPart {
    PanelPart { offset: center, size, pitch_deg: 0.0, roll_deg: 0.0, yaw_deg: 0.0 }
}

fn panels(id: &str, color: [f32; 4], family: MaterialFamily, parts: Vec<PanelPart>) -> AssetRecipe {
    model(id, color, family, ParametricRecipe::PanelGroup { center: [0.0; 3], parts })
}

pub fn assets(upgrades: u32) -> Vec<AssetRecipe> {
    let half_length = 1.85 + upgrades.min(4) as f32 * 0.07;
    let coachwork = crate::vehicles::coachwork(0.0, 0.0, &[
        [-half_length, 0.69, 0.48, 1.87, 0.18],
        [-half_length + 0.24, 0.84, 0.46, 2.10, 0.16],
        [-half_length + 0.55, 0.86, 0.46, 2.14, 0.14],
        [half_length - 0.50, 0.86, 0.46, 2.14, 0.14],
        [half_length - 0.15, 0.83, 0.48, 2.03, 0.15],
        [half_length, 0.71, 0.52, 1.77, 0.18],
    ]);
    let mut out = vec![model("camp-shelter", [0.89, 0.87, 0.79, 1.0], PAINTED_METAL,
        ParametricRecipe::Yawed { source: Box::new(coachwork), pivot: [0.0; 3],
            degrees: 90.0, target: [X, 0.0, Z] })];
    // Separate flat side stripes avoid the old revolved ring projecting well
    // beyond the body. Metal trim has its own physical response.
    out.push(panels("camp-shelter-band", [0.64, 0.43, 0.24, 1.0], PAINTED_METAL, vec![
        part([X, 0.91, Z + 0.865], [half_length * 2.0 - 0.58, 0.16, 0.018]),
        part([X, 0.91, Z - 0.865], [half_length * 2.0 - 0.58, 0.16, 0.018]),
    ]));
    let mut chassis = vec![
        part([X, 0.43, Z - 0.48], [half_length * 2.0, 0.12, 0.09]),
        part([X, 0.43, Z + 0.48], [half_length * 2.0, 0.12, 0.09]),
        part([X - 0.45, 0.35, Z], [0.09, 0.09, 1.71]),
        part([X + half_length - 0.11, 0.63, Z], [0.08, 0.12, 1.51]),
        part([X - half_length + 0.10, 0.57, Z], [0.08, 0.12, 1.42]),
    ];
    for (window_x, window_z) in [
        (X - 0.62, Z + 0.876), (X + 0.52, Z + 0.876),
        (X - 0.62, Z - 0.876), (X + 0.52, Z - 0.876),
    ] {
        chassis.extend([
            part([window_x, 1.30, window_z], [0.79, 0.045, 0.045]),
            part([window_x, 1.95, window_z], [0.79, 0.045, 0.045]),
            part([window_x - 0.37, 1.625, window_z], [0.045, 0.65, 0.045]),
            part([window_x + 0.37, 1.625, window_z], [0.045, 0.65, 0.045]),
        ]);
    }
    // Roof hatch and seam rails are included in the existing chassis identity.
    chassis.extend([
        part([X - 0.27, 2.61, Z], [0.52, 0.045, 0.57]),
        part([X + 0.53, 2.615, Z], [0.33, 0.045, 0.34]),
        part([X, 2.52, Z - 0.74], [half_length * 2.0 - 0.62, 0.04, 0.045]),
        part([X, 2.52, Z + 0.74], [half_length * 2.0 - 0.62, 0.04, 0.045]),
    ]);
    out.push(panels("camp-shelter-chassis", [0.38, 0.41, 0.40, 1.0], GALVANISED_METAL, chassis));
    for (id, z) in [("camp-shelter-wheel-port", Z - 0.88), ("camp-shelter-wheel-starboard", Z + 0.88)] {
        out.push(model(id, [0.08, 0.09, 0.085, 1.0], RUBBER,
            ParametricRecipe::Wheel { center: [X - 0.45, 0.34, z], radius: 0.34,
                width: 0.21, segments: 32 }));
    }
    out.push(panels("camp-shelter-hitch", [0.43, 0.45, 0.43, 1.0], GALVANISED_METAL, vec![
        PanelPart { yaw_deg: 29.0, ..part([X + half_length + 0.28, 0.34, Z - 0.20], [0.94, 0.09, 0.08]) },
        PanelPart { yaw_deg: -29.0, ..part([X + half_length + 0.28, 0.34, Z + 0.20], [0.94, 0.09, 0.08]) },
        part([X + half_length + 0.70, 0.35, Z], [0.18, 0.11, 0.13]),
        part([X + half_length + 0.48, 0.22, Z], [0.06, 0.29, 0.06]),
    ]));
    out.push(panels("camp-shelter-windows", [0.27, 0.40, 0.44, 1.0], FRAMED_GLASS, vec![
        part([X - 0.62, 1.625, Z + 0.875], [0.70, 0.59, 0.018]),
        part([X + 0.52, 1.625, Z + 0.875], [0.70, 0.59, 0.018]),
        part([X - 0.62, 1.625, Z - 0.875], [0.70, 0.59, 0.018]),
        part([X + 0.52, 1.625, Z - 0.875], [0.70, 0.59, 0.018]),
    ]));
    out.push(panels("camp-shelter-door", [0.77, 0.75, 0.65, 1.0], PAINTED_METAL, vec![
        part([X + half_length - 0.04, 1.39, Z], [0.037, 1.74, 0.60]),
        part([X + half_length + 0.004, 1.35, Z - 0.20], [0.025, 0.12, 0.035]),
        part([X + half_length + 0.18, 0.28, Z], [0.38, 0.055, 0.62]),
        part([X + half_length + 0.10, 0.11, Z], [0.055, 0.30, 0.055]),
    ]));
    out.push(model("camp-shelter-awning", [0.78, 0.76, 0.62, 1.0], SHADE_CANVAS,
        ParametricRecipe::TiltedBox { center: [X + half_length + 0.43, 2.28, Z],
            size: [0.95, 0.025, 1.62], pitch_deg: 0.0, roll_deg: -12.0, yaw_deg: 0.0 }));
    for (id, z) in [("camp-shelter-awning-post-a", Z - 0.73), ("camp-shelter-awning-post-b", Z + 0.73)] {
        out.push(model(id, [0.44, 0.47, 0.43, 1.0], GALVANISED_METAL,
            ParametricRecipe::Cylinder { center: [X + half_length + 0.83, 1.09, z],
                radius: 0.022, height: 2.18, segments: 12 }));
    }
    if upgrades > 0 {
        let span = 1.28 + upgrades.min(3) as f32 * 0.07;
        let center = X - half_length - span * 0.5 + 0.10;
        // Attached canvas room with upright walls and pitched roof. The front
        // has two narrow leaves around an unobstructed central entry.
        out.push(panels("camp-shelter-annex", [0.61, 0.64, 0.47, 1.0], SHADE_CANVAS, vec![
            part([center, 1.0, Z - 0.76], [span, 1.94, 0.028]),
            part([center, 1.0, Z + 0.76], [span, 1.94, 0.028]),
            part([center - span * 0.5, 1.0, Z - 0.54], [0.028, 1.94, 0.44]),
            part([center - span * 0.5, 1.0, Z + 0.54], [0.028, 1.94, 0.44]),
            PanelPart { pitch_deg: 16.0, ..part([center, 2.10, Z + 0.38], [span, 0.025, 0.80]) },
            PanelPart { pitch_deg: -16.0, ..part([center, 2.10, Z - 0.38], [span, 0.025, 0.80]) },
        ]));
    }
    out
}
