//! Anatomical pose data for original quadrupeds. Qualia owns sampling,
//! skinning, transitions, semantic joint lookup and renderer upload.
use crate::asset_catalog::{AssetRecipe,ParametricRecipe};
use crate::resident_animation::AssetRigBinding;
use qualia_core_db::render::skeletal::{Motor,Joint,SkinWeight,Track,Key};
use qualia_core_db::render::skeletal::authoring::{BoneSegment,weights_from_segments_into};
use qualia_core_db::container_10d::skeletal_section::ClipRecord;
const NAMES:[&str;19]=["root","body","neck","head","left-ear","right-ear","tail",
    "front-left-hip","front-left-knee","front-left-ankle","front-right-hip","front-right-knee","front-right-ankle",
    "rear-left-hip","rear-left-knee","rear-left-ankle","rear-right-hip","rear-right-knee","rear-right-ankle"];
const PARENTS:[Option<u16>;19]=[None,Some(0),Some(1),Some(2),Some(3),Some(3),Some(1),
    Some(0),Some(7),Some(8),Some(0),Some(10),Some(11),Some(0),Some(13),Some(14),Some(0),Some(16),Some(17)];
pub(crate) fn author(recipe:&AssetRecipe,positions:&[[f32;3]])->Result<Option<AssetRigBinding>,String> {
    let Some(id)=recipe.id.strip_prefix("rc:asset/fauna/") else {return Ok(None);};
    let Some((animal,part))=id.split_once('/') else {return Ok(None);};
    let Some(profile)=crate::quadruped_models::profile(animal) else {return Ok(None);};
    let Some(ParametricRecipe::Transform{target:origin,..})=&recipe.parametric else {return Err("quadruped placement missing".into());};
    let points=crate::quadruped_models::joints(profile).map(|p|std::array::from_fn(|i|origin[i]+p[i]));
    let joints:Vec<Joint>=points.iter().enumerate().map(|(i,p)| {
        let local=match PARENTS[i] {None=>*p,Some(parent)=>std::array::from_fn(|a|p[a]-points[parent as usize][a])};
        Joint{parent:PARENTS[i],rest_local:Motor::from_translation(local[0] as f64,local[1] as f64,local[2] as f64),
            inverse_bind:Motor::from_translation(-p[0] as f64,-p[1] as f64,-p[2] as f64)}
    }).collect();
    let links:&[(usize,usize)]=match part {
        "coat"|"body"|"shoulders"=>&[(1,1)],"neck"|"mane"=>&[(2,3)],
        "head"|"muzzle"|"nose"|"horns"|"horns-right"|"eye-left"|"eye-right"|
        "antler-left"|"antler-right"|"antler-tine-left"|"antler-tine-right"=>&[(3,3)],
        "ear-left"=>&[(4,4)],"ear-right"=>&[(5,5)],"tail"=>&[(6,6)],
        "leg-front-left"=>&[(7,8),(8,9)],"leg-front-right"=>&[(10,11),(11,12)],
        "leg-rear-left"=>&[(13,14),(14,15)],"leg-rear-right"=>&[(16,17),(17,18)],
        "hooves"|"paws"=>&[(9,9),(12,12),(15,15),(18,18)],
        _=>return Err(format!("quadruped part has no binding: {part}")),
    };
    let segments:Vec<BoneSegment>=links.iter().map(|(a,b)|BoneSegment{joint:*a as u16,start:points[*a],end:points[*b]}).collect();
    let mut skin=vec![SkinWeight{joints:[0;4],weights:[1.0,0.0,0.0,0.0]};positions.len()];
    weights_from_segments_into(positions,&segments,joints.len(),if part.starts_with("leg-") {2}else{1},&mut skin)
        .map_err(|e|format!("quadruped weights: {e:?}"))?;
    let ids=NAMES.iter().map(|name|qualia_core_db::q_hash(&format!("rc:rig/fauna/{animal}/{name}"))).collect();
    let mut result=AssetRigBinding{joints,ids,skin,clips:Vec::new(),tracks:Vec::new(),keys:Vec::new()};
    for (clip,duration) in [("idle",5.2),("alert",2.6),("graze",4.0),("walk",1.3)] {
        let first_track=result.tracks.len() as u32;
        for joint in 0..result.joints.len() {
            let first_key=result.keys.len() as u32;
            for frame in 0..=8 {
                let phase=frame as f64/8.0;
                result.keys.push(Key{time:phase*duration,local:pose(clip,joint,phase,result.joints[joint].rest_local,profile.height as f64)});
            }
            result.tracks.push(Track{joint:joint as u16,first_key,key_count:9});
        }
        result.clips.push(ClipRecord{id:qualia_core_db::q_hash(&format!("rc:clip/quadruped/{clip}")),duration,
            first_track,track_count:result.joints.len() as u32,looping:true});
    }
    Ok(Some(result))
}
fn pose(clip:&str,joint:usize,phase:f64,rest:Motor,height:f64)->Motor {
    let wave=(phase*std::f64::consts::TAU).sin();let pulse=(1.0-(phase*std::f64::consts::TAU).cos())*0.5;
    let mut angle=[0.0_f64;3];let (_,mut translation)=rest.to_rotation_translation();
    match clip {
        "idle"=>match joint {1=>translation[1]+=wave*0.004*height,3=>angle[1]=wave*0.035,
            4=>angle[2]=wave*0.08,5=>angle[2]=-wave*0.07,6=>angle[1]=wave*0.05,_=>{}},
        "alert"=>match joint {2=>angle[0]=0.04,3=>angle[1]=wave*0.16,4=>angle[2]=0.07,5=>angle[2]=-0.07,_=>{}},
        "graze"=>match joint {2=>angle[0]=-0.60-pulse*0.07,3=>angle[0]=-0.18+wave*0.03,_=>{}},
        "walk"=>match joint {7|16=>angle[0]=wave*0.22,10|13=>angle[0]=-wave*0.22,
            8|17=>angle[0]=pulse*0.25,11|14=>angle[0]=(1.0-pulse)*0.25,6=>angle[1]=wave*0.04,_=>{}},_=>{}
    }
    let mut rotation=Motor::identity();
    for (axis,radians) in angle.into_iter().enumerate() {
        let half=radians*0.5;let mut q=[half.cos(),0.0,0.0,0.0];q[axis+1]=half.sin();
        rotation=rotation.mul(&Motor::from_rotation_translation(q,[0.0;3]));
    }
    let (q,_)=rotation.to_rotation_translation();Motor::from_rotation_translation(q,translation)
}
