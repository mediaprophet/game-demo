//! Windowed cab/body shells and original interiors. Panels describe authored
//! apertures; Qualia compiles the panels, curved roof caps and rounded furniture.
use crate::asset_catalog::{AssetRecipe,ParametricRecipe as Shape,PanelPart,
    MaterialFamily,PAINTED_METAL,FRAMED_GLASS,CLOTH_AND_SKIN,GALVANISED_METAL,
    WEATHERED_TIMBER,RUBBER,LimbSegment};
fn panel(offset:[f32;3],size:[f32;3])->PanelPart {
    PanelPart{offset,size,pitch_deg:0.0,roll_deg:0.0,yaw_deg:0.0}
}
fn model(id:String,shape:Shape,color:[f32;4],family:MaterialFamily)->AssetRecipe {
    AssetRecipe{id,parts:Vec::new(),parametric:Some(shape),color,material_family:family,
        vibe_source:None,render_role:"opaque".into()}
}
fn group(parts:Vec<PanelPart>)->Shape {Shape::PanelGroup{center:[0.0;3],parts}}
fn prefix(rig:u8)->&'static str {match rig {0=>"campervan",1=>"4wd-caravan",2=>"car-trailer",_=>"4wd-roof-tent"}}
fn id(rig:u8,name:&str)->String {format!("rc:asset/rig/{}/{name}",prefix(rig))}
fn seals(base:&str,panes:&[PanelPart])->Vec<AssetRecipe> {
    let mut models=Vec::new();
    for (index,pane) in panes.iter().enumerate() {
        let side=pane.size[0]<pane.size[2];let horizontal=if side {2}else{0};
        let mut parts=Vec::new();
        for sign in [-1.0,1.0] {
            let mut at=[0.0;3];at[1]=sign*(pane.size[1]*0.5+0.012);
            let mut size=pane.size;size[1]=0.024;size[horizontal]+=0.048;
            if side {size[0]=0.027;}else{size[2]=0.027;}
            parts.push(panel(at,size));
            let mut at=[0.0;3];at[horizontal]=sign*(pane.size[horizontal]*0.5+0.012);
            let mut size=pane.size;size[horizontal]=0.024;
            if side {size[0]=0.027;}else{size[2]=0.027;}
            parts.push(panel(at,size));
        }
        models.push(model(format!("{base}-seal-{index}"),Shape::Oriented{
            source:Box::new(group(parts)),pivot:[0.0;3],target:pane.offset,
            degrees:[pane.pitch_deg,pane.yaw_deg,pane.roll_deg]},[0.10,0.14,0.14,1.0],RUBBER));
    }
    models
}

