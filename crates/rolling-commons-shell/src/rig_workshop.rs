//! Bounded VibeScript workshop operations persisted inside the shared RDF save.
//! Separate event namespace preserves the existing campaign tape/day semantics.
use crate::GameSession;
use serde::{Deserialize,Serialize};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

const MAX_OPERATIONS:u32=256;
const POLICY:&str=include_str!("../../../web/fixtures/rig-workshop.vibe");
thread_local! { static PROGRAM:RefCell<Option<vibe::Program>>=const {RefCell::new(None)}; }
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Assembly {
    version:f64, revision:f64, rig:f64, hood:bool, isolated:bool,
    diagnosed:bool, drained:bool, hose:bool, radiator:f64, filled:bool,
    cooling_verified:bool, controller:bool, fuse:bool, panel:bool,
    electrical_verified:bool, spare_radiator:f64, spare_panel:f64,
}
impl Assembly {
    pub(crate) fn visual_state(&self)->crate::workshop_assembly::VisualState {
        crate::workshop_assembly::VisualState {rig:self.rig as u8,hood:self.hood,
            hose:self.hose,radiator:self.radiator as i8,filled:self.filled,
            controller:self.controller,fuse:self.fuse,panel:self.panel}
    }
}
#[derive(Deserialize)]
struct Outcome {ok:bool,detail:String,state:Assembly}
const OPERATIONS:[(&str,&str);17]=[
    ("open","Open bonnet · hand"),("isolate","Isolate power · disconnect switch"),
    ("measure","Pressure test · gauge"),("drain","Drain coolant · catch tray"),
    ("remove-hose","Release hose clamps · socket"),("remove-radiator","Undo radiator mounts · socket"),
    ("install-compact","Fit compact radiator · socket"),("install-touring","Fit touring radiator · socket"),
    ("install-hose","Fit hoses and clamps · socket"),("fill","Fill cooling circuit · funnel"),
    ("controller","Mount controller · driver"),("fuse","Fit labelled protection cartridge · hand"),
    ("panel","Connect supplied solar panel · driver"),("reconnect","Reconnect power · switch"),
    ("close","Close bonnet · hand"),("test-cooling","Run cooling test · test stand"),
    ("test-electrical","Run electrical load test · meter")];

