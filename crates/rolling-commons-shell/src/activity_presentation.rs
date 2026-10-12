//! Accepted event presentation policy, evaluated by VibeScript in WASM.
use crate::{GamePortal, GameSession};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {static ROUTER:RefCell<Option<vibe::Program>>=const {RefCell::new(None)};}
fn route(action: &str) -> Result<Option<(String, String, f64)>, JsValue> {
    ROUTER.with(|cached| {
        let mut cached = cached.borrow_mut();
        if cached.is_none() {
            *cached = Some(
                vibe::load_program(include_str!(
                    "../../../web/fixtures/resident-activities.vibe"
                ))
                .map_err(|_| JsValue::from_str("resident VibeScript policy could not load"))?,
            );
        }
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        let value = vibe::eval_function(
            cached.as_ref().unwrap(),
            "activity_for",
            vec![vibe::Value::String(action.to_owned())],
            &mut host,
            &mut env,
        )
        .map_err(|_| JsValue::from_str("resident VibeScript policy failed"))?;
        let vibe::Value::Record(record) = value else {
            return Err(JsValue::from_str("activity policy did not return a record"));
        };
        let Some(vibe::Value::String(actor)) = record.get("actor") else {
            return Err(JsValue::from_str("activity actor missing"));
        };
        if actor.is_empty() {
            return Ok(None);
        }
        let Some(vibe::Value::String(clip)) = record.get("clip") else {
            return Err(JsValue::from_str("activity clip missing"));
        };
        let Some(vibe::Value::F64(duration)) = record.get("duration") else {
            return Err(JsValue::from_str("activity duration missing"));
        };
        if actor.len() > 64
            || clip.len() > 64
            || !duration.is_finite()
            || !(0.01..=120.0).contains(duration)
        {
            return Err(JsValue::from_str("activity policy exceeds bounds"));
        }
        Ok(Some((actor.clone(), clip.clone(), *duration)))
    })
}

#[wasm_bindgen]
impl GameSession {
    /// The session checks its own accepted tape, not a browser-supplied action.
    pub fn present_accepted_activity(
        &self,
        portal: &mut GamePortal,
        sequence: u32,
    ) -> Result<u32, JsValue> {
        if sequence == 0 || sequence != self.seq {
            return Err(JsValue::from_str("activity receipt is not current"));
        }
        let action = self
            .events
            .last()
            .ok_or_else(|| JsValue::from_str("accepted activity event missing"))?;
        let Some((actor, clip, duration)) = route(action)? else {
            return Ok(0);
        };
        let token = qualia_core_db::q_hash(&format!("rc:animation/event/{sequence}/{action}"));
        portal.inner.present_scene_activity(
            token,
            qualia_core_db::q_hash(&format!("rc:rig/resident-{actor}/root")),
            qualia_core_db::q_hash(&format!("rc:clip/resident/{clip}")),
            qualia_core_db::q_hash("rc:clip/resident/idle"),
            duration,
            0.3,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn policy_routes_work_and_excludes_unrelated_actions() {
        assert_eq!(
            route("startGarden").unwrap(),
            Some(("ada".into(), "plant".into(), 10.0))
        );
        assert_eq!(
            route("repairTankLabour").unwrap(),
            Some(("bo".into(), "repair".into(), 8.0))
        );
        assert_eq!(
            route("reserveHarvest").unwrap(),
            Some(("cam".into(), "harvest".into(), 9.0))
        );
        assert_eq!(route("unknown-action").unwrap(), None);
    }
}
