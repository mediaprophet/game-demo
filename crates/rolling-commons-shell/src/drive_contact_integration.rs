//! Game policy selects spring facts; Qualia owns numerical integration.
use qualia_core_db::render::physics::response::{step_spring, DrivenSpring, SpringState};
use serde_json::Value;
use wasm_bindgen::prelude::*;
pub(crate) fn prepare(
    sim: &crate::GameDriveSimulation,
    args: &mut [vibe::Value],
) -> Result<(), JsValue> {
    if args.len() != 4 {
        return Err(JsValue::from_str("contact arguments invalid"));
    }
    let parameters =
        super::drive_session::evaluate_raw(sim, "drive_contact_parameters", args.to_vec())?;
    let field = |name: &str| {
        parameters
            .get(name)
            .and_then(Value::as_f64)
            .filter(|n| n.is_finite())
            .ok_or_else(|| JsValue::from_str("contact spring parameter missing"))
    };
    let target = field("target")?;
    let dt = field("dt")?;
    if !(0.0..=0.05).contains(&dt) {
        return Err(JsValue::from_str("contact interval exceeds bound"));
    }
    let reset = parameters
        .get("reset")
        .and_then(Value::as_bool)
        .ok_or_else(|| JsValue::from_str("contact reset flag missing"))?;
    let state = if reset {
        SpringState {
            position: target,
            velocity: 0.0,
        }
    } else {
        step_spring(
            SpringState {
                position: field("position")?,
                velocity: field("velocity")?,
            },
            DrivenSpring {
                target,
                stiffness: field("stiffness")?,
                damping: field("damping")?,
                max_displacement: field("travel")?,
                max_speed: field("speed_limit")?,
            },
            dt,
        )
        .map_err(|_| JsValue::from_str("native contact integrator refused parameters"))?
    };
    let vibe::Value::Record(contacts) = &mut args[2] else {
        return Err(JsValue::from_str("contact facts must be a record"));
    };
    contacts.insert("integrated_height".into(), vibe::Value::F64(state.position));
    contacts.insert(
        "integrated_velocity".into(),
        vibe::Value::F64(state.velocity),
    );
    Ok(())
}