fn fail(message:&str)->JsValue {JsValue::from_str(message)}
pub(crate) fn value_json(value:&vibe::Value,depth:u8)->Result<serde_json::Value,JsValue> {
    if depth>8 {return Err(fail("workshop policy record exceeds depth bound"));}
    Ok(match value {
        vibe::Value::Bool(v)=>serde_json::Value::Bool(*v),
        vibe::Value::F64(v)=>serde_json::Value::Number(serde_json::Number::from_f64(*v)
            .ok_or_else(||fail("workshop policy returned non-finite number"))?),
        vibe::Value::String(v) if v.len()<=1024=>serde_json::Value::String(v.clone()),
        vibe::Value::Record(v) if v.len()<=64=>{
            let mut out=serde_json::Map::new();
            for (key,value) in v {out.insert(key.clone(),value_json(value,depth+1)?);}
            serde_json::Value::Object(out)
        },
        _=>return Err(fail("unsupported workshop policy value")),
    })
}
fn evaluate(function:&str,args:Vec<vibe::Value>)->Result<serde_json::Value,JsValue> {
    PROGRAM.with(|slot| {
        let mut slot=slot.borrow_mut();
        if slot.is_none() {*slot=Some(vibe::load_program(POLICY)
            .map_err(|_|fail("workshop VibeScript failed to load"))?);}
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let value=vibe::eval_function(slot.as_ref().unwrap(),function,args,
            &mut host,&mut vibe::Env::default())
            .map_err(|_|fail("workshop VibeScript evaluation failed"))?;
        value_json(&value,0)
    })
}
fn validate(state:&Assembly,rig:u8)->Result<(),JsValue> {
    let integer=|v:f64,min:f64,max:f64|v.is_finite()&&v.fract()==0.0&&(min..=max).contains(&v);
    if state.version!=1.0 || state.rig!=rig as f64 ||
        !integer(state.revision,0.0,MAX_OPERATIONS as f64) ||
        !integer(state.radiator,-1.0,2.0) || !integer(state.spare_radiator,0.0,1.0) ||
        !integer(state.spare_panel,0.0,1.0) ||
        (state.filled&&state.drained) || (state.radiator<0.0&&state.hose) ||
        (state.cooling_verified&&(!state.hose||!state.filled||state.radiator<=0.0)) ||
        (state.fuse&&!state.controller) || (state.panel&&(!state.controller||!state.fuse)) ||
        (state.electrical_verified&&!state.panel) {
        return Err(fail("workshop assembly invariants failed"));
    }
    Ok(())
}
fn initial(rig:u8)->Result<Assembly,JsValue> {
    if rig>3 {return Err(fail("unknown touring rig"));}
    let value=evaluate("workshop_initial",vec![vibe::Value::F64(rig as f64)])?;
    let state=serde_json::from_value(value).map_err(|_|fail("invalid initial assembly"))?;
    validate(&state,rig)?;Ok(state)
}
fn operation(state:&Assembly,command:&str)->Result<Outcome,JsValue> {
    if !OPERATIONS.iter().any(|(id,_)|*id==command) {return Err(fail("unknown workshop operation"));}
    let value=serde_json::to_value(state).map_err(|_|fail("assembly encoding failed"))?;
    let outcome:Outcome=serde_json::from_value(evaluate("workshop_step",vec![
        crate::json_to_vibe(&value),vibe::Value::String(command.to_owned())])?)
        .map_err(|_|fail("invalid workshop outcome"))?;
    validate(&outcome.state,state.rig as u8)?;
    if (outcome.ok && outcome.state.revision!=state.revision+1.0) ||
        (!outcome.ok && outcome.state!=*state) {return Err(fail("workshop receipt transition invalid"));}
    Ok(outcome)
}
fn prefix(rig:u8)->String {format!("rc:touringRig{rig} rc:assembly ")}
fn event_prefix(rig:u8,sequence:u32)->String {
    format!("<urn:civics:workshop:rig{rig}:e{sequence}> rc:operation ")
}
fn literal(line:&str,prefix:&str)->Result<String,JsValue> {
    let text=line.strip_prefix(prefix).and_then(|v|v.strip_suffix(" ."))
        .ok_or_else(||fail("invalid workshop RDF record"))?;
    serde_json::from_str(text).map_err(|_|fail("invalid workshop RDF literal"))
}
fn snapshot(session:&GameSession,rig:u8)->Result<Assembly,JsValue> {
    if rig>3 {return Err(fail("unknown touring rig"));}
    let prefix=prefix(rig);
    let mut saved=session.lines.iter().filter(|line|line.starts_with(&prefix));
    let snapshot=if let Some(line)=saved.next() {
        let json=literal(line,&prefix)?;
        if json.len()>4096 {return Err(fail("assembly snapshot exceeds bound"));}
        let value:Assembly=serde_json::from_str(&json).map_err(|_|fail("invalid assembly snapshot"))?;
        validate(&value,rig)?;
        if saved.next().is_some() {return Err(fail("duplicate assembly snapshot"));}
        value
    }else {initial(rig)?};
    Ok(snapshot)
}
pub(crate) fn tick_conditions(session:&GameSession,rig:u8)->Result<serde_json::Value,JsValue> {
    // Full tape verification occurs on entry/reload and after native mutation.
    let state=serde_json::to_value(snapshot(session,rig)?).map_err(|_|fail("assembly encoding failed"))?;
    evaluate("workshop_drive",vec![crate::json_to_vibe(&state)])
}
fn read(session:&GameSession,rig:u8)->Result<Assembly,JsValue> {
    let mut state=initial(rig)?;
    let snapshot=snapshot(session,rig)?;
    let namespace=format!("<urn:civics:workshop:rig{rig}:e");
    if session.lines.iter().filter(|line|line.starts_with(&namespace)).count()!=snapshot.revision as usize {
        return Err(fail("workshop event count does not match assembly revision"));
    }
    for sequence in 1..=snapshot.revision as u32 {
        let key=event_prefix(rig,sequence);
        let mut events=session.lines.iter().filter(|line|line.starts_with(&key));
        let command=literal(events.next().ok_or_else(||fail("missing workshop replay event"))?,&key)?;
        if events.next().is_some() {return Err(fail("duplicate workshop replay event"));}
        let next=operation(&state,&command)?;
        if !next.ok {return Err(fail("workshop replay rejected an event"));}
        state=next.state;
    }
    if state!=snapshot {return Err(fail("assembly differs from deterministic replay"));}
    Ok(state)
}

