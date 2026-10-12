//! Authored resident rigs and activity clips; Qualia owns weights and playback.
use crate::asset_catalog::{community_work_pose, AssetRecipe, ParametricRecipe};
use qualia_core_db::container_10d::skeletal_section::{
    encode_skeletal_section_into, ClipRecord, Counts, SkeletalData,
};
use qualia_core_db::render::skeletal::authoring::{
    remap_skin_weights_into, weights_from_segments_into, BoneSegment,
};
use qualia_core_db::render::skeletal::{Joint, Key, Motor, SkinWeight, Track};

const PARENTS: [Option<u16>; 18] = [
    None,
    Some(0),
    Some(1),
    Some(2),
    Some(3),
    Some(4),
    Some(3),
    Some(6),
    Some(7),
    Some(3),
    Some(9),
    Some(10),
    Some(1),
    Some(12),
    Some(13),
    Some(1),
    Some(15),
    Some(16),
];
const NAMES: [&str; 18] = [
    "root",
    "pelvis",
    "spine",
    "chest",
    "neck",
    "head",
    "left-shoulder",
    "left-elbow",
    "left-hand",
    "right-shoulder",
    "right-elbow",
    "right-hand",
    "left-hip",
    "left-knee",
    "left-ankle",
    "right-hip",
    "right-knee",
    "right-ankle",
];
const CLIPS: [(&str, f64); 8] = [
    ("idle", 4.0),
    ("walk", 1.2),
    ("repair", 1.4),
    ("plant", 2.8),
    ("harvest", 2.1),
    ("carry", 1.8),
    ("talk", 3.2),
    ("rest", 5.0),
];

pub(crate) struct AssetRigBinding {
    pub(crate) joints: Vec<Joint>,
    pub(crate) ids: Vec<u64>,
    pub(crate) skin: Vec<SkinWeight>,
    pub(crate) clips: Vec<ClipRecord>,
    pub(crate) tracks: Vec<Track>,
    pub(crate) keys: Vec<Key>,
}
pub(crate) type ResidentBinding = AssetRigBinding;

fn offset(origin: [f32; 3], p: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|a| origin[a] + p[a])
}
fn arm_point(p: [f32; 3]) -> [f32; 3] {
    [p[0] * 0.74, 0.815 + (p[1] - 0.48) * 1.04, p[2] * 0.74]
}

