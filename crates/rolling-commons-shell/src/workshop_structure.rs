//! Human-scale, open-bay community shed, authored in source-world metres.
//! All surfaces compile through Qualia's boxes, transforms and panel assemblies.
//! Stable workshop/solar identities retain their existing semantic picks.

use crate::asset_catalog::{
    AssetRecipe, MaterialFamily, PanelPart, ParametricRecipe, FRAMED_GLASS,
    GALVANISED_METAL, PAINTED_METAL, SOLAR_GLASS, WEATHERED_CONCRETE,
    WEATHERED_TIMBER,
};

const STEEL: [f32; 4] = [0.37, 0.42, 0.39, 1.0];
const TIMBER: [f32; 4] = [0.54, 0.41, 0.28, 1.0];
const ROOF: [f32; 4] = [0.39, 0.44, 0.36, 1.0];
const RIDGE_Y: f32 = 3.25;
const RIDGE_Z: f32 = -2.0;
const PITCH: f32 = 22.0;
// Set the enlarged shed behind the existing road. Its covered work frontage
// still meets the unchanged bench/activity pick at source Z = -0.91.
const SITE_Z_OFFSET: f32 = -0.8;

fn part(center: [f32; 3], size: [f32; 3]) -> PanelPart {
    PanelPart { offset: [center[0], center[1], center[2] + SITE_Z_OFFSET],
        size, pitch_deg: 0.0, roll_deg: 0.0, yaw_deg: 0.0 }
}

fn pitched(center: [f32; 3], size: [f32; 3], pitch: f32) -> PanelPart {
    PanelPart { pitch_deg: pitch, ..part(center, size) }
}

fn group(id: &str, color: [f32; 4], family: MaterialFamily, parts: Vec<PanelPart>) -> AssetRecipe {
    AssetRecipe {
        id: format!("rc:asset/{id}"), color, material_family: family,
        parts: Vec::new(), vibe_source: None, render_role: "opaque".into(),
        parametric: Some(ParametricRecipe::PanelGroup { center: [0.0; 3], parts }),
    }
}

fn roof_y(z: f32) -> f32 {
    RIDGE_Y - (z - RIDGE_Z).abs() * PITCH.to_radians().tan()
}

