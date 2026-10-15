//! Profile-selected Australian native planting for the fictional showcase grounds.
//!
//! Stable render keys let a source-backed locality choose appropriate plants
//! without turning one corridor-wide list into a claim of local co-occurrence.

use crate::asset_catalog::{material_family_for, AssetRecipe, EllipsoidInstance, ParametricRecipe};

fn stem(id: String, start: [f32; 3], end: [f32; 3], radius: f32, color: [f32; 4]) -> AssetRecipe {
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::Stem {
            start,
            end,
            radius,
            segments: 9,
        }),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn cluster(id: String, instances: Vec<EllipsoidInstance>, color: [f32; 4]) -> AssetRecipe {
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::EllipsoidCluster {
            instances,
            latitude: 7,
            longitude: 10,
        }),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn ellipsoid(id: String, center: [f32; 3], radii: [f32; 3], color: [f32; 4]) -> AssetRecipe {
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::Ellipsoid {
            center,
            radii,
            latitude: 8,
            longitude: 12,
        }),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn botanical_envelope(
    id: String,
    center: [f32; 3],
    spread: f32,
    height: f32,
    color: [f32; 4],
) -> AssetRecipe {
    let material_family = material_family_for(&id);
    let r = spread.max(0.18);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(ParametricRecipe::BotanicalHull {
            center,
            radii: [r * 0.90, r * 1.10, r * 0.95, r * 1.05],
            heights: [0.0, 0.04, -0.02, 0.03],
            bottom_elevation: -height * 0.45,
            top_elevation: height * 0.55,
            bottom_shape: 1.85,
            top_shape: 2.15,
            slices: 14,
            stacks: 7,
        }),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn radial_leaves(
    center: [f32; 3],
    count: usize,
    radius: f32,
    leaf_length: f32,
    leaf_width: f32,
    pitch: f32,
) -> Vec<EllipsoidInstance> {
    (0..count)
        .map(|index| {
            let angle = index as f32 * std::f32::consts::TAU / count.max(1) as f32;
            let wobble = ((index * 17 + 5) % 11) as f32 / 10.0;
            EllipsoidInstance {
                center: [
                    center[0] + angle.cos() * radius,
                    center[1] + (wobble - 0.5) * 0.18,
                    center[2] + angle.sin() * radius,
                ],
                radii: [
                    leaf_width,
                    leaf_width * 0.75,
                    leaf_length * (0.78 + wobble * 0.22),
                ],
                pitch_deg: pitch + wobble * 7.0,
                yaw_deg: angle.to_degrees(),
                roll_deg: (wobble - 0.5) * 14.0,
            }
        })
        .collect()
}

fn crown_leaves(
    center: [f32; 3],
    count: usize,
    spread: f32,
    leaf_length: f32,
    leaf_width: f32,
    pitch: f32,
) -> Vec<EllipsoidInstance> {
    (0..count)
        .map(|index| {
            let angle = index as f32 * std::f32::consts::TAU / count.max(1) as f32;
            let radial = spread * (0.22 + ((index * 11 + 3) % 17) as f32 / 17.0 * 0.78);
            let vertical = (((index * 7 + 5) % 13) as f32 / 12.0 - 0.5) * spread * 0.52;
            let variation = ((index * 5 + 2) % 9) as f32 / 8.0;
            EllipsoidInstance {
                center: [
                    center[0] + angle.cos() * radial,
                    center[1] + vertical,
                    center[2] + angle.sin() * radial,
                ],
                radii: [
                    leaf_width * (0.78 + variation * 0.28),
                    leaf_width * 0.62,
                    leaf_length * (0.82 + variation * 0.28),
                ],
                pitch_deg: pitch + vertical * 9.0,
                yaw_deg: angle.to_degrees() + variation * 22.0,
                roll_deg: (variation - 0.5) * 18.0,
            }
        })
        .collect()
}

/// A profile-selectable, generic eucalypt silhouette. It is intentionally
/// genus-level: it adds an Australian landscape form without implying a
/// species occurrence for the synthetic demo locality.
fn eucalypt_gum() -> Vec<AssetRecipe> {
    let (x, z) = (6.05, -5.55);
    let bark = [0.70, 0.67, 0.56, 1.0];
    let branch = [0.42, 0.39, 0.31, 1.0];
    let mut assets = vec![stem(
        "rc:asset/flora/eucalypt-gum/trunk".into(),
        [x, 0.02, z],
        [x - 0.06, 5.65, z + 0.02],
        0.24,
        bark,
    )];
    for (index, (dx, dz, end_y, radius)) in [
        (-1.18, -0.20, 4.86, 0.105),
        (1.12, 0.18, 5.05, 0.095),
        (-0.62, 0.92, 5.32, 0.09),
        (0.52, -0.98, 5.26, 0.085),
        (0.96, 0.78, 4.72, 0.08),
    ]
    .into_iter()
    .enumerate()
    {
        assets.push(stem(
            format!("rc:asset/flora/eucalypt-gum/branch-{index}"),
            [x - 0.04, 3.52 + index as f32 * 0.17, z],
            [x + dx, end_y, z + dz],
            radius,
            branch,
        ));
    }
    for (index, (center, spread, color)) in [
        ([x - 0.66, 5.18, z - 0.12], 1.10, [0.34, 0.48, 0.37, 1.0]),
        ([x + 0.58, 5.28, z + 0.12], 1.08, [0.29, 0.43, 0.34, 1.0]),
        ([x - 0.04, 5.75, z + 0.06], 0.94, [0.38, 0.51, 0.39, 1.0]),
    ]
    .into_iter()
    .enumerate()
    {
        assets.push(botanical_envelope(
            format!("rc:asset/flora/eucalypt-gum/blue-grey-leaves-{index}"),
            center,
            spread * 1.18,
            spread * 0.92,
            color,
        ));
    }
    assets
}

/// A compact golden-wattle understory group, authored as layered fine stems,
/// narrow phyllodes and distinct round blossom clusters.
fn golden_wattle() -> Vec<AssetRecipe> {
    let (x, z) = (5.78, 5.72);
    let mut assets = Vec::new();
    for (index, (dx, dz, height)) in [
        (-0.26, -0.12, 2.55),
        (0.18, 0.10, 2.85),
        (0.38, -0.18, 2.35),
        (-0.08, 0.30, 2.62),
    ]
    .into_iter()
    .enumerate()
    {
        assets.push(stem(
            format!("rc:asset/flora/golden-wattle/stem-{index}"),
            [x, 0.03, z],
            [x + dx, height, z + dz],
            0.042,
            [0.39, 0.30, 0.20, 1.0],
        ));
    }
    for (index, (center, spread)) in [
        ([x - 0.36, 2.00, z - 0.03], 0.56),
        ([x + 0.20, 2.26, z + 0.12], 0.62),
        ([x + 0.48, 1.82, z - 0.18], 0.50),
        ([x - 0.08, 2.55, z + 0.30], 0.53),
    ]
    .into_iter()
    .enumerate()
    {
        assets.push(botanical_envelope(
            format!("rc:asset/flora/golden-wattle/blue-green-phyllodes-{index}"),
            center,
            spread * 0.95,
            spread * 0.82,
            [0.38, 0.54, 0.39, 1.0],
        ));
        assets.push(cluster(
            format!("rc:asset/flora/golden-wattle/gold-blossoms-{index}"),
            radial_leaves(
                [center[0], center[1] + 0.18, center[2]],
                9,
                spread * 0.42,
                0.13,
                0.105,
                0.0,
            ),
            [0.94, 0.66, 0.16, 1.0],
        ));
    }
    assets
}

fn bottlebrush() -> Vec<AssetRecipe> {
    let (x, z) = (-5.15, -5.45);
    let mut assets = vec![
        stem(
            "rc:asset/flora/coastal-bottlebrush/trunk".into(),
            [x, 0.0, z],
            [x, 1.05, z],
            0.105,
            [0.37, 0.25, 0.17, 1.0],
        ),
        stem(
            "rc:asset/flora/coastal-bottlebrush/branch-west".into(),
            [x, 0.64, z],
            [x - 0.50, 1.13, z + 0.04],
            0.055,
            [0.40, 0.28, 0.18, 1.0],
        ),
        stem(
            "rc:asset/flora/coastal-bottlebrush/branch-east".into(),
            [x, 0.70, z],
            [x + 0.46, 1.16, z - 0.05],
            0.05,
            [0.40, 0.28, 0.18, 1.0],
        ),
    ];
    assets.push(cluster(
        "rc:asset/flora/coastal-bottlebrush/foliage".into(),
        radial_leaves([x, 1.27, z], 22, 0.36, 0.31, 0.085, 8.0),
        [0.22, 0.43, 0.25, 1.0],
    ));
    assets.push(cluster(
        "rc:asset/flora/coastal-bottlebrush/brush-flowers".into(),
        [
            EllipsoidInstance {
                center: [x - 0.39, 1.31, z + 0.04],
                radii: [0.105, 0.34, 0.12],
                pitch_deg: -9.0,
                yaw_deg: -24.0,
                roll_deg: 0.0,
            },
            EllipsoidInstance {
                center: [x + 0.36, 1.36, z - 0.04],
                radii: [0.10, 0.32, 0.115],
                pitch_deg: 8.0,
                yaw_deg: 21.0,
                roll_deg: 0.0,
            },
            EllipsoidInstance {
                center: [x + 0.02, 1.57, z + 0.31],
                radii: [0.095, 0.30, 0.11],
                pitch_deg: -13.0,
                yaw_deg: 0.0,
                roll_deg: 4.0,
            },
        ]
        .to_vec(),
        [0.78, 0.22, 0.18, 1.0],
    ));
    assets
}

fn banksia() -> Vec<AssetRecipe> {
    let (x, z) = (5.35, -5.20);
    let mut assets = vec![stem(
        "rc:asset/flora/banksia/trunk".into(),
        [x, 0.0, z],
        [x, 1.30, z],
        0.115,
        [0.37, 0.25, 0.17, 1.0],
    )];
    for (index, (dx, dz)) in [(-0.38, -0.12), (0.32, 0.20), (0.10, -0.34)]
        .into_iter()
        .enumerate()
    {
        assets.push(stem(
            format!("rc:asset/flora/banksia/branch-{index}"),
            [x, 0.85, z],
            [x + dx, 1.46, z + dz],
            0.045,
            [0.40, 0.28, 0.18, 1.0],
        ));
    }
    assets.push(cluster(
        "rc:asset/flora/banksia/serrated-leaves".into(),
        radial_leaves([x, 1.57, z], 20, 0.34, 0.43, 0.065, 12.0),
        [0.25, 0.43, 0.23, 1.0],
    ));
    assets.push(cluster(
        "rc:asset/flora/banksia/golden-cones".into(),
        [
            EllipsoidInstance {
                center: [x - 0.24, 1.63, z - 0.02],
                radii: [0.13, 0.32, 0.13],
                pitch_deg: 0.0,
                yaw_deg: -22.0,
                roll_deg: 0.0,
            },
            EllipsoidInstance {
                center: [x + 0.22, 1.70, z + 0.10],
                radii: [0.12, 0.30, 0.12],
                pitch_deg: 9.0,
                yaw_deg: 32.0,
                roll_deg: 4.0,
            },
            EllipsoidInstance {
                center: [x + 0.02, 1.79, z - 0.25],
                radii: [0.115, 0.28, 0.115],
                pitch_deg: -6.0,
                yaw_deg: -4.0,
                roll_deg: 0.0,
            },
        ]
        .to_vec(),
        [0.82, 0.50, 0.18, 1.0],
    ));
    assets
}

fn grass_tree() -> Vec<AssetRecipe> {
    let (x, z) = (-5.55, 5.55);
    let mut assets = vec![stem(
        "rc:asset/flora/grass-tree/trunk".into(),
        [x, 0.0, z],
        [x + 0.06, 1.38, z],
        0.16,
        [0.22, 0.18, 0.14, 1.0],
    )];
    assets.push(cluster(
        "rc:asset/flora/grass-tree/spiky-crown-leaves".into(),
        radial_leaves([x + 0.04, 1.52, z], 36, 0.31, 0.95, 0.035, 7.0),
        [0.28, 0.42, 0.22, 1.0],
    ));
    assets.push(cluster(
        "rc:asset/flora/grass-tree/old-leaf-skirt".into(),
        radial_leaves([x, 0.70, z], 18, 0.20, 0.46, 0.045, 58.0),
        [0.31, 0.26, 0.19, 1.0],
    ));
    assets
}

fn saltbush() -> Vec<AssetRecipe> {
    let (x, z) = (2.20, -17.10);
    let mut assets = Vec::new();
    for (index, (dx, dz, height)) in [
        (-0.34, -0.08, 0.60),
        (0.28, -0.14, 0.72),
        (-0.08, 0.27, 0.67),
    ]
    .into_iter()
    .enumerate()
    {
        assets.push(stem(
            format!("rc:asset/flora/saltbush/woody-stem-{index}"),
            [x, 0.02, z],
            [x + dx, height, z + dz],
            0.035,
            [0.42, 0.34, 0.25, 1.0],
        ));
    }
    assets.push(cluster(
        "rc:asset/flora/saltbush/silvery-foliage".into(),
        radial_leaves([x, 0.51, z], 25, 0.35, 0.21, 0.12, 12.0),
        [0.57, 0.62, 0.49, 1.0],
    ));
    assets
}

fn kangaroo_paw() -> Vec<AssetRecipe> {
    let (x, z) = (-2.90, -5.35);
    let mut assets = vec![cluster(
        "rc:asset/flora/kangaroo-paw/strappy-leaves".into(),
        radial_leaves([x, 0.25, z], 22, 0.16, 0.61, 0.045, 63.0),
        [0.24, 0.43, 0.25, 1.0],
    )];
    for index in 0..4 {
        let angle = index as f32 * std::f32::consts::TAU / 4.0;
        let end = [
            x + angle.cos() * 0.28,
            1.18 + (index % 2) as f32 * 0.13,
            z + angle.sin() * 0.28,
        ];
        assets.push(stem(
            format!("rc:asset/flora/kangaroo-paw/flower-stalk-{index}"),
            [x, 0.28, z],
            end,
            0.026,
            [0.23, 0.40, 0.21, 1.0],
        ));
        assets.push(ellipsoid(
            format!("rc:asset/flora/kangaroo-paw/velvet-flower-{index}"),
            [end[0], end[1] + 0.10, end[2]],
            [0.095, 0.20, 0.11],
            if index % 2 == 0 {
                [0.78, 0.25, 0.16, 1.0]
            } else {
                [0.88, 0.56, 0.16, 1.0]
            },
        ));
    }
    assets
}

/// Build only the original Australian plant groups selected by a place profile.
/// Unrecognised profile keys remain data-only and never receive a guessed model.
pub fn profile_assets(selection: &[String]) -> Vec<AssetRecipe> {
    let mut assets = Vec::new();
    for key in selection {
        match key.as_str() {
            "eucalypt-gum" => assets.extend(eucalypt_gum()),
            "golden-wattle" => assets.extend(golden_wattle()),
            "coastal-bottlebrush" => assets.extend(bottlebrush()),
            "banksia" => assets.extend(banksia()),
            "grass-tree" => assets.extend(grass_tree()),
            "saltbush" => assets.extend(saltbush()),
            "kangaroo-paw" => assets.extend(kangaroo_paw()),
            _ => {}
        }
    }
    assets
}
