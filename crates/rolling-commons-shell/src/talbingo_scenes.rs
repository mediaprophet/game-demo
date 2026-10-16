//! Illustrative Talbingo camping play spaces. Geometry is authored here and
//! compiled to verified .10d assets by the pinned QualiaDB asset pipeline.
use crate::asset_catalog::{asset, block, AssetRecipe};
use crate::resident_animation::AssetRigBinding;
use qualia_core_db::container_10d::skeletal_section::ClipRecord;
use qualia_core_db::render::scene_primitives::Primitive;
use qualia_core_db::render::skeletal::{Joint, Key, Motor, SkinWeight, Track};
use wasm_bindgen::prelude::*;

pub(crate) fn road_z(site: u8, x: f32) -> f32 {
    let base = if x < 6.0 {
        0.48 - 0.25 * (((x + 6.1) / 12.1) * std::f32::consts::TAU).sin()
    } else if x < 9.0 {
        0.48
    } else if x < 9.6 {
        0.48 + (x - 9.0) * 0.12
    } else {
        0.54 - 0.29 * (((x - 9.6) / 12.7).clamp(0.0, 1.0) * std::f32::consts::TAU).sin()
    };
    let t = ((x + 5.45) / 27.1).clamp(0.0, 1.0);
    let bend = match site {
        1 => 0.55 * (t * std::f32::consts::TAU).sin(),
        2 => -0.72 * (t * std::f32::consts::PI).sin(),
        3 => 0.80 * (t * std::f32::consts::PI).sin(),
        4 => 1.03 * (t * std::f32::consts::TAU).sin(),
        5 => -0.63 * (t * std::f32::consts::TAU * 2.0).sin(),
        _ => 0.0,
    };
    base + bend
}

fn push(
    out: &mut Vec<AssetRecipe>,
    name: &str,
    color: [f32; 4],
    parts: Vec<qualia_core_db::render::scene_primitives::Primitive>,
) {
    out.push(asset(name, color, parts));
}

