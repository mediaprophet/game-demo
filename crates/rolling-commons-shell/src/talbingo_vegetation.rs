//! Site-specific planting authored with Qualia's computational geometry.
//! Distinct Australian native flora for each of the six Talbingo and Kosciuszko zones:
//! - The Pines (Blowering): Ribbon gums and golden wattle
//! - Humes Crossing (Blowering inlet): Lakeside gums, wattle and reed margins
//! - Jounama Creek (Montane corridor): Mountain gums and river bottlebrush
//! - Yarrangobilly Village (River flats): River gums, bottlebrush and wattle
//! - O'Hares (Steep reservoir slopes): Majestic tall alpine ash
//! - Three Mile Dam (Kiandra high plains, 1400m): Contorted snow gums & alpine tussock
use crate::asset_catalog::{
    AssetRecipe, EllipsoidInstance, LimbSegment, ParametricRecipe, LIVING_FOLIAGE, WEATHERED_TIMBER,
};
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::ExtrusionStation;

fn plant(
    id: String,
    color: [f32; 4],
    family: crate::asset_catalog::MaterialFamily,
    geometry: ParametricRecipe,
) -> AssetRecipe {
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(geometry),
        color,
        material_family: family,
        vibe_source: None,
        render_role: "opaque".into(),
    }
}

fn trunk(site: u8, index: usize, x: f32, y: f32, z: f32, h: f32, lean: f32) -> AssetRecipe {
    let sides = 12;
    let profile = (0..sides)
        .map(|i| {
            let a = i as f32 * std::f32::consts::TAU / sides as f32;
            [a.cos() * 0.19, a.sin() * 0.19]
        })
        .collect();

    // Distinct bark palettes reflecting species ecology:
    // Snow gums: pale cream/olive/pale-grey mottled bark.
    // Alpine ash: tall straight grey/tan trunks.
    // Ribbon gums: warm grey with pale upper branches.
    // River gums / black sallee: olive-bronze bark.
    let (bark_color, axis) = if site == 5 {
        // Gnarled subalpine snow gum: mottled pale cream, olive-green and silver-grey bark
        (
            [0.68, 0.66, 0.58, 1.0],
            vec![
                [0.0, 0.0, 0.0],
                [lean * 0.40, h * 0.28, 0.14],
                [lean * 0.95, h * 0.58, -0.16],
                [lean * 1.35, h * 0.84, 0.12],
            ],
        )
    } else if site == 4 {
        // Towering alpine ash: straight tall fibrous grey/tan trunk
        (
            [0.48, 0.42, 0.35, 1.0],
            vec![
                [0.0, 0.0, 0.0],
                [lean * 0.15, h * 0.30, 0.01],
                [lean * 0.35, h * 0.65, -0.04],
                [lean * 0.60, h * 0.92, 0.02],
            ],
        )
    } else if site == 2 || site == 3 {
        // Montane river gums and black sallee: warm olive-bronze bark
        (
            [0.44, 0.40, 0.32, 1.0],
            vec![
                [0.0, 0.0, 0.0],
                [lean * 0.25, h * 0.26, 0.02],
                [lean * 0.65, h * 0.58, -0.08],
                [lean, h * 0.86, 0.03],
            ],
        )
    } else {
        // Ribbon gums: warm weathered grey bark
        (
            [0.52, 0.48, 0.42, 1.0],
            vec![
                [0.0, 0.0, 0.0],
                [lean * 0.22, h * 0.26, 0.02],
                [lean * 0.55, h * 0.58, -0.06],
                [lean * 0.85, h * 0.86, 0.02],
            ],
        )
    };

    plant(
        format!("rc:asset/talbingo/{site}/tree-{index}-swept-bark"),
        bark_color,
        WEATHERED_TIMBER,
        ParametricRecipe::SweepExtrusion {
            center: [x, y, z],
            axis,
            profile,
            stations: Some(vec![
                ExtrusionStation {
                    scale_x: 1.45,
                    scale_y: 1.45,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 1.02,
                    scale_y: 0.94,
                    twist_rad: if site == 5 { 0.24 } else { 0.08 },
                },
                ExtrusionStation {
                    scale_x: 0.70,
                    scale_y: 0.66,
                    twist_rad: if site == 5 { 0.42 } else { 0.18 },
                },
                ExtrusionStation {
                    scale_x: 0.38,
                    scale_y: 0.36,
                    twist_rad: if site == 5 { 0.62 } else { 0.28 },
                },
            ]),
            closed_section: true,
            cap_ends: true,
        },
    )
}

