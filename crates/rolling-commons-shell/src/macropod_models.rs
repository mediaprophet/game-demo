//! Original anatomical kangaroo/wallaby source assets. Qualia compiles all
//! curved forms, placement, surface attributes and native skeletal sections.
use crate::asset_catalog::{AssetRecipe,ParametricRecipe as Shape,LimbSegment,material_family_for};
fn ellipsoid(center:[f32;3],radii:[f32;3])->Shape {
    Shape::Ellipsoid{center,radii,latitude:20,longitude:28}
}
fn limbs(segments:&[([f32;3],[f32;3],f32,f32)])->Shape {
    Shape::LimbCluster{center:[0.0;3],radial_segments:12,
        limbs:segments.iter().map(|(start,end,a,b)|LimbSegment{start:*start,end:*end,start_radius:*a,end_radius:*b}).collect()}
}
pub(crate) fn models(prefix:&str,origin:[f32;3],wallaby:bool)->Vec<AssetRecipe> {
    let scale=if wallaby {0.79}else{1.0};
    let coat=if wallaby {[0.48,0.37,0.27,1.0]}else{[0.53,0.37,0.25,1.0]};
    let belly=if wallaby {[0.73,0.62,0.47,1.0]}else{[0.78,0.67,0.52,1.0]};
    let mut out=Vec::new();
    let mut add=|part:&str,color:[f32;4],source:Shape| {
        let id=format!("rc:asset/fauna/{prefix}/{part}");
        out.push(AssetRecipe{material_family:material_family_for(&id),id,parts:Vec::new(),
            parametric:Some(Shape::Transform{source:Box::new(source),pivot:[0.0;3],target:origin,scale:[scale;3]}),
            color,vibe_source:None,render_role:"opaque".into()});
    };
    add("torso",coat,ellipsoid([0.0,1.13,0.02],[0.33,0.49,0.40]));
    add("chest",belly,ellipsoid([0.0,1.30,-0.27],[0.25,0.40,0.22]));
    add("neck",coat,limbs(&[([0.0,1.41,-0.17],[0.0,1.69,-0.42],0.21,0.145)]));
    add("head",coat,ellipsoid([0.0,1.77,-0.49],[0.18,0.21,0.25]));
    add("muzzle",belly,ellipsoid([0.0,1.71,-0.70],[0.125,0.105,0.20]));
    add("nose",[0.10,0.085,0.07,1.0],ellipsoid([0.0,1.73,-0.88],[0.095,0.06,0.04]));
    add("tail",coat,limbs(&[
        ([0.0,0.91,0.32],[0.0,0.59,0.91],0.16,0.095),
        ([0.0,0.59,0.91],[0.04,0.16,1.54],0.095,0.025)]));
    for (side,sign) in [("left",-1.0),("right",1.0)] {
        add(&format!("ear-{side}"),coat,ellipsoid([sign*0.115,2.06,-0.49],[0.067,0.22,0.048]));
        add(&format!("ear-inner-{side}"),[0.30,0.20,0.17,1.0],ellipsoid([sign*0.115,2.065,-0.532],[0.036,0.16,0.009]));
        add(&format!("eye-{side}"),[0.052,0.041,0.033,1.0],ellipsoid([sign*0.167,1.82,-0.57],[0.026,0.032,0.037]));
        add(&format!("arm-{side}"),coat,limbs(&[
            ([sign*0.21,1.36,-0.28],[sign*0.24,1.14,-0.34],0.067,0.048),
            ([sign*0.24,1.14,-0.34],[sign*0.17,1.03,-0.46],0.048,0.028)]));
        add(&format!("hind-leg-{side}"),coat,limbs(&[
            ([sign*0.23,0.91,0.18],[sign*0.28,0.48,-0.30],0.19,0.115),
            ([sign*0.28,0.48,-0.30],[sign*0.28,0.16,-0.04],0.095,0.046)]));
        add(&format!("foot-{side}"),[0.25,0.19,0.15,1.0],ellipsoid([sign*0.28,0.075,-0.33],[0.095,0.075,0.32]));
    }
    out
}
