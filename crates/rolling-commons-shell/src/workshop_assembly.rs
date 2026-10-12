//! Original service-bay assembly geometry. Qualia owns mesh, material, picking
//! and rendering; accepted VibeScript state determines component placement.
use crate::asset_catalog::{AssetRecipe,ParametricRecipe as Shape,PanelPart,MaterialFamily,
    GALVANISED_METAL,PAINTED_METAL,RUBBER,SOLAR_GLASS,FRAMED_GLASS,WEATHERED_CONCRETE};
use wasm_bindgen::prelude::*;

#[derive(Clone,Copy)]
pub(crate) struct VisualState {
    pub rig:u8,pub hood:bool,pub hose:bool,pub radiator:i8,pub filled:bool,
    pub controller:bool,pub fuse:bool,pub panel:bool,
}
fn panel(offset:[f32;3],size:[f32;3])->PanelPart {
    PanelPart{offset,size,pitch_deg:0.0,roll_deg:0.0,yaw_deg:0.0}
}
fn group(parts:Vec<PanelPart>)->Shape {Shape::PanelGroup{center:[0.0;3],parts}}
fn model(name:&str,shape:Shape,color:[f32;4],family:MaterialFamily)->AssetRecipe {
    AssetRecipe{id:format!("rc:asset/workshop-assembly-{name}"),parts:Vec::new(),
        parametric:Some(shape),color,material_family:family,vibe_source:None,render_role:"opaque".into()}
}
fn tube(start:[f32;3],end:[f32;3],radius:f32)->Shape {Shape::Stem{start,end,radius,segments:20}}
fn box_shape(center:[f32;3],size:[f32;3])->Shape {group(vec![panel(center,size)])}