fn scene(site: u8) -> Vec<AssetRecipe> {
    let mut out = Vec::new();
    let soil = match site {
        3 | 5 => [0.09, 0.15, 0.075, 1.0],
        4 => [0.085, 0.14, 0.065, 1.0],
        _ => [0.075, 0.18, 0.06, 1.0],
    };
    push(
        &mut out,
        "talbingo-ground",
        soil,
        vec![block([8.2, -0.16, 0.0], [31.0, 0.28, 17.0])],
    );
    let mut roadway = Vec::new();
    let mut verge = Vec::new();
    for i in 0..58 {
        let x = -5.7 + i as f32 * 0.49;
        let z = road_z(site, x);
        roadway.push(block([x, 0.01, z], [0.51, 0.045, 1.55]));
        if i % 4 == 0 {
            verge.push(block([x, 0.037, z], [0.22, 0.014, 0.035]));
        }
    }
    push(
        &mut out,
        "talbingo-road",
        [0.035, 0.042, 0.045, 1.0],
        roadway,
    );
    push(
        &mut out,
        "talbingo-road-markers",
        [0.96, 0.74, 0.29, 1.0],
        verge,
    );
    let event_z = road_z(site, 8.0);
    push(
        &mut out,
        "talbingo-road-event-post",
        [0.18, 0.14, 0.10, 1.0],
        vec![block([8.0, 0.55, event_z + 1.2], [0.12, 1.1, 0.12])],
    );
    push(
        &mut out,
        "talbingo-road-event-sign",
        [0.92, 0.26, 0.10, 1.0],
        vec![block([8.0, 1.16, event_z + 1.2], [0.74, 0.62, 0.08])],
    );
    // Roadside native wildlife encounter matching the site journey:
    match site {
        0 => out.extend(crate::fauna::macropod(
            &format!("talbingo/{site}/roadside-kangaroo"),
            [8.0, 0.0, event_z + 1.35],
            false,
        )),
        1 => out.extend(crate::fauna::macropod(
            &format!("talbingo/{site}/roadside-wallaby"),
            [8.0, 0.0, event_z + 1.35],
            true,
        )),
        2 => out.extend(crate::fauna::wombat(
            &format!("talbingo/{site}/roadside-wombat"),
            [8.0, 0.0, event_z + 1.25],
        )),
        3 => out.extend(crate::fauna::macropod(
            &format!("talbingo/{site}/roadside-wallaby"),
            [8.0, 0.0, event_z + 1.35],
            true,
        )),
        4 => out.extend(crate::fauna::macropod(
            &format!("talbingo/{site}/roadside-wallaby"),
            [8.0, 0.0, event_z + 1.35],
            true,
        )),
        _ => out.extend(crate::fauna::horse(
            &format!("talbingo/{site}/roadside-brumby"),
            [8.0, 0.0, event_z + 1.45],
        )),
    }
    // A destination gate and camp spur make the arrival leg readable.
    let camp_shift = road_z(site, 18.7) - road_z(0, 18.7);
    push(
        &mut out,
        "talbingo-camp-spur",
        [0.42, 0.32, 0.20, 1.0],
        vec![
            block([18.7, 0.015, 2.0 + camp_shift], [2.9, 0.04, 3.3]),
            block([19.8, 0.02, 3.8 + camp_shift], [5.3, 0.05, 2.3]),
        ],
    );
    push(
        &mut out,
        "talbingo-arrival-gate",
        [0.82, 0.30, 0.09, 1.0],
        vec![
            block([16.9, 0.8, 2.0 + camp_shift], [0.16, 1.6, 0.16]),
            block([16.9, 0.8, 4.5 + camp_shift], [0.16, 1.6, 0.16]),
            block([16.9, 1.56, 3.25 + camp_shift], [0.2, 0.14, 2.6]),
        ],
    );
    // Compact player vehicle. Qualia's skeletal actor root moves these parts
    // together; the browser never redraws or recompiles meshes per frame.
    push(
        &mut out,
        "rc:asset/rig/campervan/talbingo-chassis",
        [0.94, 0.36, 0.08, 1.0],
        vec![
            block([2.35, 0.37, 4.65], [0.92, 0.35, 1.55]),
            block([2.35, 0.68, 4.85], [0.78, 0.30, 0.77]),
        ],
    );
    push(
        &mut out,
        "rc:asset/rig/campervan/talbingo-windows",
        [0.035, 0.11, 0.18, 1.0],
        vec![
            block([2.35, 0.70, 4.43], [0.65, 0.20, 0.08]),
            block([1.94, 0.70, 4.88], [0.06, 0.20, 0.44]),
            block([2.76, 0.70, 4.88], [0.06, 0.20, 0.44]),
        ],
    );
    let mut wheels = Vec::new();
    for x in [1.82, 2.88] {
        for z in [4.16, 5.16] {
            wheels.push(block([x, 0.19, z], [0.14, 0.38, 0.31]));
        }
    }
    push(
        &mut out,
        "rc:asset/rig/campervan/talbingo-wheels",
        [0.13, 0.16, 0.16, 1.0],
        wheels,
    );
    push(
        &mut out,
        "rc:asset/talbingo/walker-coat",
        [0.79, 0.14, 0.08, 1.0],
        vec![
            block([18.0, 0.76, 3.5], [0.35, 0.72, 0.25]),
            block([17.88, 0.26, 3.5], [0.12, 0.52, 0.16]),
            block([18.12, 0.26, 3.5], [0.12, 0.52, 0.16]),
        ],
    );
    push(
        &mut out,
        "rc:asset/talbingo/walker-head",
        [0.69, 0.43, 0.26, 1.0],
        vec![block([18.0, 1.25, 3.5], [0.25, 0.28, 0.23])],
    );
    out.extend(crate::talbingo_vegetation::assets(site));
    let mut tent_roofs = Vec::new();
    let mut tent_floors = Vec::new();
    let mut tent_doors = Vec::new();
    let mut tent_poles = Vec::new();
    for (x, z) in [(18.0, 6.25), (21.2, 6.2), (23.3, 3.6)] {
        let z = z + camp_shift;
        tent_floors.push(block([x, 0.085, z], [2.15, 0.13, 1.8]));
        tent_roofs.push(Primitive::Roof {
            center: [x, 0.13, z],
            size: [2.05, 1.28, 1.72],
        });
        tent_doors.push(Primitive::Roof {
            center: [x - 1.032, 0.14, z],
            size: [0.025, 0.98, 1.1],
        });
        for side in [-1.0, 1.0] {
            tent_poles.push(block([x + side * 1.08, 0.60, z], [0.025, 1.16, 0.025]));
            tent_poles.push(block([x + side * 1.36, 0.035, z], [0.11, 0.07, 0.11]));
        }
    }
    push(
        &mut out,
        "talbingo-tent-canvas",
        [0.77, 0.32, 0.14, 1.0],
        tent_roofs,
    );
    push(
        &mut out,
        "talbingo-tent-doors",
        [0.38, 0.15, 0.10, 1.0],
        tent_doors,
    );
    push(
        &mut out,
        "talbingo-tent-groundsheet",
        [0.12, 0.17, 0.13, 1.0],
        tent_floors,
    );
    push(
        &mut out,
        "talbingo-tent-poles",
        [0.64, 0.67, 0.62, 1.0],
        tent_poles,
    );
    match site {
        // The Pines: broad reservoir shore, picnic lawn, long sight line and grazing kangaroos.
        0 => {
            push(
                &mut out,
                "pines-reservoir",
                [0.025, 0.19, 0.38, 1.0],
                vec![block([7.0, -0.015, -6.4], [26.0, 0.025, 3.7])],
            );
            push(
                &mut out,
                "pines-picnic-tables",
                [0.38, 0.19, 0.07, 1.0],
                vec![
                    block([20.0, 0.46, -2.4], [1.5, 0.1, 0.8]),
                    block([22.5, 0.46, -2.8], [1.5, 0.1, 0.8]),
                ],
            );
            out.extend(crate::fauna::macropod(
                &format!("talbingo/{site}/camp-kangaroo"),
                [19.2, 0.0, -1.8],
                false,
            ));
        }
        // Humes Crossing: narrow inlet, timber crossing and swamp wallaby habitat.
        1 => {
            push(
                &mut out,
                "humes-inlet",
                [0.025, 0.22, 0.42, 1.0],
                vec![
                    block([10.8, -0.01, -5.9], [8.0, 0.03, 3.3]),
                    block([14.7, -0.01, -4.0], [2.4, 0.03, 7.2]),
                ],
            );
            push(
                &mut out,
                "humes-crossing-deck",
                [0.32, 0.17, 0.08, 1.0],
                vec![
                    block([14.7, 0.15, -3.9], [3.5, 0.13, 1.15]),
                    block([14.7, 0.42, -4.55], [3.5, 0.1, 0.12]),
                ],
            );
            out.extend(crate::fauna::macropod(
                &format!("talbingo/{site}/camp-wallaby"),
                [13.2, 0.0, -2.6],
                true,
            ));
        }
        // Jounama Creek: creek crossing, boulders, montane bottlebrush and resident bare-nosed wombats.
        2 => {
            push(
                &mut out,
                "jounama-creek",
                [0.04, 0.26, 0.40, 1.0],
                vec![
                    block([15.5, -0.01, -5.6], [13.0, 0.025, 1.5]),
                    block([22.0, -0.01, -3.8], [2.0, 0.025, 5.0]),
                ],
            );
            push(
                &mut out,
                "jounama-boulders",
                [0.52, 0.52, 0.46, 1.0],
                vec![
                    block([18.1, 0.35, -3.1], [1.1, 0.7, 1.0]),
                    block([20.1, 0.24, -3.8], [0.8, 0.48, 0.9]),
                    block([23.4, 0.31, -2.0], [0.9, 0.62, 1.0]),
                ],
            );
            out.extend(crate::fauna::wombat(
                &format!("talbingo/{site}/camp-wombat"),
                [21.5, 0.0, -2.4],
            ));
        }
        // Yarrangobilly Village: riverbank, historic hut, river bottlebrush, kookaburras and deer.
        3 => {
            push(
                &mut out,
                "yarrangobilly-river",
                [0.04, 0.22, 0.40, 1.0],
                vec![block([10.5, -0.01, -6.4], [25.0, 0.025, 2.7])],
            );
            push(
                &mut out,
                "yarrangobilly-hut",
                [0.68, 0.58, 0.42, 1.0],
                vec![
                    block([20.3, 0.85, -2.7], [2.5, 1.7, 2.0]),
                    block([20.3, 1.83, -2.7], [2.9, 0.3, 2.4]),
                ],
            );
            out.extend(crate::fauna::bird(
                &format!("talbingo/{site}/camp-kookaburra"),
                [20.3, 1.95, -1.6],
                true,
            ));
            out.extend(crate::fauna::deer(
                &format!("talbingo/{site}/camp-deer"),
                [15.0, 0.0, -4.5],
            ));
        }
        // O'Hares: steep alpine ash slopes, reservoir lookout and basking lace monitors.
        4 => {
            push(
                &mut out,
                "ohares-ridge",
                [0.19, 0.27, 0.11, 1.0],
                vec![
                    block([6.0, 0.47, -5.9], [17.0, 0.95, 4.3]),
                    block([18.8, 0.80, -6.1], [9.2, 1.6, 4.0]),
                ],
            );
            push(
                &mut out,
                "ohares-reservoir",
                [0.02, 0.16, 0.38, 1.0],
                vec![block([9.0, -0.01, 7.0], [25.0, 0.025, 2.4])],
            );
            push(
                &mut out,
                "ohares-lookout",
                [0.60, 0.38, 0.20, 1.0],
                vec![block([21.5, 1.72, -5.1], [2.9, 0.1, 1.7])],
            );
            out.extend(crate::fauna::goanna(
                &format!("talbingo/{site}/camp-goanna"),
                [21.8, 1.74, -4.8],
            ));
        }
        // Three Mile Dam: open alpine grass, shallow dam, gnarled snow gums, sphagnum marsh and Kiandra brumbies.
        _ => {
            push(
                &mut out,
                "three-mile-dam",
                [0.04, 0.21, 0.40, 1.0],
                vec![block([7.5, -0.01, -6.5], [18.0, 0.025, 2.9])],
            );
            push(
                &mut out,
                "three-mile-marsh",
                [0.68, 0.70, 0.44, 1.0],
                vec![
                    block([18.0, 0.04, -3.1], [6.0, 0.03, 1.6]),
                    block([21.5, 0.04, -4.6], [3.0, 0.03, 1.5]),
                ],
            );
            push(
                &mut out,
                "three-mile-boulders",
                [0.65, 0.66, 0.62, 1.0],
                vec![
                    block([19.0, 0.31, -2.6], [1.0, 0.62, 0.8]),
                    block([23.0, 0.38, -3.7], [1.1, 0.76, 0.9]),
                ],
            );
            out.extend(crate::fauna::horse(
                &format!("talbingo/{site}/camp-brumby"),
                [20.5, 0.0, -1.8],
            ));
        }
    }
    out
}

