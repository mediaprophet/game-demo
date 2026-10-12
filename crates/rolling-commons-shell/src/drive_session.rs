//! Native-owned touring condition. Bounded latest-tick checkpoints live in the
//! shared RDF document; they are not a substitute for a full driving event tape.
use crate::{GameSession,GameDriveSimulation};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use wasm_bindgen::prelude::*;

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Checkpoint {
    version:u8,rig:u8,policy_hash:u64,contact_hash:u64,ticks:u64,state:Value,
    previous:Option<Value>,input:Option<Value>,
}
fn fail(text:&str)->JsValue {JsValue::from_str(text)}
fn prefix(rig:u8)->String {format!("rc:touringRig{rig} rc:driveCheckpoint ")}
pub(crate) fn is_checkpoint(line:&str)->bool {(0..4).any(|rig|line.starts_with(&prefix(rig)))}
pub(crate) fn evaluate_raw(sim:&GameDriveSimulation,function:&str,args:Vec<vibe::Value>)->Result<Value,JsValue> {
    let value=vibe::eval_function(&sim.program,function,args,&mut qualia_core_db::poet_host::PoetSnapshot::default(),&mut vibe::Env::default())
        .map_err(|_|fail("native drive policy failed"))?;
    crate::rig_workshop::value_json(&value,0)
}
fn evaluate(sim:&GameDriveSimulation,function:&str,mut args:Vec<vibe::Value>)->Result<Value,JsValue> {
    if function=="drive_contact_step" {crate::drive_contact_integration::prepare(sim,&mut args)?;}
    evaluate_raw(sim,function,args)
}
fn number(state:&Value,key:&str,min:f64,max:f64)->Result<f64,JsValue> {
    let n=state.get(key).and_then(Value::as_f64).ok_or_else(||fail("drive numeric field missing"))?;
    if !n.is_finite()||!(min..=max).contains(&n) {return Err(fail("drive numeric field exceeds bound"));}
    Ok(n)
}
fn validate(state:&Value)->Result<(),JsValue> {
    let record=state.as_object().ok_or_else(||fail("drive state must be a record"))?;
    if record.len()>64 {return Err(fail("drive state field budget exceeded"));}
    for field in ["x","z"] {number(state,field,-1e9,1e9)?;}
    number(state,"heading",-1e12,1e12)?;number(state,"speed",-1000.0,1000.0)?;
    number(state,"oil_temp",-50.0,250.0)?;number(state,"coolant_temp",-50.0,200.0)?;
    number(state,"engine_health",0.0,100.0)?;number(state,"distance",0.0,1e15)?;
    number(state,"wheel_angle",-1e15,1e15)?;number(state,"steering_angle",-1.0,1.0)?;
    number(state,"wheel_radius",0.1,2.0)?;
    number(state,"trailer_heading",-1e12,1e12)?;
    number(state,"trailer_wheel_angle",-1e15,1e15)?;
    number(state,"trailer_articulation",-std::f64::consts::PI,std::f64::consts::PI)?;
    number(state,"body_height",-1e8,1e8)?;number(state,"body_velocity",-3.0,3.0)?;
    for field in ["body_pitch","body_roll","trailer_pitch","trailer_roll"] {number(state,field,-0.41,0.41)?;}
    for field in ["suspension_fl","suspension_fr","suspension_rl","suspension_rr",
        "suspension_tfl","suspension_tfr","suspension_trl","suspension_trr"] {number(state,field,-0.18,0.18)?;}
    if state.get("contact_initialized").and_then(Value::as_bool).is_none() {return Err(fail("drive contact flag missing"));}
    for field in ["overheat_seconds","hazard_stops","emergency_stops"] {number(state,field,0.0,1e12)?;}
    if !matches!(state.get("period").and_then(Value::as_str),Some("dawn"|"day"|"dusk"|"night")) {
        return Err(fail("drive period invalid"));
    }
    Ok(())
}
fn initial(sim:&GameDriveSimulation,rig:u8)->Result<Checkpoint,JsValue> {
    if rig>3 {return Err(fail("unknown touring rig"));}
    let state=evaluate(sim,"drive_setup",vec![vibe::Value::F64(rig as f64)])?;
    validate(&state)?;
    Ok(Checkpoint{version:1,rig,policy_hash:sim.policy_hash,contact_hash:crate::drive_contacts::SOURCE_HASH,ticks:0,state,previous:None,input:None})
}
fn next(sim:&GameDriveSimulation,previous:&Value,input:&Value)->Result<Value,JsValue> {
    if input.get("park")==Some(&Value::Bool(true)) {
        let patch=evaluate(sim,"drive_park",Vec::new())?;
        let mut parked=previous.clone();
        let record=parked.as_object_mut().ok_or_else(||fail("drive state must be a record"))?;
        for (key,value) in patch.as_object().ok_or_else(||fail("drive parking policy failed"))? {
            record.insert(key.clone(),value.clone());
        }
        if previous.get("contact_initialized")==Some(&Value::Bool(false)) {
            let rig=number(input,"rig_id",0.0,3.0)? as u8;
            let contact=crate::drive_contacts::sample(rig,&parked)?;
            let result=evaluate(sim,"drive_contact_step",vec![crate::json_to_vibe(&parked),crate::json_to_vibe(previous),
                crate::json_to_vibe(&contact),vibe::Value::F64(0.0)])?;
            parked.as_object_mut().unwrap().extend(result.as_object().ok_or_else(||fail("parking contact output invalid"))?.clone());
        }
        validate(&parked)?;return Ok(parked);
    }
    let mut state=evaluate(sim,"drive_step",vec![crate::json_to_vibe(input)])?;
    let delta=number(&state,"distance_delta",0.0,100.0)?;
    let distance=number(previous,"distance",0.0,1e15)?+delta;
    let record=state.as_object_mut().ok_or_else(||fail("drive output is not a record"))?;
    record.insert("distance".into(),json!(distance));
    record.insert("period".into(),input.get("period").cloned().ok_or_else(||fail("drive period missing"))?);
    let rig=number(input,"rig_id",0.0,3.0)? as u8;
    let contact=crate::drive_contacts::sample(rig,&state)?;
    let contact_state=evaluate(sim,"drive_contact_step",vec![crate::json_to_vibe(&state),
        crate::json_to_vibe(previous),crate::json_to_vibe(&contact),
        vibe::Value::F64(number(input,"dt",0.0,0.05)?)])?;
    state.as_object_mut().unwrap().extend(contact_state.as_object().ok_or_else(||fail("contact policy output invalid"))?.clone());
    validate(&state)?;
    if number(&state,"engine_health",0.0,100.0)?>number(previous,"engine_health",0.0,100.0)? {
        return Err(fail("drive tick cannot repair engine damage"));
    }
    Ok(state)
}
fn load(session:&GameSession,sim:&GameDriveSimulation,rig:u8,verify:bool)->Result<Checkpoint,JsValue> {
    let key=prefix(rig);let mut records=session.lines.iter().filter(|line|line.starts_with(&key));
    let Some(line)=records.next() else {return initial(sim,rig);};
    if records.next().is_some()||line.len()>65_536 {return Err(fail("duplicate or oversized drive checkpoint"));}
    let literal=line.strip_prefix(&key).and_then(|line|line.strip_suffix(" .")).ok_or_else(||fail("drive RDF record malformed"))?;
    let encoded:String=serde_json::from_str(literal).map_err(|_|fail("drive RDF literal malformed"))?;
    let checkpoint:Checkpoint=serde_json::from_str(&encoded).map_err(|_|fail("drive checkpoint malformed"))?;
    if checkpoint.version!=1||checkpoint.rig!=rig||checkpoint.policy_hash!=sim.policy_hash||checkpoint.contact_hash!=crate::drive_contacts::SOURCE_HASH {
        return Err(fail("drive checkpoint requires its original version and policy"));
    }
    validate(&checkpoint.state)?;
    if verify {
        if checkpoint.ticks==0 {
            if checkpoint.previous.is_some()||checkpoint.input.is_some()||checkpoint.state!=initial(sim,rig)?.state {
                return Err(fail("initial drive checkpoint differs from policy"));
            }
        }else {
            let previous=checkpoint.previous.as_ref().ok_or_else(||fail("drive previous state missing"))?;
            validate(previous)?;
            let input=checkpoint.input.as_ref().ok_or_else(||fail("drive checkpoint input missing"))?;
            if checkpoint.state!=next(sim,previous,input)? {return Err(fail("latest drive tick does not replay"));}
        }
    }
    Ok(checkpoint)
}
pub(crate) fn verify_checkpoints(session:&GameSession,sim:&GameDriveSimulation)->Result<(),JsValue> {
    for rig in 0..4 {load(session,sim,rig,true)?;}
    Ok(())
}
fn persist(session:&mut GameSession,checkpoint:&Checkpoint)->Result<(),JsValue> {
    let encoded=serde_json::to_string(checkpoint).map_err(|_|fail("drive checkpoint encoding failed"))?;
    let literal=serde_json::to_string(&encoded).map_err(|_|fail("drive checkpoint literal encoding failed"))?;
    if literal.len()>65_000 {return Err(fail("drive checkpoint exceeds persistence budget"));}
    let key=prefix(checkpoint.rig);let line=format!("{key}{literal} .");
    session.lines.retain(|line|!line.starts_with(&key));session.lines.push(line);
    Ok(())
}