fn crown(
    site: u8,
    index: usize,
    layer: usize,
    center: [f32; 3],
    spread: f32,
    height: f32,
    color: [f32; 4],
) -> AssetRecipe {
    plant(
        format!("rc:asset/talbingo/{site}/tree-{index}-canopy-{layer}"),
        color,
        LIVING_FOLIAGE,
        ParametricRecipe::BotanicalHull {
            center,
            radii: [spread * 0.82, spread * 1.18, spread * 0.91, spread * 1.09],
            heights: [-0.08, 0.10, 0.03, -0.04],
            bottom_elevation: -height * 0.48,
            top_elevation: height * 0.52,
            bottom_shape: if site == 5 { 2.1 } else { 1.85 },
            top_shape: if site == 5 { 1.9 } else { 2.2 },
            slices: 16,
            stacks: 8,
        },
    )
}

fn gum(out: &mut Vec<AssetRecipe>, site: u8, i: usize, x: f32, z: f32, h: f32) {
    let y = crate::talbingo_terrain::elevation(site, x, z);
    let phase = i as f32 * 1.69 + site as f32 * 0.73;
    let lean = phase.sin() * (if site == 5 { 0.58 } else { 0.36 });
    out.push(trunk(site, i, x, y, z, h, lean));
    let mut limbs = Vec::new();
    let mut leaves = Vec::new();
    let branch_count = if site == 5 { 16 } else { 13 };
    for b in 0..branch_count {
        let a = phase + b as f32 * 2.399_963;
        let spread = if site == 5 {
            0.95 + (b % 4) as f32 * 0.26
        } else {
            0.80 + (b % 3) as f32 * 0.22
        };
        let start = [lean * 0.56, h * (0.42 + b as f32 * 0.024), 0.0];
        let elbow = [
            lean + a.cos() * spread * 0.52,
            h * (0.64 + (b % 3) as f32 * 0.035),
            a.sin() * spread * 0.52,
        ];
        let tip = [
            lean + a.cos() * spread,
            h * (0.75 + (b % 4) as f32 * 0.042),
            a.sin() * spread,
        ];
        limbs.push(LimbSegment {
            start,
            end: elbow,
            start_radius: 0.075,
            end_radius: 0.040,
        });
        limbs.push(LimbSegment {
            start: elbow,
            end: tip,
            start_radius: 0.040,
            end_radius: 0.009,
        });
        for n in 0..16 {
            let aa = a + n as f32 * 2.399_963;
            let rr = 0.15 + (n % 3) as f32 * 0.13;
            leaves.push(EllipsoidInstance {
                center: [
                    x + tip[0] + aa.cos() * rr,
                    y + tip[1] + ((n * 3) % 5) as f32 * 0.085,
                    z + tip[2] + aa.sin() * rr,
                ],
                radii: [0.045, 0.022, 0.13 + (n % 3) as f32 * 0.025],
                pitch_deg: 19.0 + (n % 4) as f32 * 8.0,
                yaw_deg: aa.to_degrees(),
                roll_deg: (n as f32 - 3.0) * 7.0,
            });
        }
    }
    out.push(plant(
        format!("rc:asset/talbingo/{site}/tree-{i}-forks"),
        if site == 5 {
            [0.60, 0.58, 0.50, 0.95]
        } else {
            [0.45, 0.38, 0.30, 0.95]
        },
        WEATHERED_TIMBER,
        ParametricRecipe::LimbCluster {
            center: [x, y, z],
            limbs,
            radial_segments: 10,
        },
    ));

    // Natural subalpine eucalypt foliage palettes
    let palette = if site == 5 {
        // Snow gum olive-blue-green subalpine canopy
        [
            [0.26, 0.38, 0.28, 0.95],
            [0.22, 0.34, 0.25, 0.95],
            [0.30, 0.42, 0.32, 0.95],
        ]
    } else if site == 4 {
        // Alpine ash deep rich forest canopy
        [
            [0.18, 0.36, 0.20, 0.95],
            [0.15, 0.32, 0.18, 0.95],
            [0.22, 0.40, 0.24, 0.95],
        ]
    } else {
        // Valley & foreshore gums: classic eucalyptus grey-green
        [
            [0.20, 0.38, 0.24, 0.95],
            [0.24, 0.42, 0.28, 0.95],
            [0.18, 0.34, 0.20, 0.95],
        ]
    };

    for layer in 0..3 {
        let a = phase + layer as f32 * 2.15;
        let center = [
            x + lean + a.cos() * 0.63,
            y + h * (0.79 + layer as f32 * 0.045),
            z + a.sin() * 0.55,
        ];
        out.push(crown(
            site,
            i,
            layer,
            center,
            0.77 + layer as f32 * 0.09,
            0.62,
            palette[(i + layer) % 3],
        ));
    }
    out.push(plant(
        format!("rc:asset/talbingo/{site}/tree-{i}-leaf-detail"),
        if site == 5 {
            [0.28, 0.42, 0.30, 0.95]
        } else {
            [0.22, 0.38, 0.24, 0.95]
        },
        LIVING_FOLIAGE,
        ParametricRecipe::EllipsoidCluster {
            instances: leaves,
            latitude: 5,
            longitude: 7,
        },
    ));
}