/// Two genuinely open front bays. The back and side cladding stop below the
/// roof, leaving separate structural members and a visible ventilated interior.
pub fn assets(online: bool, panels: u32) -> Vec<AssetRecipe> {
    let wall = if online { [0.43, 0.50, 0.32, 1.0] } else { [0.36, 0.42, 0.29, 1.0] };
    let mut scene = Vec::new();
    let mut walls = vec![
        part([3.2, 1.35, -3.42], [3.1, 2.62, 0.055]),
        // Four sheets around the side window leave an actual aperture.
        part([1.65, 1.35, -3.14], [0.055, 2.62, 0.56]),
        part([1.65, 1.35, -1.25], [0.055, 2.62, 1.34]),
        part([1.65, 0.65, -2.38], [0.055, 1.18, 0.96]),
        part([1.65, 2.36, -2.38], [0.055, 0.60, 0.96]),
        // A side service door is a real opening between two wall sections.
        part([4.75, 1.35, -2.92], [0.055, 2.62, 1.0]),
        part([4.75, 1.35, -0.88], [0.055, 2.62, 0.60]),
        part([4.75, 2.45, -1.90], [0.055, 0.43, 1.03]),
    ];
    // Shallow standing ribs leave the open bays entirely unobstructed.
    for i in 0..32 {
        walls.push(part([1.69 + i as f32 * 0.097, 1.35, -3.458], [0.019, 2.62, 0.026]));
    }
    for i in 0..29 {
        let z = -3.38 + i as f32 * 0.097;
        if z > -2.86 && z < -1.90 {
            walls.push(part([1.614, 0.65, z], [0.026, 1.18, 0.019]));
            walls.push(part([1.614, 2.36, z], [0.026, 0.60, 0.019]));
        } else {
            walls.push(part([1.614, 1.35, z], [0.026, 2.62, 0.019]));
        }
    }
    scene.push(group("workshop", wall, PAINTED_METAL, walls));
    scene.push(group("workshop-floor", [0.52, 0.51, 0.44, 1.0], WEATHERED_CONCRETE, vec![
        part([3.2, 0.035, -1.93], [3.16, 0.07, 2.99]),
        part([3.2, 0.027, -0.16], [3.38, 0.054, 0.58]),
    ]));

    let slope_length = 1.64 / PITCH.to_radians().cos();
    let mut roof_parts = Vec::new();
    for sign in [-1.0_f32, 1.0] {
        let z = RIDGE_Z + sign * 0.82;
        roof_parts.push(pitched([3.2, roof_y(z), z], [3.52, 0.045, slope_length], sign * PITCH));
        for rib in 0..35 {
            roof_parts.push(pitched([1.50 + rib as f32 * 0.10, roof_y(z) + 0.026, z],
                [0.022, 0.025, slope_length], sign * PITCH));
        }
    }
    roof_parts.push(part([3.2, RIDGE_Y + 0.024, RIDGE_Z], [3.55, 0.045, 0.12]));
    scene.push(group("workshop-roof", ROOF, PAINTED_METAL, roof_parts));

    let mut frame = Vec::new();
    for x in [1.65, 3.2, 4.75] {
        for z in [-3.37, -0.58] {
            frame.push(part([x, 1.34, z], [0.12, 2.62, 0.12]));
            // Bolted foot plates ground the frame on the slab.
            frame.push(part([x, 0.095, z], [0.23, 0.035, 0.23]));
        }
    }
    for z in [-3.37, -0.58] {
        frame.push(part([3.2, 2.64, z], [3.2, 0.13, 0.13]));
    }
    // Six roof trusses: tie members, two sloping rafters and king posts.
    for i in 0..6 {
        let x = 1.69 + i as f32 * 0.604;
        frame.push(part([x, 2.64, -2.0], [0.08, 0.10, 2.84]));
        frame.push(part([x, 2.94, -2.0], [0.07, 0.57, 0.07]));
        for sign in [-1.0_f32, 1.0] {
            let z = RIDGE_Z + sign * 0.72;
            frame.push(pitched([x, roof_y(z) - 0.065, z], [0.08, 0.11, 1.55], sign * PITCH));
        }
    }
    scene.push(group("workshop-bay-frame", STEEL, GALVANISED_METAL, frame));
    scene.push(group("workshop-fascia", STEEL, PAINTED_METAL, vec![
        part([3.2, 2.67, -0.52], [3.53, 0.18, 0.075]),
        part([3.2, 2.67, -3.49], [3.53, 0.18, 0.075]),
        part([1.47, 2.96, -2.0], [0.09, 0.08, 0.12]),
        part([4.93, 2.96, -2.0], [0.09, 0.08, 0.12]),
    ]));

    for (bay, z) in [("outer", -0.58), ("inner", -3.37)] {
        for (side, x, sign) in [("left", 1.65, 1.0_f32), ("right", 4.75, -1.0_f32)] {
            let brace = PanelPart { roll_deg: -sign * 45.0,
                ..part([x + sign * 0.20, 2.38, z], [0.085, 0.60, 0.085]) };
            scene.push(group(&format!("workshop-timber-knee-brace-{bay}-{side}"),
                TIMBER, WEATHERED_TIMBER, vec![brace]));
        }
    }
    // Folded timber doors sit beside the front jambs, never across the bays.
    let mut doors = Vec::new();
    for (x, yaw) in [(1.72, -78.0_f32), (4.68, 78.0_f32)] {
        doors.push(PanelPart { yaw_deg: yaw, ..part([x, 1.12, -0.68], [0.35, 2.16, 0.055]) });
    }
    scene.push(group("workshop-folding-timber-doors", TIMBER, WEATHERED_TIMBER, doors));
    scene.push(group("workshop-entry-step", [0.68, 0.62, 0.48, 1.0], WEATHERED_CONCRETE,
        vec![part([3.2, 0.045, 0.06], [3.20, 0.09, 0.35])]));

    // Covered outdoor work strip, lower than the main roof and supported at
    // both ends. Clearance at the front remains above ordinary adult height.
    let canopy_pitch = 16.0_f32;
    scene.push(group("workshop-bay-canopy", ROOF, PAINTED_METAL, vec![
        pitched([3.2, 2.53, -0.08], [3.48, 0.045, 1.15], canopy_pitch),
        part([1.65, 1.18, 0.36], [0.10, 2.30, 0.10]),
        part([4.75, 1.18, 0.36], [0.10, 2.30, 0.10]),
        part([3.2, 2.35, 0.42], [3.22, 0.10, 0.10]),
    ]));
    scene.push(group("workshop-window-glazing", [0.30, 0.46, 0.49, 1.0], FRAMED_GLASS,
        vec![part([1.61, 1.65, -2.38], [0.018, 0.70, 0.86])]));
    scene.push(group("workshop-window-frame", STEEL, GALVANISED_METAL, vec![
        part([1.59, 1.27, -2.38], [0.055, 0.05, 0.96]),
        part([1.59, 2.03, -2.38], [0.055, 0.05, 0.96]),
        part([1.59, 1.65, -2.84], [0.055, 0.80, 0.05]),
        part([1.59, 1.65, -1.92], [0.055, 0.80, 0.05]),
        part([1.58, 1.65, -2.38], [0.055, 0.76, 0.035]),
    ]));
    scene.push(group("workshop-service-details", STEEL, GALVANISED_METAL, vec![
        part([4.80, 1.23, -3.08], [0.08, 0.38, 0.26]),
        part([4.81, 2.10, -3.08], [0.032, 1.34, 0.032]),
        part([3.2, 2.64, -3.52], [3.55, 0.08, 0.11]),
        part([1.52, 1.35, -3.52], [0.075, 2.55, 0.075]),
        part([1.63, 0.10, -3.52], [0.26, 0.075, 0.075]),
    ]));
    scene.push(AssetRecipe {
        id: "rc:asset/workshop-chimney".into(), parts: Vec::new(),
        parametric: Some(ParametricRecipe::Cylinder { center: [2.20, 3.15, -2.65 + SITE_Z_OFFSET],
            radius: 0.095, height: 0.95, segments: 16 }), color: STEEL,
        material_family: GALVANISED_METAL, vibe_source: None, render_role: "opaque".into(),
    });
    scene.push(group("workshop-chimney-cap", STEEL, GALVANISED_METAL,
        vec![part([2.20, 3.65, -2.65], [0.27, 0.035, 0.27])]));
    // Preserve the former trim/detail IDs at the new structural heights.
    scene.push(group("workshop-roof-panels", STEEL, GALVANISED_METAL, vec![
        part([3.2, 2.60, -3.53], [3.55, 0.07, 0.09]),
        part([3.2, 2.60, -0.47], [3.55, 0.07, 0.09]),
        part([3.2, RIDGE_Y + 0.055, RIDGE_Z], [3.55, 0.035, 0.10]),
    ]));
    scene.push(group("workshop-windows", [0.30, 0.46, 0.49, 1.0], FRAMED_GLASS, vec![
        part([1.605, 1.65, -2.60], [0.014, 0.68, 0.41]),
        part([1.605, 1.65, -2.16], [0.014, 0.68, 0.41]),
    ]));
    scene.push(group("workshop-sign", [0.68, 0.65, 0.48, 1.0], PAINTED_METAL,
        vec![part([3.2, 2.64, -0.43], [1.28, 0.23, 0.045])]));
    scene.push(group("workshop-entry-awning", STEEL, GALVANISED_METAL, vec![
        pitched([3.2, 2.49, -0.08], [3.47, 0.04, 1.15], canopy_pitch),
        part([3.2, 2.36, 0.45], [3.48, 0.09, 0.07]),
    ]));
    // Visible fixture faces change with commissioning. They are not claimed
    // to be light emitters until the engine light binding is connected.
    let fixture_color = if online { [0.91, 0.85, 0.65, 1.0] } else { [0.37, 0.40, 0.36, 1.0] };
    scene.push(group("workshop-door-glow", fixture_color, PAINTED_METAL, vec![
        part([2.43, 2.56, -0.82], [0.54, 0.055, 0.15]),
        part([3.97, 2.56, -0.82], [0.54, 0.055, 0.15]),
    ]));
    solar_roof(&mut scene, panels);
    scene
}