pub(crate) fn assets(state:VisualState)->Vec<AssetRecipe> {
    let metal=[0.52,0.57,0.56,1.0];let dark=[0.12,0.16,0.17,1.0];
    let paint=match state.rig {0=>[0.85,0.82,0.70,1.0],1=>[0.61,0.54,0.38,1.0],
        2=>[0.43,0.58,0.59,1.0],_=>[0.76,0.70,0.54,1.0]};
    let width=if state.rig==2 {0.73}else{0.88};
    let rear=if state.rig==0 {2.4}else{1.7};
    let mut out=vec![
        model("service-floor",box_shape([0.0,-0.08,0.5],[5.6,0.16,6.0]),[0.58,0.59,0.53,1.0],WEATHERED_CONCRETE),
        model("chassis",group(vec![panel([-width*0.7,0.37,0.5],[0.11,0.14,rear+2.0]),
            panel([width*0.7,0.37,0.5],[0.11,0.14,rear+2.0]),panel([0.0,0.39,-0.9],[width*1.5,0.12,0.12]),
            panel([0.0,0.39,0.5],[width*1.5,0.12,0.12])]),dark,GALVANISED_METAL),
        model("cab",crate::vehicles::coachwork(0.0,0.0,&[
            [0.46,width,0.54,0.72,0.10],[0.95,width,0.54,1.3,0.13],
            [rear-0.12,width,0.54,1.3,0.12],[rear,width*0.92,0.58,1.15,0.12]]),paint,PAINTED_METAL),
        model("engine-bay-walls",group(vec![panel([-width,0.72,-0.54],[0.09,0.52,1.84]),
            panel([width,0.72,-0.54],[0.09,0.52,1.84]),panel([0.0,0.65,-1.46],[width*2.0,0.20,0.10])]),paint,PAINTED_METAL),
        model("engine-block",crate::vehicles::coachwork(0.0,-0.22,&[
            [-0.36,0.29,0.48,0.46,0.065],[0.32,0.29,0.48,0.46,0.065]]),dark,GALVANISED_METAL),
        model("rocker-cover",crate::vehicles::coachwork(0.0,-0.22,&[
            [-0.32,0.23,0.90,0.16,0.045],[0.28,0.23,0.90,0.16,0.045]]),metal,GALVANISED_METAL),
        model("parts-bench",group(vec![panel([1.85,0.76,0.0],[1.2,0.11,2.7]),
            panel([1.37,0.38,-1.1],[0.06,0.72,0.06]),panel([2.33,0.38,-1.1],[0.06,0.72,0.06]),
            panel([1.37,0.38,1.1],[0.06,0.72,0.06]),panel([2.33,0.38,1.1],[0.06,0.72,0.06])]),metal,GALVANISED_METAL),
    ];
    let mut ribs=Vec::new();
    for i in 0..9 {ribs.push(panel([0.0,1.065,-0.5+i as f32*0.063],[0.43,0.018,0.014]));}
    out.push(model("cover-ribs",group(ribs),metal,GALVANISED_METAL));
    for (i,z) in [-0.96,rear-0.35].into_iter().enumerate() {
        for (side,x) in [-width-0.05,width+0.05].into_iter().enumerate() {
            out.push(model(&format!("tyre-{i}-{side}"),Shape::WheelX{center:[x,0.32,z],
                radius:0.325,width:0.19,segments:36},dark,RUBBER));
            out.push(model(&format!("wheel-{i}-{side}"),Shape::Ellipsoid{center:[x,0.32,z],
                radii:[0.09,0.185,0.185],latitude:16,longitude:24},metal,GALVANISED_METAL));
        }
    }
    // Bonnet rotates about its rear hinge; a support stem is present when open.
    out.push(model("bonnet",Shape::TiltedBox{center:if state.hood {[0.0,1.60,0.02]}else{[0.0,1.01,-0.52]},
        size:[width*1.98,0.065,1.72],pitch_deg:if state.hood {-68.0}else{0.0},roll_deg:0.0,yaw_deg:0.0},paint,PAINTED_METAL));
    if state.hood {out.push(model("bonnet-stay",tube([width*0.75,0.73,-1.2],[width*0.75,1.66,-0.23],0.012),metal,GALVANISED_METAL));}
    out.push(model("windscreen",Shape::TiltedBox{center:[0.0,1.44,0.73],size:[width*1.75,0.64,0.032],
        pitch_deg:-36.0,roll_deg:0.0,yaw_deg:0.0},[0.27,0.43,0.47,1.0],FRAMED_GLASS));
    let radiator=if state.radiator<0 {[1.85,0.90,-0.83]}else{[0.0,0.79,-1.16]};
    let mut frame=vec![panel([radiator[0]-0.36,radiator[1],radiator[2]],[0.065,0.56,0.12]),
        panel([radiator[0]+0.36,radiator[1],radiator[2]],[0.065,0.56,0.12]),
        panel([radiator[0],radiator[1]-0.25,radiator[2]],[0.72,0.07,0.12]),
        panel([radiator[0],radiator[1]+0.25,radiator[2]],[0.72,0.07,0.12])];
    for i in 0..30 {frame.push(panel([radiator[0]-0.32+i as f32*0.022,radiator[1],radiator[2]],[0.006,0.45,0.074]));}
    out.push(model("radiator",group(frame),if state.radiator==0 {[0.37,0.36,0.29,1.0]}else{metal},GALVANISED_METAL));
    for (index,x) in [-0.39,0.39].into_iter().enumerate(){
        out.push(model(&format!("radiator-mount-{index}"),Shape::Cylinder{center:if state.radiator<0 {[1.70+index as f32*0.1,0.84,-0.4]}else{[x,0.89,-1.16]},
            radius:0.035,height:0.022,segments:12},metal,GALVANISED_METAL));
    }
    let hose_points=if state.hose {vec![[0.29,0.93,-1.16],[0.47,0.98,-1.02],[0.52,0.93,-0.64],[0.36,0.85,-0.40]]}
        else {vec![[1.6,0.90,0.0],[1.82,0.99,0.04],[2.03,0.96,0.28],[2.1,0.86,0.5]]};
    for i in 0..hose_points.len()-1 {out.push(model(&format!("hose-{i}"),tube(hose_points[i],hose_points[i+1],0.045),dark,RUBBER));}
    out.push(model("coolant-reservoir",Shape::Ellipsoid{center:[-0.58,0.83,-0.44],radii:[0.10,0.19,0.12],
        latitude:18,longitude:24},if state.filled {[0.72,0.81,0.64,1.0]}else{[0.83,0.81,0.73,1.0]},PAINTED_METAL));
    let controller=if state.controller {[-0.57,0.86,0.12]}else{[1.65,0.86,0.78]};
    out.push(model("controller",box_shape(controller,[0.26,0.12,0.29]),[0.26,0.42,0.48,1.0],PAINTED_METAL));
    let mut fins=Vec::new();
    for i in 0..7 {fins.push(panel([controller[0]-0.11+i as f32*0.036,controller[1]+0.075,controller[2]],[0.014,0.04,0.26]));}
    out.push(model("controller-heatsink",group(fins),metal,GALVANISED_METAL));
    out.push(model("protection-cartridge",box_shape(if state.fuse {[-0.56,0.93,0.31]}else{[2.03,0.84,0.73]},[0.09,0.065,0.11]),[0.76,0.46,0.24,1.0],PAINTED_METAL));
    let solar=if state.panel {[0.0,1.9,1.4]}else{[2.07,1.2,1.20]};
    out.push(model("solar-frame",box_shape(solar,[1.12,0.055,0.72]),metal,GALVANISED_METAL));
    let mut cells=Vec::new();
    for x in 0..6 {for z in 0..4 {cells.push(panel([solar[0]-0.46+x as f32*0.184,solar[1]+0.034,solar[2]-0.255+z as f32*0.17],[0.176,0.012,0.158]));}}
    out.push(model("solar-cells",group(cells),[0.11,0.22,0.37,1.0],SOLAR_GLASS));
    // Supplied tools remain visible, independently selectable from assembly parts.
    out.push(model("socket",tube([1.42,0.85,0.45],[1.80,0.85,0.45],0.017),metal,GALVANISED_METAL));
    out.push(model("pressure-gauge",Shape::Cylinder{center:[2.18,0.88,0.3],radius:0.065,height:0.027,segments:32},[0.81,0.82,0.74,1.0],PAINTED_METAL));
    out.push(model("catch-tray",group(vec![panel([0.0,0.055,-1.07],[0.95,0.045,0.52]),
        panel([-0.47,0.11,-1.07],[0.03,0.10,0.52]),panel([0.47,0.11,-1.07],[0.03,0.10,0.52]),
        panel([0.0,0.11,-1.33],[0.95,0.10,0.03]),panel([0.0,0.11,-0.81],[0.95,0.10,0.03])]),dark,PAINTED_METAL));
    out
}