pub(crate) fn build(site: u8) -> Result<JsValue, JsValue> {
    if site > 5 {
        return Err(JsValue::from_str("unknown Talbingo scene"));
    }
    let assets = js_sys::Array::new();
    for recipe in scene(site) {
        let (_, record) = crate::asset_compile::compile_asset(&recipe)?;
        assets.push(&record);
    }
    let result = js_sys::Object::new();
    let textures = crate::material_library::texture_receipts()?;
    js_sys::Reflect::set(&result, &"assets".into(), &assets)?;
    js_sys::Reflect::set(&result, &"textures".into(), &textures)?;
    Ok(result.into())
}

pub(crate) fn walker_root() -> u64 {
    qualia_core_db::q_hash("rc:talbingo/walker/root")
}

pub(crate) fn author_walker(
    recipe: &AssetRecipe,
    positions: &[[f32; 3]],
) -> Option<AssetRigBinding> {
    if !recipe.id.starts_with("rc:asset/talbingo/walker-") {
        return None;
    }
    let origin = [18.0, 0.0, 3.5];
    let rest = Motor::from_translation(origin[0], origin[1], origin[2]);
    Some(AssetRigBinding {
        joints: vec![Joint {
            parent: None,
            rest_local: rest,
            inverse_bind: Motor::from_translation(-origin[0], -origin[1], -origin[2]),
        }],
        ids: vec![walker_root()],
        skin: vec![
            SkinWeight {
                joints: [0, 0, 0, 0],
                weights: [1.0, 0.0, 0.0, 0.0]
            };
            positions.len()
        ],
        clips: vec![ClipRecord {
            id: qualia_core_db::q_hash("rc:clip/talbingo/walker-idle"),
            duration: 1.0,
            first_track: 0,
            track_count: 1,
            looping: true,
        }],
        tracks: vec![Track {
            joint: 0,
            first_key: 0,
            key_count: 2,
        }],
        keys: vec![
            Key {
                time: 0.0,
                local: rest,
            },
            Key {
                time: 1.0,
                local: rest,
            },
        ],
    })
}
