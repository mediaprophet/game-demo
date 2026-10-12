//! Source-metre appearance controls over Qualia's native water renderer.
use crate::GamePortal;
use wasm_bindgen::prelude::*;
fn scaled_wave(amplitude:f32,frequency:f32,scale:f32)->Result<(f32,f32),JsValue> {
    if !amplitude.is_finite()||!frequency.is_finite()||!scale.is_finite()||
        amplitude<0.0||frequency<0.0||scale<=0.0 {
        return Err(JsValue::from_str("water appearance units must be finite and positive"));
    }
    let amplitude=amplitude*scale;let frequency=frequency/scale;
    if !amplitude.is_finite()||!frequency.is_finite() {return Err(JsValue::from_str("water appearance conversion overflow"));}
    Ok((amplitude,frequency))
}
#[wasm_bindgen]
impl GamePortal {
    pub fn set_water_surface_metres(&mut self,r:f32,g:f32,b:f32,alpha:f32,amplitude_metres:f32,
        radians_per_metre:f32,world_scale:f32,quality:u32)->Result<(),JsValue> {
        let (amplitude,frequency)=scaled_wave(amplitude_metres,radians_per_metre,world_scale)?;
        self.set_water_surface(r,g,b,alpha,amplitude,frequency,quality);Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    #[wasm_bindgen_test]
    fn world_scale_changes_units_without_changing_physical_wave_slope() {
        for scale in [0.01,0.05,1.0,10.0] {
            let (amplitude,frequency)=scaled_wave(0.025,1.8,scale).unwrap();
            assert!((amplitude*frequency-0.045).abs()<1e-6);
            assert!((amplitude/scale-0.025).abs()<1e-6);
        }
        assert!(scaled_wave(0.025,1.8,0.0).is_err());
    }
}
