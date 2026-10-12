//! Native sampling of the authored terrain at model contact locations.
//! VibeScript owns suspension parameters; Qualia owns integration.
use serde_json::{Value,json};
use wasm_bindgen::prelude::*;
use qualia_core_db::render::skeletal::Motor;
pub(crate) const SOURCE_HASH:u64=qualia_core_db::q_hash(include_str!("terrain.rs"))
    ^ qualia_core_db::q_hash(include_str!("drive_contact_integration.rs")).rotate_left(11)
    ^ qualia_core_db::render::physics::response::SOURCE_HASH.rotate_left(29);
fn number(state:&Value,key:&str)->Result<f64,JsValue> {
    state.get(key).and_then(Value::as_f64).filter(|n|n.is_finite())
        .ok_or_else(||JsValue::from_str("contact motion field missing"))
}
pub(crate) fn sample(rig:u8,state:&Value)->Result<Value,JsValue> {
    let x=number(state,"x")?;let z=number(state,"z")?;let heading=number(state,"heading")?;
    let rotation=Motor::from_rotation_translation([(heading*0.5).cos(),0.0,(heading*0.5).sin(),0.0],[x,0.0,z]);
    let (front,rear,half_track)=match rig {0=>(-1.11,1.24,0.94),2=>(-0.84,0.82,0.86),_=>(-0.77,0.78,0.86)};
    let height=|motor:Motor,x:f64,z:f64| {
        let point=motor.transform_point([x,0.0,z]);
        crate::terrain_height_world(point[0] as f32,point[2] as f32) as f64
    };
    let mut contacts=json!({"fl":height(rotation,-half_track,front),"fr":height(rotation,half_track,front),
        "rl":height(rotation,-half_track,rear),"rr":height(rotation,half_track,rear),
        "front_z":front,"rear_z":rear,"track":half_track*2.0});
    let trailer_heading=number(state,"trailer_heading")?;
    let hitch_offset=if rig==1 {1.36}else{1.55};
    let hitch=rotation.transform_point([0.0,0.0,hitch_offset]);
    let trailer=Motor::from_rotation_translation([(trailer_heading*0.5).cos(),0.0,(trailer_heading*0.5).sin(),0.0],[hitch[0],0.0,hitch[2]]);
    let (a,b,trailer_track)=if rig==1 {(1.90,2.45,1.86)}else{(1.10,1.10,1.56)};
    let record=contacts.as_object_mut().unwrap();
    for (key,value) in [("tfl",height(trailer,-trailer_track*0.5,a)),("tfr",height(trailer,trailer_track*0.5,a)),
        ("trl",height(trailer,-trailer_track*0.5,b)),("trr",height(trailer,trailer_track*0.5,b)),
        ("trailer_front_z",a),("trailer_rear_z",b),("trailer_track",trailer_track),("hitch_offset",hitch_offset)] {
        if !value.is_finite() {return Err(JsValue::from_str("contact terrain height invalid"));}
        record.insert(key.into(),json!(value));
    }
    Ok(contacts)
}
pub(crate) fn road_samples(state:&Value)->Result<[f64;3],JsValue> {
    let x=number(state,"x")?;let z=number(state,"z")?;let heading=number(state,"heading")?;
    let motor=Motor::from_rotation_translation([(heading*0.5).cos(),0.0,(heading*0.5).sin(),0.0],[x,0.0,z]);
    let mut output=[0.0;3];
    for (index,offset) in [0.0,-0.55,0.55].into_iter().enumerate() {
        let point=motor.transform_point([0.0,0.0,offset]);
        output[index]=crate::terrain_height_world(point[0] as f32,point[2] as f32) as f64;
        if !output[index].is_finite() {return Err(JsValue::from_str("road contact height invalid"));}
    }
    Ok(output)
}
