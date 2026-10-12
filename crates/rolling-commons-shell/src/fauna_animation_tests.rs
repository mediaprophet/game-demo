use super::*;
use qualia_core_db::render::skeletal::{Clip,sample_pose_into,compose_palette_into,skin_vertices_into};
use qualia_core_db::container_10d::skeletal_section::{Counts,SkeletalData,SkeletalSection,encode_skeletal_section_into};

#[test]
fn original_encounter_parts_retain_identity_and_receive_native_rigs() {
    for (animal,wallaby) in [("road-kangaroo-01",false),("road-wallaby-01",true)] {
        let recipes=crate::macropod_models::models(animal,[2.3,0.0,0.5],wallaby);
        for part in ["torso","chest","head","muzzle","ear-left","ear-right","tail",
            "arm-left","arm-right","hind-leg-left","hind-leg-right","foot-left","foot-right"] {
            assert!(recipes.iter().any(|r|r.id==format!("rc:asset/fauna/{animal}/{part}")));
        }
        let mut group=None;
        for recipe in recipes {
            let mesh=recipe.parametric.as_ref().unwrap().compile().unwrap();
            let rig=author(&recipe,&mesh.positions).unwrap().unwrap();
            assert_eq!(rig.joints.len(),18);assert_eq!(rig.clips.len(),3);
            if let Some(root)=group {assert_eq!(rig.ids[0],root);}else{group=Some(rig.ids[0]);}
            let counts=Counts{joints:rig.joints.len(),vertices:rig.skin.len(),clips:rig.clips.len(),tracks:rig.tracks.len(),keys:rig.keys.len()};
            let mut encoded=vec![0;counts.encoded_len().unwrap()];
            encode_skeletal_section_into(&SkeletalData{joints:&rig.joints,joint_ids:&rig.ids,skin:&rig.skin,
                clips:&rig.clips,tracks:&rig.tracks,keys:&rig.keys},&mut encoded).unwrap();
            let read=SkeletalSection::parse(&encoded,mesh.positions.len()).unwrap();
            assert_eq!(read.joint_id(0),Some(rig.ids[0]));
        }
    }
}

#[test]
fn idle_deforms_head_but_keeps_feet_and_encounter_root_fixed() {
    let recipes=crate::macropod_models::models("road-kangaroo-01",[2.3,0.0,0.5],false);
    for part in ["head","foot-left"] {
        let recipe=recipes.iter().find(|r|r.id.ends_with(&format!("/{part}"))).unwrap();
        let mesh=recipe.parametric.as_ref().unwrap().compile().unwrap();let rig=author(recipe,&mesh.positions).unwrap().unwrap();
        let clip=rig.clips[0];let tracks=&rig.tracks[clip.first_track as usize..(clip.first_track+clip.track_count) as usize];
        let mut local=vec![Motor::identity();rig.joints.len()];
        sample_pose_into(&rig.joints,Clip{duration:clip.duration,looping:true,tracks,keys:&rig.keys},1.2,None,1.0,&mut local).unwrap();
        let mut globals=local.clone();let mut palette=local.clone();
        compose_palette_into(&rig.joints,&local,Motor::identity(),&mut globals,&mut palette).unwrap();
        let mut output=mesh.positions.clone();let normals=vec![[0.0,1.0,0.0];output.len()];let mut out_normals=normals.clone();
        skin_vertices_into(&mesh.positions,&normals,&rig.skin,&palette,&mut output,&mut out_normals).unwrap();
        let maximum=output.iter().zip(&mesh.positions).map(|(a,b)|(0..3).map(|i|(a[i]-b[i]).abs()).fold(0.0_f32,f32::max)).fold(0.0_f32,f32::max);
        if part=="head" {assert!(maximum>0.001);}else{assert!(maximum<1e-5);}
        let root=globals[0].transform_point([0.0;3]);
        assert!((root[0]-2.3).abs()<1e-5);assert!(root[1].abs()<1e-5);assert!((root[2]-0.5).abs()<1e-5);
    }
}