/// Replaces the old closed upper loft while retaining `/cabin` and `/glazing`.
fn cab(rig:u8,x:f32,z:f32,paint:[f32;4])->Vec<AssetRecipe> {
    let sedan=rig==2;
    let (width,belt,roof,front,back):(f32,f32,f32,f32,f32)=if sedan {(0.67,0.88,1.27,-0.63,0.75)}else{(0.71,1.12,1.62,-0.49,0.73)};
    let roof_front=-0.34;let roof_back=if sedan {0.40}else{0.60};
    let floor=0.46;let guard_top=if sedan {0.78}else{0.86};
    let mut parts=vec![panel([x,floor,z+(front+back)*0.5],[width*2.0,0.09,back-front])];
    for side in [-1.0,1.0] {
        let mut door=panel([x+side*(width+0.84)*0.5,(guard_top+belt)*0.5,z+(front+back)*0.5],
            [0.055,((belt-guard_top).powi(2)+(0.84-width).powi(2)).sqrt(),back-front]);
        door.roll_deg=side*((0.84-width)/(belt-guard_top)).atan().to_degrees();parts.push(door);
    }
    for end in [front,back] {parts.push(panel([x,(floor+belt)*0.5,z+end],[width*2.0,belt-floor,0.055]));}
    // A/B/C pillars leave actual side openings, including sloping end pillars.
    for side in [-1.0,1.0] {
        let sx=x+side*width;
        parts.push(panel([sx,(belt+roof)*0.5,z+0.22],[0.060,roof-belt,0.055]));
        parts.push(panel([sx,(belt+roof)*0.5,z+(front+roof_front)*0.5],
            [0.055,((roof-belt).powi(2)+(roof_front-front).powi(2)).sqrt(),0.055]));
        let last=parts.last_mut().unwrap();last.pitch_deg=((roof_front-front)/(roof-belt)).atan().to_degrees();
        parts.push(panel([sx,(belt+roof)*0.5,z+(back+roof_back)*0.5],
            [0.055,((roof-belt).powi(2)+(roof_back-back).powi(2)).sqrt(),0.055]));
        let last=parts.last_mut().unwrap();last.pitch_deg=((roof_back-back)/(roof-belt)).atan().to_degrees();
    }
    let mut out=vec![model(id(rig,"cabin"),group(parts),paint,PAINTED_METAL),
        model(id(rig,"body"),crate::vehicles::coachwork(x,z,&[
            [if sedan {-1.43}else{-1.28},0.74,0.44,0.32,0.08],
            [if sedan {-1.04}else{-1.08},0.83,0.42,if sedan {0.38}else{0.45},0.10],
            [front,0.84,0.42,if sedan {0.41}else{0.52},0.10]]),paint,PAINTED_METAL),
        model(id(rig,"rear-body"),crate::vehicles::coachwork(x,z,&[
            [back,0.84,0.42,if sedan {0.41}else{0.55},0.10],
            [if sedan {1.40}else{1.23},0.76,0.46,if sedan {0.28}else{0.50},0.08]]),paint,PAINTED_METAL),
        model(id(rig,"roof-cap"),crate::vehicles::coachwork(x,z,&[
            [roof_front-0.055,width-0.045,roof-0.035,0.075,0.028],
            [roof_front,width,roof-0.035,0.095,0.035],
            [roof_back,width,roof-0.035,0.095,0.035],
            [roof_back+0.055,width-0.045,roof-0.035,0.075,0.028]]),paint,PAINTED_METAL)];
    for side in [-1.0,1.0] {
        out.push(model(id(rig,if side<0.0 {"door-guard-left"}else{"door-guard-right"}),
            Shape::WheelClearancePanel{center_x:x+side*0.82,thickness:0.055,z:[z+front,z+back],
                bottom:floor,top:[guard_top;2],wheel_y:0.29,clearance_radius:if sedan {0.345}else{0.385},
                axles:if sedan {vec![z-0.84,z+0.82]}else{vec![z-0.77,z+0.78]},segments:96},paint,PAINTED_METAL));
    }
    let mut panes=Vec::new();
    let gap=0.042;
    for side in [-1.0,1.0] {
        for (a,b) in [(front+0.08,0.19),(0.26,back-0.06)] {
            panes.push(panel([x+side*(width+0.006),(belt+roof)*0.5,z+(a+b)*0.5],
                [0.012,roof-belt-gap,b-a]));
        }
    }
    for (base,top) in [(front,roof_front),(back,roof_back)] {
        let mut pane=panel([x,(belt+roof)*0.5,z+(base+top)*0.5],
            [width*2.0-0.10,((roof-belt).powi(2)+(top-base).powi(2)).sqrt()-gap,0.012]);
        pane.pitch_deg=((top-base)/(roof-belt)).atan().to_degrees();panes.push(pane);
    }
    out.extend(seals(&id(rig,"glazing"),&panes));
    out.push(model(id(rig,"glazing"),group(panes),[0.46,0.62,0.65,1.0],FRAMED_GLASS));
    out.extend(interior(rig,x,z,belt-0.27,width,false));out
}

