//! Original touring bind rigs. The common native player handles body motion,
//! wheel hinges, UV-seam weights, normals/tangents and backend uploads.
use crate::asset_catalog::AssetRecipe;
use crate::resident_animation::AssetRigBinding;
use qualia_core_db::render::skeletal::{Joint,Motor,SkinWeight,Track,Key};
use qualia_core_db::container_10d::skeletal_section::ClipRecord;

pub(crate) fn rig_name(rig:u8)->&'static str {match rig {0=>"campervan",1=>"4wd-caravan",2=>"car-trailer",_=>"4wd-roof-tent"}}
pub(crate) fn root_id(rig:u8)->u64 {
    const ROOTS:[u64;4]=[qualia_core_db::q_hash("rc:rig/touring-campervan/root"),qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/root"),
        qualia_core_db::q_hash("rc:rig/touring-car-trailer/root"),qualia_core_db::q_hash("rc:rig/touring-4wd-roof-tent/root")];
    ROOTS[rig.min(3) as usize]
}
macro_rules! wheel_set {($name:literal)=>{[
    (qualia_core_db::q_hash(concat!("rc:rig/touring-",$name,"/wheel-0-left")),true,false),
    (qualia_core_db::q_hash(concat!("rc:rig/touring-",$name,"/wheel-0-right")),true,false),
    (qualia_core_db::q_hash(concat!("rc:rig/touring-",$name,"/wheel-1-left")),false,false),
    (qualia_core_db::q_hash(concat!("rc:rig/touring-",$name,"/wheel-1-right")),false,false)]};}
