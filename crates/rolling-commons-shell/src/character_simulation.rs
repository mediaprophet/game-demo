//! Character arcs, traits and narrative goals evaluated via VibeScript in WASM.
use crate::{err_obj, json_to_vibe, vibe_value_to_js};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CharacterSimulation {
    program: vibe::Program,
    policy_hash: u64,
}

#[wasm_bindgen]
impl CharacterSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<CharacterSimulation, JsValue> {
        if source.len() > 65_536 {
            return Err(JsValue::from_str("character policy exceeds 64 KiB bound"));
        }
        let program = vibe::load_program(source)
            .map_err(|error| JsValue::from_str(&format!("VibeScript character module: {error}")))?;
        Ok(CharacterSimulation {
            program,
            policy_hash: qualia_core_db::q_hash(source),
        })
    }

    pub fn policy_hash(&self) -> u64 {
        self.policy_hash
    }

    /// Evaluates chapter title, goal, and opening based on world facts.
    pub fn chapter(&self, arc: &str, world_json: &str) -> JsValue {
        let input: serde_json::Value = match serde_json::from_str(world_json) {
            Ok(v) => v,
            Err(e) => return err_obj("CharacterSimulation world", &e.to_string()),
        };
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "character_chapter",
            vec![vibe::Value::String(arc.to_owned()), json_to_vibe(&input)],
            &mut host,
            &mut env,
        ) {
            Ok(value) => vibe_value_to_js(&value),
            Err(error) => err_obj("VibeScript character chapter", &error.to_string()),
        }
    }

    /// Evaluates movement speed given character trait and running flag.
    pub fn walk_speed(&self, trait_id: &str, running: bool) -> f64 {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "walk_speed",
            vec![
                vibe::Value::String(trait_id.to_owned()),
                vibe::Value::Bool(running),
            ],
            &mut host,
            &mut env,
        ) {
            Ok(vibe::Value::F64(speed)) => speed,
            _ => {
                if running {
                    5.0
                } else {
                    2.6
                }
            }
        }
    }

    /// Evaluates detection / interaction radius given character trait.
    pub fn nearby_radius(&self, trait_id: &str) -> f64 {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "nearby_radius",
            vec![vibe::Value::String(trait_id.to_owned())],
            &mut host,
            &mut env,
        ) {
            Ok(vibe::Value::F64(radius)) => radius,
            _ => 3.6,
        }
    }

    /// Evaluates biography description for a character name.
    pub fn bio(&self, name: &str) -> String {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "character_bio",
            vec![vibe::Value::String(name.to_owned())],
            &mut host,
            &mut env,
        ) {
            Ok(vibe::Value::String(bio)) => bio,
            _ => "A new neighbour, ready to leave a mark on the valley.".to_owned(),
        }
    }
}