/// Cell centres follow their panel's tilted plane; formerly all cell rows sat
/// at a single Y value, visibly stepping above/below the underlying glass.
fn solar_roof(scene: &mut Vec<AssetRecipe>, count: u32) {
    if count == 0 { return; }
    let mut rails = Vec::new();
    let mut cells = Vec::new();
    let mut seams = Vec::new();
    for i in 0..count.min(8) {
        let sign = if i < 4 { 1.0_f32 } else { -1.0_f32 };
        let pitch = sign * PITCH;
        let angle = pitch.to_radians();
        let x = 2.12 + (i % 4) as f32 * 0.72;
        let z = RIDGE_Z + sign * 0.79;
        let y = roof_y(z) + 0.09;
        let at = |dx: f32, dz: f32, lift: f32| [x + dx,
            y - dz * angle.sin() + lift * angle.cos(),
            z + dz * angle.cos() + lift * angle.sin()];
        rails.push(pitched(at(0.0, 0.0, 0.0), [0.67, 0.055, 1.27], pitch));
        for edge in [-1.0_f32, 1.0] {
            rails.push(pitched(at(edge * 0.326, 0.0, 0.031), [0.025, 0.028, 1.27], pitch));
            rails.push(pitched(at(0.0, edge * 0.62, 0.031), [0.67, 0.028, 0.025], pitch));
        }
        for row in 0..6 {
            for col in 0..3 {
                cells.push(pitched(at((col as f32 - 1.0) * 0.205,
                    (row as f32 - 2.5) * 0.198, 0.033), [0.194, 0.012, 0.185], pitch));
            }
        }
        for col in 0..4 {
            seams.push(pitched(at((col as f32 - 1.5) * 0.205, 0.0, 0.041),
                [0.009, 0.006, 1.20], pitch));
        }
        for row in 0..7 {
            seams.push(pitched(at(0.0, (row as f32 - 3.0) * 0.198, 0.041),
                [0.62, 0.006, 0.009], pitch));
        }
    }
    scene.push(group("solar-array", STEEL, GALVANISED_METAL, rails));
    scene.push(group("solar-cells", [0.16, 0.27, 0.39, 1.0], SOLAR_GLASS, cells));
    scene.push(group("solar-grid", [0.57, 0.62, 0.64, 1.0], GALVANISED_METAL, seams));
}
