//! Authored right-hand driver mounts; camera math remains Qualia-owned.
use serde_json::{Value,json};
use wasm_bindgen::prelude::*;
use qualia_core_db::render::skeletal::Motor;
pub(crate) fn body_pose(state:&Value)->Result<Motor,JsValue> {
    let n=|key:&str|state.get(key).and_then(Value::as_f64).filter(|v|v.is_finite())
        .ok_or_else(||JsValue::from_str("vehicle pose field missing"));
    let yaw=n("heading")?.rem_euclid(std::f64::consts::TAU)*0.5;
    let pitch=n("body_pitch")?*0.5;let roll=n("body_roll")?*0.5;
    Ok(Motor::from_translation(n("x")?,n("body_height")?,n("z")?)
        .mul(&Motor::from_rotation_translation([yaw.cos(),0.0,yaw.sin(),0.0],[0.0;3]))
        .mul(&Motor::from_rotation_translation([pitch.cos(),pitch.sin(),0.0,0.0],[0.0;3]))
        .mul(&Motor::from_rotation_translation([roll.cos(),0.0,0.0,roll.sin()],[0.0;3])))
}
pub(crate) fn view(rig:u8,state:&Value,aerial:bool,look:[f32;2],center:[f64;3],scale:f64)->Result<String,JsValue> {
    if rig>3 {return Err(JsValue::from_str("camera rig invalid"));}
    let pose=body_pose(state)?;
    if aerial {
        let point=pose.transform_point([0.0,0.35,0.0]);
        let target=std::array::from_fn::<_,3,_>(|i|((point[i]-center[i])*scale) as f32);
        let heading=state["heading"].as_f64().unwrap() as f32;
        return Ok(json!({"yaw":heading,"pitch":0.92,"zoom":3.8,"target":target}).to_string());
    }
    // Match seat center (width*.48), cushion height + seated eye height, and
    // cab forward offset. Campervan interior is authored 0.8 m forward.
    let eye=match rig {0=>[0.40,1.42,-0.75],2=>[0.3216,1.16,0.05],_=>[0.3408,1.40,0.05]};
    let camera=qualia_core_db::render::camera_mount::mounted_camera(pose,eye,look,center,scale)
        .map_err(|_|JsValue::from_str("native mounted camera refused placement"))?;
    Ok(json!({"yaw":camera.yaw,"pitch":camera.pitch,"zoom":camera.zoom,"target":camera.target,"eye":camera.eye}).to_string())
}