/// Golden wattle shrub (*Acacia pycnantha*): woody stems with radiant golden puff blossoms.
fn wattle(out: &mut Vec<AssetRecipe>, site: u8, i: usize, x: f32, z: f32) {
    let y = crate::talbingo_terrain::elevation(site, x, z);
    let mut limbs = Vec::new();
    let mut blossoms = Vec::new();
    for b in 0..6 {
        let a = b as f32 * 1.047;
        let tip = [a.cos() * 0.65, 0.72 + (b % 3) as f32 * 0.18, a.sin() * 0.65];
        limbs.push(LimbSegment {
            start: [0.0, 0.02, 0.0],
            end: tip,
            start_radius: 0.035,
            end_radius: 0.012,
        });
        for p in 0..4 {
            let pa = a + (p as f32 - 1.5) * 0.42;
            blossoms.push(EllipsoidInstance {
                center: [
                    x + tip[0] + pa.cos() * 0.18,
                    y + tip[1] + (p as f32) * 0.06,
                    z + tip[2] + pa.sin() * 0.18,
                ],
                radii: [0.075, 0.075, 0.075],
                pitch_deg: 0.0,
                yaw_deg: pa.to_degrees(),
                roll_deg: 0.0,
            });
        }
    }
    out.push(plant(
        format!("rc:asset/talbingo/{site}/wattle-{i}-stems"),
        [0.42, 0.32, 0.22, 1.0],
        WEATHERED_TIMBER,
        ParametricRecipe::LimbCluster {
            center: [x, y, z],
            limbs,
            radial_segments: 8,
        },
    ));
    out.push(plant(
        format!("rc:asset/talbingo/{site}/wattle-{i}-canopy"),
        [0.24, 0.38, 0.22, 0.95],
        LIVING_FOLIAGE,
        ParametricRecipe::BotanicalHull {
            center: [x, y + 0.75, z],
            radii: [0.72, 0.78, 0.70, 0.75],
            heights: [0.0, 0.04, -0.02, 0.03],
            bottom_elevation: -0.35,
            top_elevation: 0.45,
            bottom_shape: 1.8,
            top_shape: 2.1,
            slices: 12,
            stacks: 6,
        },
    ));
    out.push(plant(
        format!("rc:asset/talbingo/{site}/wattle-{i}-blooms"),
        [0.98, 0.82, 0.12, 1.0],
        LIVING_FOLIAGE,
        ParametricRecipe::EllipsoidCluster {
            instances: blossoms,
            latitude: 6,
            longitude: 8,
        },
    ));
}

