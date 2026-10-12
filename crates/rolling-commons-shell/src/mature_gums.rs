//! Authored mature eucalypt framing family for the Community Grounds.
//! Qualia's revolve/transform primitives compile the woody skeleton and leaf
//! assemblies. These are genus-level fictional plantings, not occurrence data.

use crate::asset_catalog::{
    AssetRecipe, EllipsoidInstance, LimbSegment, ParametricRecipe, LIVING_FOLIAGE,
    WEATHERED_TIMBER,
};

fn wood(id: &str, center: [f32; 3], limbs: Vec<LimbSegment>, color: [f32; 4]) -> AssetRecipe {
    AssetRecipe {
        id: format!("rc:asset/{id}"),
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::LimbCluster {
            center, limbs, radial_segments: 14,
        }),
        color, material_family: WEATHERED_TIMBER, vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn limb(start: [f32; 3], end: [f32; 3], a: f32, b: f32) -> LimbSegment {
    LimbSegment { start, end, start_radius: a, end_radius: b }
}

/// Narrow, pendulous leaflets grouped into branch-end sprays, with open sky
/// between sprays. There is deliberately no solid crown hiding the branches.
fn leaves(id: &str, tips: &[[f32; 3]], phase: f32, tone: usize) -> AssetRecipe {
    let mut instances = Vec::with_capacity(tips.len() * 24);
    for (spray, tip) in tips.iter().enumerate() {
        for leaf in 0..24 {
            let angle = phase + leaf as f32 * 2.399_963;
            let variation = ((leaf * 13 + spray * 7 + tone * 11) % 23) as f32 / 23.0;
            let reach = 0.18 + variation * 0.48;
            instances.push(EllipsoidInstance {
                center: [
                    tip[0] + angle.cos() * reach,
                    tip[1] + (variation - 0.5) * 0.40 - (leaf % 4) as f32 * 0.065,
                    tip[2] + angle.sin() * reach * 0.76,
                ],
                radii: [0.035 + variation * 0.016, 0.19 + variation * 0.13, 0.013],
                pitch_deg: 10.0 + angle.sin() * 32.0,
                yaw_deg: angle.to_degrees(),
                roll_deg: angle.cos() * 24.0,
            });
        }
    }
    let palette = [
        [0.34, 0.42, 0.24, 1.0], [0.42, 0.49, 0.29, 1.0],
        [0.30, 0.39, 0.28, 1.0], [0.49, 0.53, 0.33, 1.0],
    ];
    AssetRecipe {
        id: format!("rc:asset/{id}"), parts: Vec::new(),
        parametric: Some(ParametricRecipe::EllipsoidCluster {
            instances, latitude: 4, longitude: 6,
        }),
        color: palette[tone % palette.len()], material_family: LIVING_FOLIAGE,
        vibe_source: None, render_role: "opaque".into(),
    }
}

/// Keep every former trunk/canopy/spray identity, replacing its construction.
/// Rear gums are taller; foreground trees spread away from the central routes.
pub fn assets() -> Vec<AssetRecipe> {
    let mut result = Vec::with_capacity(36);
    for (index, (key, x, z, height, lean, phase)) in [
        ("nw", -6.15, -5.5, 6.4, -0.42, 0.20),
        ("ne", 6.1, -5.2, 5.8, 0.37, 0.83),
        ("sw", -6.1, 5.2, 5.4, -0.50, 1.41),
        ("se", 6.1, 4.8, 5.9, 0.32, 2.06),
    ].into_iter().enumerate() {
        let root = [x, crate::terrain::settlement_height(0.0, x, z) - 0.01, z];
        let h = height;
        let joint = [lean * 0.65, h * 0.45, -0.10];
        let top = [lean, h * 0.73, 0.08];
        result.push(wood(&format!("tree-trunk-{key}"), root, vec![
            limb([0.0, 0.0, 0.0], [lean * 0.28, h * 0.23, 0.04], 0.31, 0.24),
            limb([lean * 0.28, h * 0.23, 0.04], joint, 0.24, 0.17),
            limb(joint, top, 0.17, 0.07),
        ], [0.76, 0.74, 0.63, 1.0]));
        let mut branches = Vec::with_capacity(18);
        let mut tips = Vec::with_capacity(9);
        for branch in 0..9 {
            let angle = phase + branch as f32 * 2.399_963;
            let spread = 1.05 + (branch % 3) as f32 * 0.38;
            let start = [lean * 0.72, h * (0.42 + branch as f32 * 0.023), -0.06];
            let elbow = [lean + angle.cos() * spread * 0.55,
                h * (0.68 + (branch % 3) as f32 * 0.04), angle.sin() * spread * 0.49];
            let tip = [lean + angle.cos() * spread,
                h * (0.79 + (branch % 4) as f32 * 0.047), angle.sin() * spread * 0.83];
            branches.push(limb(start, elbow, 0.11 - branch as f32 * 0.004, 0.046));
            branches.push(limb(elbow, tip, 0.046, 0.012));
            tips.push([root[0] + tip[0], root[1] + tip[1], root[2] + tip[2]]);
        }
        result.push(wood(&format!("gum-fork-{key}"), root, branches, [0.64, 0.62, 0.51, 1.0]));
        result.push(wood(&format!("gum-bark-{key}"), root, vec![
            limb([-0.035, 0.04, 0.03], [lean * 0.10 - 0.04, h * 0.12, 0.05], 0.318, 0.27),
            limb([lean * 0.28 + 0.012, h * 0.23, 0.046],
                 [lean * 0.44 + 0.015, h * 0.33, 0.04], 0.242, 0.207),
        ], [0.45, 0.41, 0.32, 1.0]));
        // Stable groups occupy different branch ends instead of layering solid
        // ellipsoids and fine leaves at the same position.
        for (id, range, tone) in [
            (format!("tree-crown-{key}"), 0..2, index),
            (format!("canopy-{key}-a"), 2..4, index + 1),
            (format!("canopy-{key}-b"), 4..6, index + 2),
            (format!("gum-spray-{key}-west"), 6..7, index + 2),
            (format!("gum-spray-{key}-east"), 7..8, index + 1),
            (format!("gum-foliage-{key}"), 8..9, index + 3),
        ] {
            result.push(leaves(&id, &tips[range], phase, tone));
        }
    }
    result
}
