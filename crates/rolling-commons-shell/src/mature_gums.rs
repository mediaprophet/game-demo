//! Authored mature eucalypt framing family for the Community Grounds.
//! Qualia's botanical hull (Cescatti 1997) and sweep extrusion (Bishop frame)
//! primitives compile the organic woody skeleton and continuous foliage canopies.

use crate::asset_catalog::{
    AssetRecipe, LimbSegment, ParametricRecipe, LIVING_FOLIAGE, WEATHERED_TIMBER,
};
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::ExtrusionStation;

fn limb(start: [f32; 3], end: [f32; 3], a: f32, b: f32) -> LimbSegment {
    LimbSegment {
        start,
        end,
        start_radius: a,
        end_radius: b,
    }
}

fn wood_limbs(id: &str, center: [f32; 3], limbs: Vec<LimbSegment>, color: [f32; 4]) -> AssetRecipe {
    AssetRecipe {
        id: format!("rc:asset/{id}"),
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::LimbCluster {
            center,
            limbs,
            radial_segments: 12,
        }),
        color,
        material_family: WEATHERED_TIMBER,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn swept_trunk(id: &str, root: [f32; 3], h: f32, lean: f32, color: [f32; 4]) -> AssetRecipe {
    let axis = vec![
        [0.0, 0.0, 0.0],
        [lean * 0.28, h * 0.23, 0.04],
        [lean * 0.65, h * 0.45, -0.10],
        [lean, h * 0.73, 0.08],
    ];
    let sides = 12;
    let r = 0.32;
    let profile: Vec<[f32; 2]> = (0..sides)
        .map(|i| {
            let a = i as f32 * std::f32::consts::TAU / sides as f32;
            [a.cos() * r, a.sin() * r]
        })
        .collect();
    let stations = vec![
        ExtrusionStation {
            scale_x: 1.0,
            scale_y: 1.0,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: 0.82,
            scale_y: 0.82,
            twist_rad: 0.05,
        },
        ExtrusionStation {
            scale_x: 0.58,
            scale_y: 0.58,
            twist_rad: 0.12,
        },
        ExtrusionStation {
            scale_x: 0.32,
            scale_y: 0.32,
            twist_rad: 0.18,
        },
    ];
    AssetRecipe {
        id: format!("rc:asset/{id}"),
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::SweepExtrusion {
            center: root,
            axis,
            profile,
            stations: Some(stations),
            closed_section: true,
            cap_ends: true,
        }),
        color,
        material_family: WEATHERED_TIMBER,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

/// Continuous, watertight botanical canopy envelopes (Cescatti 1997)
/// replace hundreds of disjoint micro-spheres with smooth eucalypt foliage pads.
fn canopy_envelope(id: &str, tips: &[[f32; 3]], phase: f32, tone: usize) -> AssetRecipe {
    let mut center = [0.0; 3];
    for tip in tips {
        center[0] += tip[0];
        center[1] += tip[1];
        center[2] += tip[2];
    }
    let n = tips.len().max(1) as f32;
    center[0] /= n;
    center[1] /= n;
    center[2] /= n;

    let palette = [
        [0.34, 0.42, 0.24, 1.0],
        [0.42, 0.49, 0.29, 1.0],
        [0.30, 0.39, 0.28, 1.0],
        [0.49, 0.53, 0.33, 1.0],
    ];
    let variation = (phase * 3.7 + tone as f32 * 0.8).sin() * 0.18;
    let base_r = (0.95 + variation).max(0.45);
    let rx_neg = (base_r * 0.88).max(0.35);
    let rx_pos = (base_r * 1.18).max(0.35);
    let rz_neg = (base_r * 0.92).max(0.35);
    let rz_pos = (base_r * 1.12).max(0.35);

    let h_span = 0.85 + (phase * 2.1).cos().abs() * 0.3;
    let bot_elev = -h_span * 0.42;
    let top_elev = h_span * 0.58;
    let mid_h = (bot_elev + top_elev) * 0.5;

    AssetRecipe {
        id: format!("rc:asset/{id}"),
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::BotanicalHull {
            center,
            radii: [rx_neg, rx_pos, rz_neg, rz_pos],
            heights: [mid_h - 0.05, mid_h + 0.08, mid_h + 0.02, mid_h - 0.04],
            bottom_elevation: bot_elev,
            top_elevation: top_elev,
            bottom_shape: 1.8,
            top_shape: 2.1,
            slices: 14,
            stacks: 7,
        }),
        color: palette[tone % palette.len()],
        material_family: LIVING_FOLIAGE,
        vibe_source: None,
        render_role: "opaque".into(),
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
    ]
    .into_iter()
    .enumerate()
    {
        let root = [x, crate::terrain::settlement_height(0.0, x, z) - 0.01, z];
        let h = height;

        // Smooth swept trunk with continuous taper
        result.push(swept_trunk(
            &format!("tree-trunk-{key}"),
            root,
            h,
            lean,
            [0.76, 0.74, 0.63, 1.0],
        ));

        let mut branches = Vec::with_capacity(18);
        let mut tips = Vec::with_capacity(9);
        for branch in 0..9 {
            let angle = phase + branch as f32 * 2.399_963;
            let spread = 1.05 + (branch % 3) as f32 * 0.38;
            let start = [lean * 0.72, h * (0.42 + branch as f32 * 0.023), -0.06];
            let elbow = [
                lean + angle.cos() * spread * 0.55,
                h * (0.68 + (branch % 3) as f32 * 0.04),
                angle.sin() * spread * 0.49,
            ];
            let tip = [
                lean + angle.cos() * spread,
                h * (0.79 + (branch % 4) as f32 * 0.047),
                angle.sin() * spread * 0.83,
            ];
            branches.push(limb(start, elbow, 0.11 - branch as f32 * 0.004, 0.046));
            branches.push(limb(elbow, tip, 0.046, 0.012));
            tips.push([root[0] + tip[0], root[1] + tip[1], root[2] + tip[2]]);
        }
        result.push(wood_limbs(
            &format!("gum-fork-{key}"),
            root,
            branches,
            [0.64, 0.62, 0.51, 1.0],
        ));
        result.push(wood_limbs(
            &format!("gum-bark-{key}"),
            root,
            vec![
                limb(
                    [-0.035, 0.04, 0.03],
                    [lean * 0.10 - 0.04, h * 0.12, 0.05],
                    0.318,
                    0.27,
                ),
                limb(
                    [lean * 0.28 + 0.012, h * 0.23, 0.046],
                    [lean * 0.44 + 0.015, h * 0.33, 0.04],
                    0.242,
                    0.207,
                ),
            ],
            [0.45, 0.41, 0.32, 1.0],
        ));

        // Stable canopy envelope groups occupy branch ends with continuous watertight meshes
        for (id, range, tone) in [
            (format!("tree-crown-{key}"), 0..2, index),
            (format!("canopy-{key}-a"), 2..4, index + 1),
            (format!("canopy-{key}-b"), 4..6, index + 2),
            (format!("gum-spray-{key}-west"), 6..7, index + 2),
            (format!("gum-spray-{key}-east"), 7..8, index + 1),
            (format!("gum-foliage-{key}"), 8..9, index + 3),
        ] {
            result.push(canopy_envelope(&id, &tips[range], phase, tone));
        }
    }
    result
}