pub(crate) fn scene(state:VisualState)->Result<JsValue,JsValue> {
    let recipes=assets(state);let assets=js_sys::Array::new();
    let mut min=[f32::INFINITY;3];let mut max=[f32::NEG_INFINITY;3];
    for recipe in &recipes {
        let (mesh,record)=crate::asset_compile::compile_asset(recipe)?;
        for axis in 0..3 {min[axis]=min[axis].min(mesh.min[axis]);max[axis]=max[axis].max(mesh.max[axis]);}
        assets.push(&record);
    }
    let span=(0..3).map(|axis|max[axis]-min[axis]).fold(1e-6_f32,f32::max);
    let center=[(min[0]+max[0])*0.5,(min[1]+max[1])*0.5,(min[2]+max[2])*0.5];
    let scale=1.7/span;
    let targets=js_sys::Array::new();let mut nodes=Vec::new();
    for (part,label,point) in [
        ("bonnet","Bonnet and support",[0.0,1.5,-0.3]),
        ("radiator","Radiator core and mounts",if state.radiator<0 {[1.85,0.95,-0.83]}else{[0.0,0.85,-1.16]}),
        ("hose","Hoses and clamps",if state.hose {[0.48,0.97,-0.84]}else{[1.85,0.97,0.18]}),
        ("controller","Controller and protection",if state.controller {[-0.57,0.92,0.12]}else{[1.65,0.92,0.78]}),
        ("solar","Solar panel and frame",if state.panel {[0.0,1.95,1.4]}else{[2.07,1.26,1.20]}),
        ("gauge","Pressure gauge",[2.18,0.90,0.3]),
    ] {
        let mut p=[0.0;3];
        for axis in 0..3 {p[axis]=(point[axis]-center[axis])*scale;}
        nodes.extend_from_slice(&[0.0,0.0,1.2,p[0],p[1],p[2],0.0,1.0,0.7,0.5]);
        let target=js_sys::Object::new();
        js_sys::Reflect::set(&target,&"workshopPart".into(),&JsValue::from_str(part))?;
        js_sys::Reflect::set(&target,&"label".into(),&JsValue::from_str(label))?;
        targets.push(&target);
    }
    let tensor=crate::tensor_buffer_build(&nodes)?;
    let textures = crate::material_library::texture_receipts()?;
    let result = js_sys::Object::new();
    for (key, value) in [
        ("assets", JsValue::from(assets)),
        ("textures", JsValue::from(textures)),
        ("tensor", JsValue::from(js_sys::Uint8Array::from(tensor.as_slice()))),
        ("pickTargets", JsValue::from(targets)),
        ("worldScale", JsValue::from_f64(scale as f64)),
    ] {
        js_sys::Reflect::set(&result, &key.into(), &value)?;
    }
    let origin=js_sys::Object::new();
    for (key,value) in ["x","y","z"].into_iter().zip(center) {js_sys::Reflect::set(&origin,&key.into(),&JsValue::from_f64(value as f64))?;}
    js_sys::Reflect::set(&result,&"worldCenter".into(),&origin)?;
    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    fn disassembled_radiator_and_hose_move_to_bench_with_stable_identity() {
        let initial=VisualState{rig:1,hood:false,hose:true,radiator:0,filled:true,controller:false,fuse:false,panel:false};
        let removed=VisualState{hood:true,hose:false,radiator:-1,filled:false,..initial};
        for name in ["radiator","hose-0","bonnet","radiator-mount-0"] {
            let id=format!("rc:asset/workshop-assembly-{name}");
            let a=assets(initial).into_iter().find(|asset|asset.id==id).unwrap();
            let b=assets(removed).into_iter().find(|asset|asset.id==id).unwrap();
            let a=a.parametric.unwrap().compile().unwrap();
            let b=b.parametric.unwrap().compile().unwrap();
            assert_ne!(a.min,b.min);assert_ne!(a.max,b.max);
        }
    }
    #[wasm_bindgen_test]
    fn all_rig_assembly_geometry_is_finite_and_uses_distinct_semantic_parts() {
        for rig in 0..4 {
            let models=assets(VisualState{rig,hood:true,hose:true,radiator:2,filled:true,controller:true,fuse:true,panel:true});
            let mut ids=std::collections::BTreeSet::new();
            for model in models {
                assert!(ids.insert(model.id));
                let mesh=model.parametric.unwrap().compile().unwrap();
                assert!(!mesh.triangles.is_empty());
                assert!(mesh.positions.iter().flatten().all(|component|component.is_finite()));
            }
        }
    }
}