pub fn author(
    recipe: &AssetRecipe,
    positions: &[[f32; 3]],
) -> Result<Option<ResidentBinding>, String> {
    let Some(name) = recipe.id.strip_prefix("rc:asset/resident-") else {
        return Ok(None);
    };
    let Some((resident, part)) = name.split_once('-') else {
        return Err("resident part identity missing".into());
    };
    let index = ["ada", "bo", "cam", "dev", "eve", "finn"]
        .iter()
        .position(|key| *key == resident)
        .ok_or_else(|| "resident rig identity unknown".to_owned())?;
    let Some(ParametricRecipe::Transform { target, .. }) = recipe.parametric.as_ref() else {
        return Err("resident placement transform missing".into());
    };
    let ground = target[1]
        - match part {
            "head" | "face" | "hair" | "nose" => 1.64,
            "pants" => 0.10,
            "boots" => 0.0,
            _ => 0.815,
        };
    let origin = [target[0], ground, target[2]];
    let (le, lh, re, rh) = community_work_pose(index);
    let points = [
        [0.0, 0.0, 0.0],
        [0.0, 0.815, 0.0],
        [0.0, 1.08, 0.0],
        [0.0, 1.283, 0.0],
        [0.0, 1.3922, 0.0],
        [0.0, 1.64, 0.0],
        [-0.2146, 1.283, 0.0],
        arm_point(le),
        arm_point(lh),
        [0.2146, 1.283, 0.0],
        arm_point(re),
        arm_point(rh),
        [-0.0888, 0.815, 0.0],
        [-0.0925, 0.455, 0.0148],
        [-0.0962, 0.10, 0.0296],
        [0.0888, 0.815, 0.0],
        [0.0925, 0.455, 0.0148],
        [0.0962, 0.10, 0.0296],
    ]
    .map(|p| offset(origin, p));
    let joints: Vec<Joint> = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let local = match PARENTS[i] {
                Some(parent) => std::array::from_fn(|a| p[a] - points[parent as usize][a]),
                None => *p,
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
    let ids = NAMES
        .iter()
        .map(|joint| qualia_core_db::q_hash(&format!("rc:rig/resident-{resident}/{joint}")))
        .collect();
    let links: &[(usize, usize)] = match part {
        p if p.starts_with("attachment-left-") => &[(8, 8)],
        p if p.starts_with("attachment-right-") => &[(11, 11)],
        "head" | "face" | "hair" | "nose" => &[(5, 5)],
        "neck" => &[(4, 4)],
        "coat" => &[(1, 2), (2, 3), (3, 4)],
        "arms" => &[(6, 7), (7, 8), (9, 10), (10, 11)],
        "hands" => &[(8, 8), (11, 11)],
        "pants" => &[(12, 13), (13, 14), (15, 16), (16, 17)],
        "boots" => &[(14, 14), (17, 17)],
        _ => return Err(format!("resident part has no skin authoring rule: {part}")),
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
    let influences = if matches!(part, "coat" | "arms" | "pants") {
        2
    } else {
        1
    };
    weights_from_segments_into(positions, &segments, joints.len(), influences, &mut skin)
        .map_err(|e| format!("resident weights: {e:?}"))?;
    let mut result = ResidentBinding {
        joints,
        ids,
        skin,
        clips: Vec::new(),
        tracks: Vec::new(),
        keys: Vec::new(),
    };
    for (name, duration) in CLIPS {
        let first_track = result.tracks.len() as u32;
        for joint in 0..result.joints.len() {
            let first_key = result.keys.len() as u32;
            for frame in 0..=4 {
                let phase = frame as f64 / 4.0;
                result.keys.push(Key {
                    time: phase * duration,
                    local: pose(name, joint, phase, result.joints[joint].rest_local),
                });
            }
            result.tracks.push(Track {
                joint: joint as u16,
                first_key,
                key_count: 5,
            });
        }
        result.clips.push(ClipRecord {
            id: qualia_core_db::q_hash(&format!("rc:clip/resident/{name}")),
            duration,
            first_track,
            track_count: result.joints.len() as u32,
            looping: true,
        });
    }
    Ok(Some(result))
}

fn pose(clip: &str, joint: usize, phase: f64, rest: Motor) -> Motor {
    let wave = (phase * std::f64::consts::TAU).sin();
    let pulse = 0.5 - 0.5 * (phase * std::f64::consts::TAU).cos();
    let mut angle = [0.0f64; 3];
    let mut shift = [0.0f64; 3];
    match clip {
        "idle" => {
            if joint == 3 {
                shift[1] = wave * 0.004;
            }
            if joint == 5 {
                angle[1] = wave * 2.0;
            }
        }
        "walk" => match joint {
            12 => angle[0] = wave * 27.0,
            15 => angle[0] = -wave * 27.0,
            13 => angle[0] = pulse * 34.0,
            16 => angle[0] = (1.0 - pulse) * 34.0,
            6 => angle[0] = -wave * 16.0,
            9 => angle[0] = wave * 16.0,
            1 => shift[1] = pulse * 0.025,
            _ => {}
        },
        "repair" => match joint {
            2 => angle[0] = 9.0,
            9 => angle[0] = -12.0 + wave * 7.0,
            10 => angle[0] = -20.0 + wave * 13.0,
            11 => angle[0] = wave * 5.0,
            5 => angle[0] = 10.0,
            _ => {}
        },
        "plant" => match joint {
            2 => angle[0] = 16.0 + pulse * 7.0,
            6 | 9 => angle[0] = -8.0,
            7 => angle[0] = wave * 10.0,
            10 => angle[0] = -wave * 14.0,
            5 => angle[0] = 12.0,
            _ => {}
        },
        "harvest" => match joint {
            2 => angle[0] = 8.0 + pulse * 4.0,
            6 => angle[0] = -9.0 + wave * 9.0,
            9 => angle[0] = -12.0 - wave * 9.0,
            7 | 10 => angle[0] = -8.0 + pulse * 12.0,
            _ => {}
        },
        "carry" => match joint {
            6 | 9 => angle[0] = -7.0,
            7 | 10 => angle[0] = -12.0,
            2 => angle[0] = 3.0,
            12 => angle[0] = wave * 9.0,
            15 => angle[0] = -wave * 9.0,
            _ => {}
        },
        "talk" => match joint {
            5 => angle[1] = wave * 8.0,
            9 => angle[2] = -pulse * 22.0,
            10 => angle[0] = -pulse * 12.0,
            6 => angle[2] = pulse * 5.0,
            _ => {}
        },
        "rest" => match joint {
            2 => angle[0] = -4.0,
            5 => angle[0] = wave * 2.0,
            3 => shift[1] = wave * 0.003,
            _ => {}
        },
        _ => {}
    }
    let (_, mut translation) = rest.to_rotation_translation();
    for a in 0..3 {
        translation[a] += shift[a];
    }
    let mut rotation = Motor::identity();
    for (axis, degrees) in angle.iter().enumerate() {
        let half = degrees.to_radians() * 0.5;
        let mut quaternion = [half.cos(), 0.0, 0.0, 0.0];
        quaternion[axis + 1] = half.sin();
        rotation = rotation.mul(&Motor::from_rotation_translation(quaternion, [0.0; 3]));
    }
    let (quaternion, _) = rotation.to_rotation_translation();
    Motor::from_rotation_translation(quaternion, translation)
}

pub fn attach(
    binding: ResidentBinding,
    lineage: &[u32],
    container: &[u8],
) -> Result<Vec<u8>, String> {
    let mut skin = vec![
        SkinWeight {
            joints: [0; 4],
            weights: [1.0, 0.0, 0.0, 0.0]
        };
        lineage.len()
    ];
    remap_skin_weights_into(&binding.skin, lineage, &mut skin)
        .map_err(|e| format!("resident seam weights: {e:?}"))?;
    let counts = Counts {
        joints: binding.joints.len(),
        vertices: skin.len(),
        clips: binding.clips.len(),
        tracks: binding.tracks.len(),
        keys: binding.keys.len(),
    };
    let mut payload = vec![
        0;
        counts
            .encoded_len()
            .ok_or_else(|| "resident animation budget exceeded".to_owned())?
    ];
    let data = SkeletalData {
        joints: &binding.joints,
        joint_ids: &binding.ids,
        skin: &skin,
        clips: &binding.clips,
        tracks: &binding.tracks,
        keys: &binding.keys,
    };
    encode_skeletal_section_into(&data, &mut payload)
        .map_err(|e| format!("resident SKL1: {e:?}"))?;
    use qualia_core_db::container_10d::{section::SectionTableError, skeletal_container::*};
    let needed = match attach_skeletal_section_into(container, &payload, &mut []) {
        Err(SkeletalContainerError::Sections(SectionTableError::OutputBufferTooSmall {
            needed,
            ..
        })) => needed,
        Err(e) => return Err(format!("resident container: {e:?}")),
        Ok(n) => n,
    };
    let mut bytes = vec![0; needed];
    let written = attach_skeletal_section_into(container, &payload, &mut bytes)
        .map_err(|e| format!("resident container: {e:?}"))?;
    bytes.truncate(written);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn carried_prop_uses_one_wrist_without_deforming_its_structure() {
        let assets = crate::asset_catalog::kestrel_flats(0, 0, false, false, false, false, false);
        let prop = assets
            .iter()
            .find(|asset| {
                asset.id == "rc:asset/resident-finn-attachment-right-produce-timber-crate"
            })
            .unwrap();
        let mesh = prop.parametric.as_ref().unwrap().compile().unwrap();
        let binding = author(prop, &mesh.positions).unwrap().unwrap();
        assert!(binding
            .skin
            .iter()
            .all(|skin| skin.joints[0] == 11 && skin.weights == [1.0, 0.0, 0.0, 0.0]));
        let carry = binding
            .clips
            .iter()
            .find(|clip| clip.id == qualia_core_db::q_hash("rc:clip/resident/carry"))
            .unwrap();
        let mut local = vec![Motor::identity(); 18];
        let mut global = local.clone();
        let mut palette = local.clone();
        qualia_core_db::render::skeletal::sample_pose_into(
            &binding.joints,
            qualia_core_db::container_10d::skeletal_section::clip_view(
                *carry,
                &binding.tracks,
                &binding.keys,
            )
            .unwrap(),
            0.45,
            None,
            1.0,
            &mut local,
        )
        .unwrap();
        qualia_core_db::render::skeletal::compose_palette_into(
            &binding.joints,
            &local,
            Motor::identity(),
            &mut global,
            &mut palette,
        )
        .unwrap();
        let a = palette[11].transform_point(mesh.positions[0].map(f64::from));
        let b = palette[11].transform_point(mesh.positions[1].map(f64::from));
        let bind = (0..3)
            .map(|i| (mesh.positions[0][i] as f64 - mesh.positions[1][i] as f64).powi(2))
            .sum::<f64>();
        let posed = (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>();
        assert!((bind - posed).abs() < 1e-8);
    }
}