/// Window belts are made from separate wall strips around the apertures.
/// Their centre stays empty, so transparency reveals furniture and opposite windows.
fn dwelling(rig:u8,x:f32,z:f32,caravan:bool,paint:[f32;4])->Vec<AssetRecipe> {
    let (width,length,bottom,roof):(f32,f32,f32,f32)=if caravan {(0.90,1.58,0.36,1.68)}else{(0.92,1.78,0.30,1.70)};
    let name=if caravan {"caravan-body"}else{"body"};
    let glazing=if caravan {"caravan-glazing"}else{"glazing"};
    let windows=if caravan {vec![(-1.145,-0.455),(0.21,0.99)]}else{vec![(-1.06,-0.38),(0.04,0.76)]};
    let low=if caravan {1.06}else{0.99};let high=if caravan {1.46}else{1.51};
    let mut wall=Vec::new();
    for side in [-1.0,1.0] {
        let sx=x+side*width;
        wall.push(panel([sx,(high+roof-0.06)*0.5,z],[0.055,roof-0.06-high,length*2.0-0.30]));
        let mut previous=-length+0.15;
        for (a,b) in &windows {
            wall.push(panel([sx,(low+high)*0.5,z+(previous+a)*0.5],[0.055,high-low,a-previous]));
            previous=*b;
        }
        wall.push(panel([sx,(low+high)*0.5,z+(previous+length-0.15)*0.5],[0.055,high-low,length-0.15-previous]));
    }
    let mut panes=Vec::new();
    for side in [-1.0,1.0] {for (a,b) in &windows {
        panes.push(panel([x+side*(width+0.004),(low+high)*0.5,z+(a+b)*0.5],[0.012,high-low,b-a]));
    }}
    for end in [-1.0,1.0] {
        let ey=if !caravan&&end<0.0 {0.81}else{low};
        let ez=z+end*(length-0.08);let glass_width=width*1.52;
        wall.push(panel([x,(roof-0.06+high)*0.5,ez],[width*1.72,roof-0.06-high,0.055]));
        wall.push(panel([x,(ey+bottom+0.13)*0.5,ez],[width*1.72,ey-bottom-0.13,0.055]));
        for side in [-1.0,1.0] {wall.push(panel([x+side*width*0.81,(ey+high)*0.5,ez],[width*0.12,high-ey,0.055]));}
        panes.push(panel([x,(ey+high)*0.5,ez],[glass_width,high-ey,0.012]));
    }
    let gasket_models=seals(&id(rig,glazing),&panes);
    let mut out=vec![
        model(id(rig,name),crate::vehicles::coachwork(x,z,&[
            [-length,width*0.85,bottom+0.025,0.10,0.032],
            [-length+0.25,width,bottom,0.13,0.045],
            [length-0.25,width,bottom,0.13,0.045],
            [length,width*0.88,bottom+0.025,0.10,0.032]]),paint,PAINTED_METAL),
        model(id(rig,if caravan {"caravan-window-walls"}else{"window-walls"}),group(wall),paint,PAINTED_METAL),
        model(id(rig,if caravan {"caravan-roof-cap"}else{"roof-cap"}),crate::vehicles::coachwork(x,z,&[
            [-length+0.05,width*0.84,roof-0.13,0.08,0.028],[-length+0.25,width,roof-0.13,0.13,0.045],
            [length-0.25,width,roof-0.13,0.13,0.045],[length-0.05,width*0.87,roof-0.13,0.08,0.028]]),paint,PAINTED_METAL),
        model(id(rig,glazing),group(panes),[0.48,0.64,0.66,1.0],FRAMED_GLASS)];
    out.extend(gasket_models);
    for side in [-1.0,1.0] {
        out.push(model(id(rig,if caravan {
            if side<0.0 {"caravan-guard-left"}else{"caravan-guard-right"}
        }else if side<0.0 {"side-guard-left"}else{"side-guard-right"}),
            Shape::WheelClearancePanel{center_x:x+side*width,thickness:0.055,z:[z-length+0.15,z+length-0.15],
                bottom:bottom+0.13,top:[low;2],wheel_y:0.29,clearance_radius:if caravan {0.355}else{0.415},
                axles:if caravan {vec![z-0.13,z+0.42]}else{vec![z-1.23,z+1.12]},segments:128},paint,PAINTED_METAL));
    }
    if !caravan {out.extend(interior(rig,x,z-0.8,0.76,width,true));}
    let furniture_name=if caravan {"caravan-furniture"}else{"living-furniture"};
    out.push(model(id(rig,furniture_name),group(vec![
        panel([x-width*0.60,0.77,z+0.28],[0.48,0.72,0.76]),
        panel([x+width*0.55,0.58,z+0.85],[0.60,0.18,1.18]),
        panel([x+width*0.55,0.34,z+0.85],[0.055,0.46,1.06])]),[0.55,0.41,0.27,1.0],WEATHERED_TIMBER));
    out.push(model(id(rig,if caravan {"caravan-mattress"}else{"living-mattress"}),
        Shape::Ellipsoid{center:[x+width*0.55,0.73,z+0.87],radii:[0.30,0.14,0.59],latitude:18,longitude:32},
        [0.63,0.66,0.48,1.0],CLOTH_AND_SKIN));
    out
}