#[wasm_bindgen]
impl GameSession {
    pub fn workshop_scene(&self,rig:u8)->Result<JsValue,JsValue> {
        crate::workshop_assembly::scene(read(self,rig)?.visual_state())
    }
    /// Verify all workshop tapes before comparing the independent campaign
    /// replay. No lines are excluded until their mode's replay has passed.
    pub fn verify_connected_replay(&self,campaign_replayed_world:&str,simulation:&crate::GameDriveSimulation)->Result<bool,JsValue> {
        for rig in 0..4 {read(self,rig)?;}
        crate::drive_session::verify_checkpoints(self,simulation)?;
        let mode_line=|line:&str|crate::drive_session::is_checkpoint(line)||(0..4).any(|rig|line.starts_with(&prefix(rig))||
            line.starts_with(&format!("<urn:civics:workshop:rig{rig}:e")));
        let campaign=self.lines.iter().filter(|line|!mode_line(line)).cloned().collect::<Vec<_>>().join("\n")+"\n";
        Ok(campaign==campaign_replayed_world)
    }
    pub fn workshop_drive_conditions(&self,rig:u8)->Result<String,JsValue> {
        let state=read(self,rig)?;
        let value=serde_json::to_value(state).map_err(|_|fail("assembly encoding failed"))?;
        serde_json::to_string(&evaluate("workshop_drive",vec![crate::json_to_vibe(&value)])?)
            .map_err(|_|fail("rig condition encoding failed"))
    }
    pub fn workshop_view(&self,rig:u8)->Result<String,JsValue> {
        let state=read(self,rig)?;
        let operations:Vec<_>=OPERATIONS.iter().map(|(id,label)|{
            let part=match *id {
                "open"|"close"=>"bonnet",
                "measure"=>"gauge",
                "drain"|"fill"|"remove-hose"|"install-hose"=>"hose",
                "remove-radiator"|"install-compact"|"install-touring"|"test-cooling"=>"radiator",
                "controller"|"fuse"|"isolate"|"reconnect"=>"controller",
                "panel"|"test-electrical"=>"solar",_=>"",
            };
            serde_json::json!({"id":id,"label":label,"part":part})
        }).collect();
        serde_json::to_string(&serde_json::json!({"state":state,"operations":operations,
            "kit":"One compatible radiator and one protected solar kit supplied for this fictional puzzle."}))
            .map_err(|_|fail("workshop view encoding failed"))
    }
    pub fn workshop_operate(&mut self,rig:u8,command:&str,expected_revision:u32)->Result<String,JsValue> {
        let state=read(self,rig)?;
        if state.revision as u32!=expected_revision {return Err(fail("assembly changed; inspect it again"));}
        if expected_revision>=MAX_OPERATIONS {return Err(fail("workshop event budget reached"));}
        let outcome=operation(&state,command)?;
        let receipt=serde_json::to_string(&serde_json::json!({"ok":outcome.ok,
            "detail":outcome.detail,"state":outcome.state}))
            .map_err(|_|fail("workshop receipt encoding failed"))?;
        if outcome.ok {
            let encoded=serde_json::to_string(&outcome.state).map_err(|_|fail("assembly encoding failed"))?;
            let saved=format!("{}{} .",prefix(rig),serde_json::to_string(&encoded).unwrap());
            let event=format!("{}{} .",event_prefix(rig,expected_revision+1),serde_json::to_string(command).unwrap());
            let key=prefix(rig);
            self.lines.retain(|line|!line.starts_with(&key));
            self.lines.push(saved);self.lines.push(event);
            self.drive_verified[rig as usize]=false;
        }
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    fn cooling_puzzle_checks_evidence_compatibility_and_consumption() {
        let mut state=initial(1).unwrap();
        assert!(!operation(&state,"remove-hose").unwrap().ok);
        for command in ["open","isolate","measure","drain","remove-hose","remove-radiator"] {
            let result=operation(&state,command).unwrap();assert!(result.ok);state=result.state;
        }
        assert!(!operation(&state,"install-compact").unwrap().ok);
        for command in ["install-touring","install-hose","fill","reconnect","close","test-cooling"] {
            let result=operation(&state,command).unwrap();assert!(result.ok);state=result.state;
        }
        assert!(state.cooling_verified);assert_eq!(state.spare_radiator,0.0);
        assert!(!operation(&state,"test-cooling").unwrap().ok);
    }
    #[wasm_bindgen_test]
    fn saved_disassembly_replays_and_stale_receipts_cannot_consume_parts() {
        let mut session=GameSession{lines:Vec::new(),actions:Vec::new(),events:Vec::new(),seq:0,drive_verified:[false;4]};
        for (revision,command) in ["open","isolate","measure","drain","remove-hose","remove-radiator"].iter().enumerate() {
            session.workshop_operate(0,command,revision as u32).unwrap();
        }
        let before=session.world();
        assert!(session.workshop_operate(0,"install-compact",0).is_err());
        assert_eq!(session.world(),before);
        let loaded=GameSession{lines:before.lines().map(str::to_owned).collect(),actions:Vec::new(),events:Vec::new(),seq:0,drive_verified:[false;4]};
        assert_eq!(read(&session,0).unwrap(),read(&loaded,0).unwrap());
        assert!(session.events.is_empty());assert_eq!(session.seq,0);
        let mut corrupt=loaded;
        corrupt.lines.retain(|line|!line.starts_with(&event_prefix(0,3)));
        assert!(read(&corrupt,0).is_err());
    }
    #[wasm_bindgen_test]
    fn solar_upgrade_preserves_kit_and_requires_load_verification() {
        let mut state=initial(2).unwrap();
        assert!(!operation(&state,"test-electrical").unwrap().ok);
        for command in ["open","isolate","controller","fuse","panel","reconnect","close","test-electrical"] {
            let next=operation(&state,command).unwrap();assert!(next.ok);state=next.state;
        }
        assert!(state.electrical_verified);assert_eq!(state.spare_panel,0.0);
        assert!(!operation(&state,"test-electrical").unwrap().ok);
    }
}
