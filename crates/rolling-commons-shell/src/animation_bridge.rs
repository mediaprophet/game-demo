//! Game-facing controls delegate playback and identity resolution to Qualia.
use crate::GamePortal;
use wasm_bindgen::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct AnimationSave {
    world_fingerprint: u32,
    timeline: String,
}

#[wasm_bindgen]
impl GamePortal {
    pub fn play_character_animation(
        &mut self,
        joint_iri: &str,
        clip_iri: &str,
        fade_seconds: f64,
    ) -> Result<u32, JsValue> {
        if joint_iri.is_empty()
            || clip_iri.is_empty()
            || joint_iri.len() > 256
            || clip_iri.len() > 256
        {
            return Err(JsValue::from_str("animation identity is invalid"));
        }
        self.inner.select_scene_animation_clip(
            qualia_core_db::q_hash(joint_iri),
            qualia_core_db::q_hash(clip_iri),
            fade_seconds,
        )
    }
    pub fn set_animation_reduced_motion(&mut self, reduced: bool) {
        self.inner.set_animation_reduced_motion(reduced);
    }
    pub fn seek_scene_animation(&mut self, time: f64) -> Result<(), JsValue> {
        self.inner.seek_scene_animation(time)
    }
    pub fn scene_animation_receipt(&self) -> Result<JsValue, JsValue> {
        self.inner.scene_animation_receipt()
    }
    pub fn animation_snapshot_json(&self) -> Result<String, JsValue> {
        self.inner.animation_snapshot_json()
    }
    pub fn restore_animation_snapshot_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.inner.restore_animation_snapshot_json(json)
    }
    pub fn clear_activity_presentation(&mut self) -> Result<(), JsValue> {
        self.inner
            .clear_scene_activities(qualia_core_db::q_hash("rc:clip/resident/idle"))
    }
    pub fn animation_save_for_world(&self, world: &str) -> Result<String, JsValue> {
        let save = AnimationSave {
            world_fingerprint: qualia_core_db::container_10d::crc32c::crc32c(world.as_bytes()),
            timeline: self.inner.animation_snapshot_json()?,
        };
        serde_json::to_string(&save).map_err(|_| JsValue::from_str("visual save encoding failed"))
    }
    pub fn restore_animation_for_world(&mut self, json: &str, world: &str) -> Result<(), JsValue> {
        if json.len() > 32768 {
            return Err(JsValue::from_str("visual save exceeds bound"));
        }
        let save: AnimationSave =
            serde_json::from_str(json).map_err(|_| JsValue::from_str("invalid visual save"))?;
        if save.world_fingerprint != qualia_core_db::container_10d::crc32c::crc32c(world.as_bytes())
        {
            return Err(JsValue::from_str(
                "visual save belongs to another campaign snapshot",
            ));
        }
        self.inner.restore_animation_snapshot_json(&save.timeline)
    }
}