fn interior(rig:u8,x:f32,z:f32,seat_y:f32,width:f32,van:bool)->Vec<AssetRecipe> {
    let mut out=Vec::new();let name=prefix(rig);
    for (index,side) in [-1.0,1.0].into_iter().enumerate() {
        let sx=x+side*width*0.50;
        out.push(model(format!("rc:asset/rig/{name}/seat-{index}-cushion"),Shape::Ellipsoid{
            center:[sx,seat_y,z+0.05],radii:[width*0.27,0.085,0.23],latitude:18,longitude:24},[0.28,0.34,0.31,1.0],CLOTH_AND_SKIN));
        out.push(model(format!("rc:asset/rig/{name}/seat-{index}-back"),Shape::Ellipsoid{
            center:[sx,seat_y+0.20,z+0.24],radii:[width*0.26,0.27,0.082],latitude:18,longitude:24},[0.28,0.34,0.31,1.0],CLOTH_AND_SKIN));
        out.push(model(format!("rc:asset/rig/{name}/seat-{index}-headrest"),Shape::Ellipsoid{
            center:[sx,seat_y+0.48,z+0.26],radii:[0.13,0.10,0.062],latitude:16,longitude:20},[0.28,0.34,0.31,1.0],CLOTH_AND_SKIN));
    }
    let dash_z=z-if van {0.54}else{0.37};
    out.push(model(id(rig,"dashboard"),crate::vehicles::coachwork(x,dash_z,&[
        [-0.12,width*0.90,seat_y+0.08,0.17,0.045],[0.12,width*0.88,seat_y+0.08,0.14,0.045]]),
        [0.19,0.23,0.23,1.0],PAINTED_METAL));
    let wheel=[x+width*0.48,seat_y+0.28,dash_z+0.17];let mut limbs=Vec::new();
    for i in 0..32 {
        let a=i as f32*std::f32::consts::TAU/32.0;let b=(i+1) as f32*std::f32::consts::TAU/32.0;
        limbs.push(LimbSegment{start:[0.13*a.cos(),0.13*a.sin(),0.06*a.sin()],
            end:[0.13*b.cos(),0.13*b.sin(),0.06*b.sin()],start_radius:0.014,end_radius:0.014});
    }
    for end in [[0.12,0.0,0.0],[-0.12,0.0,0.0],[0.0,-0.11,-0.05]] {
        limbs.push(LimbSegment{start:[0.0;3],end,start_radius:0.015,end_radius:0.012});
    }
    out.push(model(id(rig,"steering-wheel"),Shape::LimbCluster{center:wheel,limbs,radial_segments:8},[0.12,0.16,0.15,1.0],PAINTED_METAL));
    out.push(model(id(rig,"gear-lever"),Shape::Stem{start:[x,seat_y-0.15,z-0.13],
        end:[x,seat_y+0.13,z-0.20],radius:0.015,segments:16},[0.38,0.42,0.40,1.0],GALVANISED_METAL));
    out
}

pub(crate) fn refine(rig:u8,assets:&mut Vec<AssetRecipe>) {
    let rig=rig.min(3);let x=2.35;let z=4.65;
    let mut replacements=if rig==0 {dwelling(rig,x,z+0.12,false,[0.88,0.84,0.73,1.0])}
        else {cab(rig,x,z,if rig==2 {[0.67,0.73,0.70,1.0]}else{[0.77,0.72,0.58,1.0]})};
    if rig==1 {replacements.extend(dwelling(rig,x,z+3.39,true,[0.88,0.85,0.74,1.0]));}
    assets.retain(|old|!replacements.iter().any(|new|new.id==old.id)||old.id.ends_with("window-frame"));
    assets.extend(replacements);
}

#[cfg(test)]
#[path="vehicle_cabins_tests.rs"]
mod tests;
