//! Physical-unit lens adapter. Projection and admitted bounds remain native.
use wasm_bindgen::prelude::*;
use super::{GameHud, GamePortal};

fn near_plane(metres: f32, scale: f32) -> Result<f32, JsValue> {
    if !metres.is_finite() || !scale.is_finite() || metres <= 0.0 || scale <= 0.0 {
        return Err(JsValue::from_str("camera physical near plane invalid"));
    }
    let near = metres * scale;
    if !near.is_finite() {
        return Err(JsValue::from_str("camera near plane overflow"));
    }
    Ok(near.clamp(0.00001, 1.0))
}

#[wasm_bindgen]
impl GamePortal {
    pub fn set_camera_near_metres(&mut self, metres: f32, scale: f32) -> Result<(), JsValue> {
        self.inner.set_camera_near_plane(near_plane(metres, scale)?)
    }
}

#[wasm_bindgen]
impl GameHud {
    pub fn set_camera_near_metres(&mut self, metres: f32, scale: f32) -> Result<(), JsValue> {
        self.inner.set_camera_near_plane(near_plane(metres, scale)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_lens_uses_world_scale_and_rejects_invalid_units() {
        assert!((near_plane(0.035, 0.04).unwrap() - 0.0014).abs() < 1e-8);
        assert!(near_plane(0.035, 0.0).is_err());
        assert!(near_plane(f32::NAN, 1.0).is_err());
    }
}