#[wasm_bindgen]
impl GameSession {
    /// Native camera mount follows the same accepted pose as the touring body.
    pub fn drive_camera(&self,sim:&GameDriveSimulation,rig:u8,aerial:bool,look_yaw:f32,look_pitch:f32,
        center_x:f64,center_y:f64,center_z:f64,scale:f64)->Result<String,JsValue> {
        if [center_x,center_y,center_z,scale].iter().any(|v|!v.is_finite())||scale<=0.0||
            !look_yaw.is_finite()||!look_pitch.is_finite() {return Err(fail("drive camera input invalid"));}
        let checkpoint=load(self,sim,rig,false)?;
        crate::vehicle_camera::view(rig,&checkpoint.state,aerial,[look_yaw,look_pitch],[center_x,center_y,center_z],scale)
    }
    /// Apply accepted native driving state to the admitted touring bind rig.
    /// No browser-supplied model transform or wheel phase is accepted here.
    pub fn present_drive_rig(&self,sim:&GameDriveSimulation,portal:&mut crate::GamePortal,rig:u8)->Result<u32,JsValue> {
        use qualia_core_db::render::skeletal::Motor;
        let checkpoint=load(self,sim,rig,false)?;
        if checkpoint.ticks==0 {return Ok(0);}
        let state=&checkpoint.state;
        let x=number(state,"x",-1e9,1e9)?;let z=number(state,"z",-1e9,1e9)?;
        let heading=number(state,"heading",-1e12,1e12)?.rem_euclid(std::f64::consts::TAU);
        let wheel=number(state,"wheel_angle",-1e15,1e15)?;
        let trailer_wheel=number(state,"trailer_wheel_angle",-1e15,1e15)?;
        let trailer_heading=number(state,"trailer_heading",-1e12,1e12)?;
        let steering=number(state,"steering_angle",-1.0,1.0)?;
        let pitch=number(state,"body_pitch",-0.41,0.41)?;let roll=number(state,"body_roll",-0.41,0.41)?;
        let yaw=Motor::from_rotation_translation([(heading*0.5).cos(),0.0,(heading*0.5).sin(),0.0],[0.0;3]);
        let slope=Motor::from_rotation_translation([(pitch*0.5).cos(),(pitch*0.5).sin(),0.0,0.0],[0.0;3]);
        let bank=Motor::from_rotation_translation([(roll*0.5).cos(),0.0,0.0,(roll*0.5).sin()],[0.0;3]);
        let orientation=yaw.mul(&slope).mul(&bank);
        let y=number(state,"body_height",-1e8,1e8)?;
        let root=crate::vehicle_camera::body_pose(state)?.mul(&Motor::from_translation(-2.35,0.0,-4.65));
        let group=crate::vehicle_animation::root_id(rig);
        let count=portal.inner.set_scene_actor_root(group,root)?;
        let hitch=crate::vehicle_animation::trailer_id(rig);
        if hitch!=0 {
            let tp=number(state,"trailer_pitch",-0.41,0.41)?;let tr=number(state,"trailer_roll",-0.41,0.41)?;
            let desired=Motor::from_rotation_translation([(trailer_heading*0.5).cos(),0.0,(trailer_heading*0.5).sin(),0.0],[0.0;3])
                .mul(&Motor::from_rotation_translation([(tp*0.5).cos(),(tp*0.5).sin(),0.0,0.0],[0.0;3]))
                .mul(&Motor::from_rotation_translation([(tr*0.5).cos(),0.0,0.0,(tr*0.5).sin()],[0.0;3]));
            let (q,_)=orientation.to_rotation_translation();
            let inverse=Motor::from_rotation_translation([q[0],-q[1],-q[2],-q[3]],[0.0;3]);
            portal.inner.set_scene_joint_delta(group,hitch,inverse.mul(&desired))?;
        }
        for (index,&(joint,front,trailer)) in crate::vehicle_animation::motion_joints(rig).iter().enumerate() {
            let angle=(if trailer {trailer_wheel}else{wheel}).rem_euclid(std::f64::consts::TAU);
            let spin=Motor::from_rotation_translation([(angle*0.5).cos(),(angle*0.5).sin(),0.0,0.0],[0.0;3]);
            let turn=if front {steering}else{0.0};
            let steer=Motor::from_rotation_translation([(turn*0.5).cos(),0.0,(turn*0.5).sin(),0.0],[0.0;3]);
            let key=["suspension_fl","suspension_fr","suspension_rl","suspension_rr",
                "suspension_tfl","suspension_tfr","suspension_trl","suspension_trr"][index];
            let travel=number(state,key,-0.18,0.18)?;
            portal.inner.set_scene_joint_delta(group,joint,Motor::from_translation(0.0,travel,0.0).mul(&steer).mul(&spin))?;
        }
        crate::fauna_animation::present_alerts(portal,state)?;
        Ok(count)
    }
    pub fn drive_park(&mut self,sim:&GameDriveSimulation,rig:u8)->Result<(),JsValue> {
        let previous=load(self,sim,rig,true)?;
        let input=json!({"park":true,"rig_id":rig});let state=next(sim,&previous.state,&input)?;
        if state.get("engine_health")!=previous.state.get("engine_health")||
            state.get("distance")!=previous.state.get("distance")||state.get("speed")!=Some(&json!(0.0)) {
            return Err(fail("parking may not change engine condition or distance"));
        }
        persist(self,&Checkpoint{version:1,rig,policy_hash:sim.policy_hash,contact_hash:crate::drive_contacts::SOURCE_HASH,
            ticks:previous.ticks.checked_add(1).ok_or_else(||fail("drive tick counter exhausted"))?,
            state,previous:Some(previous.state),input:Some(input)})?;
        if rig<4 {self.drive_verified[rig as usize]=false;}
        Ok(())
    }
    pub fn drive_resume(&self,sim:&GameDriveSimulation,rig:u8)->Result<String,JsValue> {
        self.workshop_drive_conditions(rig)?;
        serde_json::to_string(&load(self,sim,rig,true)?.state).map_err(|_|fail("drive state encoding failed"))
    }
    /// Browser supplies controls and sampled scene facts. Saved motion,
    /// condition, counters and distance override any browser state fields.
    pub fn drive_tick(&mut self,sim:&GameDriveSimulation,rig:u8,facts_json:&str)->Result<String,JsValue> {
        if facts_json.len()>16_384 {return Err(fail("drive facts exceed 16 KiB"));}
        let mut facts:Value=serde_json::from_str(facts_json).map_err(|_|fail("drive facts malformed"))?;
        number(&facts,"dt",0.0,0.05)?;
        if rig>3 {return Err(fail("unknown touring rig"));}
        let verify=!self.drive_verified[rig as usize];
        if verify {self.workshop_drive_conditions(rig)?;}
        let record=facts.as_object_mut().ok_or_else(||fail("drive facts must be a record"))?;
        let previous=load(self,sim,rig,verify)?;
        let road=crate::drive_contacts::road_samples(&previous.state)?;
        for (key,value) in ["terrain_height","terrain_height_ahead","terrain_height_behind"].into_iter().zip(road) {
            record.insert(key.into(),json!(value));
        }
        record.insert("terrain_sample_distance".into(),json!(0.55));
        for key in ["x","z","heading","speed","oil_temp","coolant_temp","engine_health",
            "overheat_seconds","traffic_queue","hazard_stops","emergency_stops","last_warning","wheel_angle",
            "trailer_heading","trailer_wheel_angle"] {
            record.insert(key.into(),previous.state.get(key).cloned().ok_or_else(||fail("saved drive field missing"))?);
        }
        record.insert("rig_id".into(),json!(rig));
        let conditions=crate::rig_workshop::tick_conditions(self,rig)?;
        for key in ["rig_roadworthy","cooling_efficiency"] {
            record.insert(key.into(),conditions.get(key).cloned().ok_or_else(||fail("native rig condition missing"))?);
        }
        let state=next(sim,&previous.state,&facts)?;
        let result=serde_json::to_string(&state).map_err(|_|fail("drive output encoding failed"))?;
        let checkpoint=Checkpoint{version:1,rig,policy_hash:sim.policy_hash,contact_hash:crate::drive_contacts::SOURCE_HASH,
            ticks:previous.ticks.checked_add(1).ok_or_else(||fail("drive tick counter exhausted"))?,
            state,previous:Some(previous.state),input:Some(facts)};
        persist(self,&checkpoint)?;
        self.drive_verified[rig as usize]=true;
        Ok(result)
    }
}

#[cfg(test)]
#[path="drive_session_tests.rs"]
mod tests;
