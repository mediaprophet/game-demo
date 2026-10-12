//! Macropod articulation authored as SKL1 data, played by Qualia's common
//! native actor lifecycle. Idle clips do not relocate road-hazard encounters.
use crate::asset_catalog::{AssetRecipe, ParametricRecipe};
use crate::resident_animation::AssetRigBinding;
use qualia_core_db::container_10d::skeletal_section::ClipRecord;
use qualia_core_db::render::skeletal::authoring::{weights_from_segments_into, BoneSegment};
use qualia_core_db::render::skeletal::{Joint, Key, Motor, SkinWeight, Track};
const NAMES: [&str; 18] = [
    "root",
    "pelvis",
    "spine",
    "neck",
    "head",
    "left-ear",
    "right-ear",
    "left-shoulder",
    "left-elbow",
    "left-hand",
    "right-shoulder",
    "right-elbow",
    "right-hand",
    "left-hip",
    "left-knee",
    "right-hip",
    "right-knee",
    "tail",
];
const PARENTS: [Option<u16>; 18] = [
    None,
    Some(0),
    Some(1),
    Some(2),
    Some(3),
    Some(4),
    Some(4),
    Some(2),
    Some(7),
    Some(8),
    Some(2),
    Some(10),
    Some(11),
    Some(1),
    Some(13),
    Some(1),
    Some(15),
    Some(1),
];
pub(crate) fn author(
    recipe: &AssetRecipe,
    positions: &[[f32; 3]],
) -> Result<Option<AssetRigBinding>, String> {
    let Some(id) = recipe.id.strip_prefix("rc:asset/fauna/") else {
        return Ok(None);
    };
    let Some((animal, part)) = id.split_once('/') else {
        return Ok(None);
    };
    if !animal.contains("kangaroo") && !animal.contains("wallaby") {
        return Ok(None);
    }
    let Some(ParametricRecipe::Transform {
        target: origin,
        scale,
        ..
    }) = &recipe.parametric
    else {
        return Err("macropod placement missing".into());
    };
    let local = [
        [0.0, 0.0, 0.0],
        [0.0, 0.91, 0.18],
        [0.0, 1.36, -0.17],
        [0.0, 1.60, -0.33],
        [0.0, 1.77, -0.49],
        [-0.115, 1.93, -0.49],
        [0.115, 1.93, -0.49],
        [-0.21, 1.36, -0.28],
        [-0.24, 1.14, -0.34],
        [-0.17, 1.03, -0.46],
        [0.21, 1.36, -0.28],
        [0.24, 1.14, -0.34],
        [0.17, 1.03, -0.46],
        [-0.23, 0.91, 0.18],
        [-0.28, 0.48, -0.30],
        [0.23, 0.91, 0.18],
        [0.28, 0.48, -0.30],
        [0.0, 0.91, 0.32],
    ];
    let points: Vec<[f32; 3]> = local
        .iter()
        .map(|p| std::array::from_fn(|i| origin[i] + p[i] * scale[i]))
        .collect();
    let joints: Vec<Joint> = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let local = match PARENTS[i] {
                None => *p,
                Some(parent) => std::array::from_fn(|a| p[a] - points[parent as usize][a]),
            };
            Joint {
                parent: PARENTS[i],
                rest_local: Motor::from_translation(
                    local[0] as f64,
                    local[1] as f64,
                    local[2] as f64,
                ),
                inverse_bind: Motor::from_translation(-p[0] as f64, -p[1] as f64, -p[2] as f64),
            }
        })
        .collect();
    let links: &[(usize, usize)] = match part {
        "torso" | "chest" => &[(1, 2)],
        "neck" => &[(3, 4)],
        "head" | "muzzle" | "nose" | "eye-left" | "eye-right" => &[(4, 4)],
        "ear-left" | "ear-inner-left" => &[(5, 5)],
        "ear-right" | "ear-inner-right" => &[(6, 6)],
        "arm-left" => &[(7, 8), (8, 9)],
        "arm-right" => &[(10, 11), (11, 12)],
        "hind-leg-left" => &[(13, 14), (14, 14)],
        "hind-leg-right" => &[(15, 16), (16, 16)],
        "foot-left" => &[(14, 14)],
        "foot-right" => &[(16, 16)],
        "tail" => &[(17, 17)],
        _ => return Err(format!("macropod part has no binding: {part}")),
    };
    let segments: Vec<BoneSegment> = links
        .iter()
        .map(|(a, b)| BoneSegment {
            joint: *a as u16,
            start: points[*a],
            end: points[*b],
        })
        .collect();
    let mut skin = vec![
        SkinWeight {
            joints: [0; 4],
            weights: [1.0, 0.0, 0.0, 0.0]
        };
        positions.len()
    ];
    weights_from_segments_into(
        positions,
        &segments,
        joints.len(),
        if links.len() > 1 { 2 } else { 1 },
        &mut skin,
    )
    .map_err(|e| format!("macropod skin: {e:?}"))?;
    let ids = NAMES
        .iter()
        .map(|joint| qualia_core_db::q_hash(&format!("rc:rig/fauna/{animal}/{joint}")))
        .collect();
    let mut result = AssetRigBinding {
        joints,
        ids,
        skin,
        clips: Vec::new(),
        tracks: Vec::new(),
        keys: Vec::new(),
    };
    for (name, duration) in [("idle", 4.8), ("alert", 2.4), ("hop", 0.9)] {
        let first_track = result.tracks.len() as u32;
        for joint in 0..result.joints.len() {
            let first_key = result.keys.len() as u32;
            for frame in 0..=8 {
                let phase = frame as f64 / 8.0;
                result.keys.push(Key {
                    time: phase * duration,
                    local: pose(
                        name,
                        joint,
                        phase,
                        result.joints[joint].rest_local,
                        scale[1] as f64,
                    ),
                });
            }
            result.tracks.push(Track {
                joint: joint as u16,
                first_key,
                key_count: 9,
            });
        }
        result.clips.push(ClipRecord {
            id: qualia_core_db::q_hash(&format!("rc:clip/macropod/{name}")),
            duration,
            first_track,
            track_count: result.joints.len() as u32,
            looping: true,
        });
    }
    Ok(Some(result))
}
fn pose(clip: &str, joint: usize, phase: f64, rest: Motor, scale: f64) -> Motor {
    let wave = (phase * std::f64::consts::TAU).sin();
    let pulse = (1.0 - (phase * std::f64::consts::TAU).cos()) * 0.5;
    let mut angles = [0.0_f64; 3];
    let (_, mut translation) = rest.to_rotation_translation();
    match clip {
        "idle" => match joint {
            2 => translation[1] += wave * 0.004 * scale,
            4 => angles[1] = wave * 0.05,
            5 => angles[2] = wave * 0.07,
            6 => angles[2] = -wave * 0.06,
            17 => angles[1] = wave * 0.02,
            _ => {}
        },
        "alert" => match joint {
            3 => angles[0] = -0.06,
            4 => angles[1] = wave * 0.18,
            5 => angles[2] = 0.08 + wave * 0.05,
            6 => angles[2] = -0.08 - wave * 0.05,
            _ => {}
        },
        "hop" => match joint {
            0 => translation[1] += pulse * 0.22 * scale,
            13 | 15 => angles[0] = wave * 0.24,
            14 | 16 => angles[0] = -wave * 0.35,
            7 | 10 => angles[0] = -pulse * 0.12,
            17 => angles[0] = -pulse * 0.07,
            _ => {}
        },
        _ => {}
    }
    let mut rotation = Motor::identity();
    for (axis, angle) in angles.into_iter().enumerate() {
        let half = angle * 0.5;
        let mut q = [half.cos(), 0.0, 0.0, 0.0];
        q[axis + 1] = half.sin();
        rotation = rotation.mul(&Motor::from_rotation_translation(q, [0.0; 3]));
    }
    let (q, _) = rotation.to_rotation_translation();
    Motor::from_rotation_translation(q, translation)
}

