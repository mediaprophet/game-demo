//! Campaign advisor and decision state machine evaluated via VibeScript in WASM.
use crate::{err_obj, json_to_vibe, vibe_value_to_js};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CampaignSimulation {
    program: vibe::Program,
    policy_hash: u64,
}

#[wasm_bindgen]
impl CampaignSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<CampaignSimulation, JsValue> {
        if source.len() > 65_536 {
            return Err(JsValue::from_str("campaign policy exceeds 64 KiB bound"));
        }
        let program = vibe::load_program(source)
            .map_err(|error| JsValue::from_str(&format!("VibeScript campaign module: {error}")))?;
        Ok(CampaignSimulation {
            program,
            policy_hash: qualia_core_db::q_hash(source),
        })
    }

    pub fn policy_hash(&self) -> u64 {
        self.policy_hash
    }

    /// Evaluates the recommended next campaign lead/choice given world state.
    pub fn lead(&self, st_json: &str) -> JsValue {
        let input: serde_json::Value = match serde_json::from_str(st_json) {
            Ok(v) => v,
            Err(e) => return err_obj("CampaignSimulation state", &e.to_string()),
        };
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "campaign_lead",
            vec![json_to_vibe(&input)],
            &mut host,
            &mut env,
        ) {
            Ok(value) => vibe_value_to_js(&value),
            Err(error) => err_obj("VibeScript campaign lead", &error.to_string()),
        }
    }
}