/// River bottlebrush (*Callistemon sieberi*): woody riparian shrub with crimson bottlebrush spikes.
fn bottlebrush(out: &mut Vec<AssetRecipe>, site: u8, i: usize, x: f32, z: f32) {
    let y = crate::talbingo_terrain::elevation(site, x, z);
    let mut limbs = Vec::new();
    let mut brushes = Vec::new();
    for b in 0..7 {
        let a = b as f32 * 0.897;
        let tip = [a.cos() * 0.58, 0.65 + (b % 3) as f32 * 0.15, a.sin() * 0.58];
        limbs.push(LimbSegment {
            start: [0.0, 0.02, 0.0],
            end: tip,
            start_radius: 0.032,
            end_radius: 0.010,
        });
        brushes.push(EllipsoidInstance {
            center: [x + tip[0], y + tip[1] + 0.10, z + tip[2]],
            radii: [0.065, 0.16, 0.065],
            pitch_deg: 24.0,
            yaw_deg: a.to_degrees(),
            roll_deg: 0.0,
        });
    }
    out.push(plant(
        format!("rc:asset/talbingo/{site}/bottlebrush-{i}-stems"),
        [0.40, 0.30, 0.20, 1.0],
        WEATHERED_TIMBER,
        ParametricRecipe::LimbCluster {
            center: [x, y, z],
            limbs,
            radial_segments: 8,
        },
    ));
    out.push(plant(
        format!("rc:asset/talbingo/{site}/bottlebrush-{i}-foliage"),
        [0.22, 0.36, 0.20, 0.95],
        LIVING_FOLIAGE,
        ParametricRecipe::BotanicalHull {
            center: [x, y + 0.62, z],
            radii: [0.65, 0.68, 0.62, 0.65],
            heights: [0.0, 0.03, -0.02, 0.02],
            bottom_elevation: -0.30,
            top_elevation: 0.38,
            bottom_shape: 1.8,
            top_shape: 2.0,
            slices: 12,
            stacks: 6,
        },
    ));
    out.push(plant(
        format!("rc:asset/talbingo/{site}/bottlebrush-{i}-spikes"),
        [0.88, 0.18, 0.16, 1.0],
        LIVING_FOLIAGE,
        ParametricRecipe::EllipsoidCluster {
            instances: brushes,
            latitude: 6,
            longitude: 8,
        },
    ));
}