pub(crate) fn present_alerts(
    portal: &mut crate::GamePortal,
    state: &serde_json::Value,
) -> Result<(), wasm_bindgen::JsValue> {
    const GROUPS: [(&str, u64, bool); 6] = [
        (
            "kangaroo-01",
            qualia_core_db::q_hash("rc:rig/fauna/road-kangaroo-01/root"),
            true,
        ),
        (
            "wallaby-01",
            qualia_core_db::q_hash("rc:rig/fauna/road-wallaby-01/root"),
            true,
        ),
        (
            "goat-01",
            qualia_core_db::q_hash("rc:rig/fauna/inland-goat-01/root"),
            false,
        ),
        (
            "horse-01",
            qualia_core_db::q_hash("rc:rig/fauna/inland-horse-01/root"),
            false,
        ),
        (
            "deer-01",
            qualia_core_db::q_hash("rc:rig/fauna/road-deer-01/root"),
            false,
        ),
        (
            "wombat-01",
            qualia_core_db::q_hash("rc:rig/fauna/road-wombat-01/root"),
            false,
        ),
    ];
    let warning = state["warning"].as_str().unwrap_or("");
    let near = matches!(warning, "hazard-near" | "safe-stop" | "emergency-stop");
    for (encounter, joint, macropod) in GROUPS {
        let alert = near && state["hazard_id"].as_str() == Some(encounter);
        let clip = match (macropod, alert) {
            (true, true) => qualia_core_db::q_hash("rc:clip/macropod/alert"),
            (true, false) => qualia_core_db::q_hash("rc:clip/macropod/idle"),
            (false, true) => qualia_core_db::q_hash("rc:clip/quadruped/alert"),
            (false, false) => qualia_core_db::q_hash("rc:clip/quadruped/idle"),
        };
        portal
            .inner
            .select_scene_animation_clip(joint, clip, 0.25)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "fauna_animation_tests.rs"]
mod tests;