pub(crate) fn motion_joints(rig:u8)->&'static [(u64,bool,bool)] {
    const VAN:[(u64,bool,bool);4]=wheel_set!("campervan");
    const FOUR:[(u64,bool,bool);4]=wheel_set!("4wd-caravan");
    const CAR:[(u64,bool,bool);4]=wheel_set!("car-trailer");
    const ROOF:[(u64,bool,bool);4]=wheel_set!("4wd-roof-tent");
    const TOW:[(u64,bool,bool);8]=[FOUR[0],FOUR[1],FOUR[2],FOUR[3],
        (qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/caravan/wheel-0-left"),false,true),
        (qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/caravan/wheel-0-right"),false,true),
        (qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/caravan/wheel-1-left"),false,true),
        (qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/caravan/wheel-1-right"),false,true)];
    const CAMPER:[(u64,bool,bool);6]=[CAR[0],CAR[1],CAR[2],CAR[3],
        (qualia_core_db::q_hash("rc:rig/touring-car-trailer/camper-trailer/wheel-0-left"),false,true),
        (qualia_core_db::q_hash("rc:rig/touring-car-trailer/camper-trailer/wheel-0-right"),false,true)];
    match rig {0=>&VAN,1=>&TOW,2=>&CAMPER,_=>&ROOF}
}
pub(crate) fn wheel_id(rig:u8,part:&str)->u64 {qualia_core_db::q_hash(&format!("rc:rig/touring-{}/{}",rig_name(rig),part))}
pub(crate) fn trailer_id(rig:u8)->u64 {match rig {
    1=>qualia_core_db::q_hash("rc:rig/touring-4wd-caravan/trailer-hitch"),
    2=>qualia_core_db::q_hash("rc:rig/touring-car-trailer/trailer-hitch"),_=>0,
}}
pub(crate) fn trailer_part(rig:u8,part:&str)->bool {match rig {
    1=>part.starts_with("caravan")||part=="drawbar",
    2=>part.starts_with("trailer-")||part.starts_with("camper-trailer/"),_=>false,
}}
pub(crate) fn wheels(rig:u8)->Vec<(String,[f64;3],bool)> {
    let mut result=Vec::new();
    let (offset,axles):(f64,Vec<f64>)=match rig {
        0=>(0.94,vec![4.77-1.23,4.77+1.12]),2=>(0.86,vec![4.65-0.84,4.65+0.82]),
        _=>(0.86,vec![4.65-0.77,4.65+0.78]),
    };
    for (index,z) in axles.into_iter().enumerate() {for (side,sign) in [("left",-1.0),("right",1.0)] {
        result.push((format!("wheel-{index}-{side}"),[2.35+sign*offset,0.29,z],index==0));
    }}
    if rig==1 {
        for (index,z) in [4.65+3.39-0.13,4.65+3.39+0.42].into_iter().enumerate() {
            for (side,sign) in [("left",-1.0),("right",1.0)] {result.push((format!("caravan/wheel-{index}-{side}"),[2.35+sign*0.93,0.29,z],false));}
        }
    }else if rig==2 {
        for (side,sign) in [("left",-1.0),("right",1.0)] {result.push((format!("camper-trailer/wheel-0-{side}"),[2.35+sign*0.78,0.29,4.65+2.65],false));}
    }
    result
}
pub(crate) fn author(recipe:&AssetRecipe,positions:&[[f32;3]])->Result<Option<AssetRigBinding>,String> {
    let Some(name)=recipe.id.strip_prefix("rc:asset/rig/") else {return Ok(None);};
    let Some((name,part))=name.split_once('/') else {return Err("touring asset part missing".into());};
    let rig=(0..4).find(|rig|rig_name(*rig)==name).ok_or_else(||"unknown touring rig identity".to_owned())?;
    let wheel_parts=wheels(rig);let origin=[2.35,0.0,4.65];
    let mut joints=vec![Joint{parent:None,rest_local:Motor::from_translation(origin[0],origin[1],origin[2]),
        inverse_bind:Motor::from_translation(-origin[0],-origin[1],-origin[2])}];
    let mut ids=vec![root_id(rig)];let has_trailer=rig==1||rig==2;
    let hitch=[2.35,0.39,4.65+if rig==1 {1.36}else{1.55}];
    if has_trailer {
        joints.push(Joint{parent:Some(0),rest_local:Motor::from_translation(hitch[0]-origin[0],hitch[1]-origin[1],hitch[2]-origin[2]),
            inverse_bind:Motor::from_translation(-hitch[0],-hitch[1],-hitch[2])});
        ids.push(trailer_id(rig));
    }
    let mut weighted_joint=if trailer_part(rig,part){1}else{0};
    for (index,(wheel,point,_)) in wheel_parts.iter().enumerate() {
        let trailer=trailer_part(rig,wheel);let parent=if trailer {1}else{0};let parent_point=if trailer {hitch}else{origin};
        joints.push(Joint{parent:Some(parent),rest_local:Motor::from_translation(point[0]-parent_point[0],point[1]-parent_point[1],point[2]-parent_point[2]),
            inverse_bind:Motor::from_translation(-point[0],-point[1],-point[2])});
        ids.push(wheel_id(rig,wheel));
        if part==wheel||part==wheel.replace("wheel-","rim-") {weighted_joint=index as u16+1+if has_trailer {1}else{0};}
    }
    let mut tracks=Vec::new();let mut keys=Vec::new();
    for (index,joint) in joints.iter().enumerate() {
        tracks.push(Track{joint:index as u16,first_key:keys.len() as u32,key_count:2});
        keys.extend([Key{time:0.0,local:joint.rest_local},Key{time:1.0,local:joint.rest_local}]);
    }
    Ok(Some(AssetRigBinding{skin:vec![SkinWeight{joints:[weighted_joint,0,0,0],weights:[1.0,0.0,0.0,0.0]};positions.len()],
        ids,clips:vec![ClipRecord{id:qualia_core_db::q_hash("rc:clip/touring/idle"),duration:1.0,first_track:0,track_count:tracks.len() as u32,looping:true}],
        joints,tracks,keys}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    fn body_and_wheels_use_distinct_rigid_joints_with_shared_root_identity() {
        for rig in 0..4 {
            let models=crate::vehicles::rig_assets(rig);
            let mut ids=std::collections::BTreeSet::new();
            for model in models {
                assert!(ids.insert(model.id.clone()),"duplicate vehicle part identity");
                let positions=if let Some(shape)=&model.parametric {shape.compile().unwrap().positions}else{vec![[0.0;3];8]};
                let binding=author(&model,&positions).unwrap().unwrap();
                assert_eq!(binding.ids[0],root_id(rig));
                if model.id.contains("/wheel-")||model.id.contains("/rim-") {
                    assert!(binding.skin.iter().all(|skin|skin.joints[0]>0));
                }else {
                    let part=model.id.strip_prefix(&format!("rc:asset/rig/{}/",rig_name(rig))).unwrap();
                    let expected=if trailer_part(rig,part){1}else{0};
                    assert!(binding.skin.iter().all(|skin|skin.joints[0]==expected));}
                for &(joint,_,_) in motion_joints(rig) {assert!(binding.ids.contains(&joint));}
            }
        }
    }
    #[wasm_bindgen_test]
    fn trailer_rotates_about_hitch_without_moving_the_tow_body_or_coupling() {
        for rig in [1,2] {
            let models=crate::vehicles::rig_assets(rig);
            let name=if rig==1 {"caravan-body"}else{"trailer-body"};
            let model=models.iter().find(|model|model.id.ends_with(name)).unwrap();
            let binding=author(model,&[[2.35,0.8,8.0]]).unwrap().unwrap();
            assert_eq!(binding.ids[1],trailer_id(rig));
            assert_eq!(binding.skin[0].joints[0],1);
            let mut local:Vec<_>=binding.joints.iter().map(|joint|joint.rest_local).collect();
            local[1]=local[1].mul(&Motor::from_rotation_translation([0.2_f64.cos(),0.0,0.2_f64.sin(),0.0],[0.0;3]));
            let mut globals=vec![Motor::identity();local.len()];let mut palette=globals.clone();
            qualia_core_db::render::skeletal::compose_palette_into(&binding.joints,&local,Motor::identity(),&mut globals,&mut palette).unwrap();
            let hitch=[2.35,0.39,4.65+if rig==1 {1.36}else{1.55}];
            let fixed=palette[1].transform_point(hitch);
            for axis in 0..3 {assert!((fixed[axis]-hitch[axis]).abs()<1e-9);}
            let point=[2.35,0.8,8.0];let moved=palette[1].transform_point(point);
            assert!((moved[0]-point[0]).abs()>0.1);
            let body=palette[0].transform_point(point);
            for axis in 0..3 {assert!((body[axis]-point[axis]).abs()<1e-9);}
            for (index,(part,_,_)) in wheels(rig).iter().enumerate() {
                let parent=if trailer_part(rig,part){1}else{0};
                assert_eq!(binding.joints[index+2].parent,Some(parent));
            }
        }
    }
}