pub(crate) fn assets(site: u8) -> Vec<AssetRecipe> {
    let mut out = Vec::new();

    // 1. Specimen trees framed by locality and ecology
    for (i, (x, z, h)) in [
        (-4.5, -3.6, 3.8),
        (-1.0, 5.2, 4.5),
        (3.3, -3.7, 4.0),
        (7.1, 5.4, 4.7),
        (11.7, -3.5, 4.2),
        (15.8, 5.0, 4.1),
        (23.4, -3.6, 4.8),
    ]
    .into_iter()
    .enumerate()
    {
        let z = z + (site as f32 * 0.31 + i as f32 * 0.81).sin() * 0.30;
        // Site 5 (Three Mile Dam): subalpine snow gums are low, compact and hardy (h ~ 2.6 - 3.2m).
        // Site 4 (O'Hares): tall, imposing alpine ash (h ~ 5.2 - 5.8m).
        let h = match site {
            5 => h * 0.68,
            4 => h * 1.25,
            _ => h,
        };
        gum(&mut out, site, i, x, z, h);
    }

    // 2. Distinct native shrubs per zone
    match site {
        // The Pines: golden wattle framing the lakeside foreshore
        0 => {
            wattle(&mut out, site, 0, 1.8, -2.8);
            wattle(&mut out, site, 1, 13.5, 4.8);
            wattle(&mut out, site, 2, 21.0, -1.2);
        }
        // Humes Crossing: lakeside wattle along the inlet approach
        1 => {
            wattle(&mut out, site, 0, 2.5, 4.5);
            wattle(&mut out, site, 1, 12.0, -3.2);
        }
        // Jounama Creek: river bottlebrush nestled among granite creek boulders
        2 => {
            bottlebrush(&mut out, site, 0, 16.8, -3.8);
            bottlebrush(&mut out, site, 1, 19.5, -4.2);
            bottlebrush(&mut out, site, 2, 22.8, -2.8);
        }
        // Yarrangobilly Village: bottlebrush along the river and wattle near the hut clearing
        3 => {
            bottlebrush(&mut out, site, 0, 12.0, -4.8);
            bottlebrush(&mut out, site, 1, 17.5, -5.2);
            wattle(&mut out, site, 0, 22.8, -1.5);
        }
        // O'Hares: mountain scrub and wattle on the high ridge
        4 => {
            wattle(&mut out, site, 0, 14.5, -4.2);
            wattle(&mut out, site, 1, 23.0, -3.8);
        }
        // Three Mile Dam: subalpine shrubbery (snow wattle/heath)
        _ => {
            wattle(&mut out, site, 0, 15.2, -3.6);
            wattle(&mut out, site, 1, 22.5, -2.4);
        }
    }

    // 3. Groundcover and understory adapted to alpine vs montane vs lowland
    let mut tufts = Vec::new();
    for i in 0..24 {
        let x = -4.7 + (i as f32 * 4.13) % 28.3;
        let z = if i % 2 == 0 {
            -2.3 - ((i * 7) % 5) as f32 * 0.36
        } else {
            2.55 + ((i * 11) % 6) as f32 * 0.42
        };
        if x > 16.0 && z > 2.0 {
            continue;
        }
        tufts.push((x, z));
    }
    for (i, (x, z)) in tufts.into_iter().enumerate() {
        let (patch_color, density, blade_h) = if site == 5 {
            // Three Mile Dam: subalpine snowgrass (Poa costiniana) & alpine tussock
            if i % 3 == 0 {
                ([0.72, 0.65, 0.38, 0.95], 42, 0.32) // Golden subalpine snowgrass
            } else {
                ([0.34, 0.42, 0.26, 0.95], 36, 0.24) // Olive-green alpine tussock
            }
        } else if site == 2 || site == 3 {
            // Creek and river riparian grasses
            if i % 3 == 0 {
                ([0.22, 0.38, 0.24, 0.95], 32, 0.26)
            } else {
                ([0.18, 0.34, 0.20, 0.95], 28, 0.20)
            }
        } else {
            // Valley and lake foreshore grasses
            if i % 3 == 0 {
                ([0.24, 0.38, 0.22, 0.95], 30, 0.22)
            } else {
                ([0.20, 0.35, 0.18, 0.95], 28, 0.18)
            }
        };

        out.push(plant(
            format!("rc:asset/talbingo/{site}/understory-{i}"),
            patch_color,
            LIVING_FOLIAGE,
            ParametricRecipe::VegetationPatch {
                center: [x, 0.02, z],
                radius: 0.42 + (i % 4) as f32 * 0.07,
                density,
                blade_height: blade_h + (i % 3) as f32 * 0.05,
            },
        ));
    }

    out
}
