//! WASM Input Bridge (`input_bridge.rs`)
//!
//! Exposes QualiaDB's zero-heap input engine (`qualia-sim::input`) to the browser canvas host:
//! - High-frequency keyboard event capture with single-frame edge transitions
//! - Mobile multi-touch virtual joysticks for steering and throttle
//! - Direct generation of canonical driving control facts for `drive_tick`
//!
//! Strictly zero heap in hot loop evaluation.

use qualia_sim::input::{
    VirtualTouchStick, ZeroHeapButtonInput, INPUT_WORDS_4, KEY_A, KEY_ARROW_DOWN, KEY_ARROW_LEFT,
    KEY_ARROW_RIGHT, KEY_ARROW_UP, KEY_D, KEY_E, KEY_ESCAPE, KEY_Q, KEY_S, KEY_SHIFT, KEY_SPACE,
    KEY_TAB, KEY_W,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameInputBridge {
    buttons: ZeroHeapButtonInput<INPUT_WORDS_4>,
    touch_stick: VirtualTouchStick,
    touch_active: bool,
}

#[wasm_bindgen]
impl GameInputBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            buttons: ZeroHeapButtonInput::new(),
            touch_stick: VirtualTouchStick::new(60.0, 12.0),
            touch_active: false,
        }
    }

    /// Records a key press event from the browser `keydown` listener.
    pub fn key_down(&mut self, key_code: u16) {
        self.buttons.press(key_code);
    }

    /// Records a key release event from the browser `keyup` listener.
    pub fn key_up(&mut self, key_code: u16) {
        self.buttons.release(key_code);
    }

    /// Checks if a key is currently held down.
    pub fn is_pressed(&self, key_code: u16) -> bool {
        self.buttons.is_pressed(key_code)
    }

    /// Checks if a key was just pressed on the current frame.
    pub fn is_just_pressed(&self, key_code: u16) -> bool {
        self.buttons.is_just_pressed(key_code)
    }

    /// Clears single-frame edge transitions (call once per render frame).
    pub fn step_frame(&mut self) {
        self.buttons.clear_edges();
    }

    /// Touch gesture started at canvas screen coordinates (x, y).
    pub fn touch_start(&mut self, x: f32, y: f32) {
        self.touch_stick.anchor_at(x, y);
        self.touch_active = true;
    }

    /// Touch pointer moved to canvas screen coordinates (x, y).
    pub fn touch_move(&mut self, x: f32, y: f32) {
        if self.touch_active {
            self.touch_stick.update_pointer(x, y);
        }
    }

    /// Touch gesture ended / cancelled.
    pub fn touch_end(&mut self) {
        self.touch_stick.release();
        self.touch_active = false;
    }

    /// Evaluates current throttle output in range `[-1.0, 1.0]` (positive = forward, negative = reverse).
    pub fn throttle(&self) -> f32 {
        let mut keyboard = 0.0f32;
        if self.buttons.is_pressed(KEY_W) || self.buttons.is_pressed(KEY_ARROW_UP) {
            keyboard += 1.0;
        }
        if self.buttons.is_pressed(KEY_S) || self.buttons.is_pressed(KEY_ARROW_DOWN) {
            keyboard -= 1.0;
        }

        if self.touch_active {
            let (_tx, ty) = self.touch_stick.axis_vector();
            // Upwards on screen is negative Y in standard canvas space
            let touch_val = -ty;
            if touch_val.abs() > keyboard.abs() {
                return touch_val;
            }
        }

        keyboard
    }

    /// Evaluates current steering output in range `[-1.0, 1.0]` (positive = right, negative = left).
    pub fn steering(&self) -> f32 {
        let mut keyboard = 0.0f32;
        if self.buttons.is_pressed(KEY_D) || self.buttons.is_pressed(KEY_ARROW_RIGHT) {
            keyboard += 1.0;
        }
        if self.buttons.is_pressed(KEY_A) || self.buttons.is_pressed(KEY_ARROW_LEFT) {
            keyboard -= 1.0;
        }

        if self.touch_active {
            let (tx, _ty) = self.touch_stick.axis_vector();
            if tx.abs() > keyboard.abs() {
                return tx;
            }
        }

        keyboard
    }

    /// Checks if handbrake / space is pressed.
    pub fn brake(&self) -> bool {
        self.buttons.is_pressed(KEY_SPACE)
    }

    /// Checks if gear shift / boost key is pressed.
    pub fn shift(&self) -> bool {
        self.buttons.is_pressed(KEY_SHIFT)
    }

    /// Checks interaction key (E).
    pub fn interact(&self) -> bool {
        self.buttons.is_just_pressed(KEY_E)
    }

    /// Checks horn / quick alert key (Q).
    pub fn horn(&self) -> bool {
        self.buttons.is_pressed(KEY_Q)
    }

    /// Checks map / menu toggle key (Tab).
    pub fn toggle_map(&self) -> bool {
        self.buttons.is_just_pressed(KEY_TAB)
    }

    /// Checks pause / escape key.
    pub fn pause(&self) -> bool {
        self.buttons.is_just_pressed(KEY_ESCAPE)
    }
}

impl Default for GameInputBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_input_mapping() {
        let mut bridge = GameInputBridge::new();
        assert_eq!(bridge.throttle(), 0.0);
        assert_eq!(bridge.steering(), 0.0);
        assert!(!bridge.brake());

        // Press W
        bridge.key_down(KEY_W);
        assert_eq!(bridge.throttle(), 1.0);
        assert!(bridge.is_pressed(KEY_W));
        assert!(bridge.is_just_pressed(KEY_W));

        // Advance frame: just_pressed resets, pressed stays true
        bridge.step_frame();
        assert!(bridge.is_pressed(KEY_W));
        assert!(!bridge.is_just_pressed(KEY_W));

        // Press D
        bridge.key_down(KEY_D);
        assert_eq!(bridge.steering(), 1.0);

        // Press Space
        bridge.key_down(KEY_SPACE);
        assert!(bridge.brake());

        // Release W
        bridge.key_up(KEY_W);
        assert_eq!(bridge.throttle(), 0.0);
    }

    #[test]
    fn test_touch_stick_mapping() {
        let mut bridge = GameInputBridge::new();
        bridge.touch_start(100.0, 100.0);
        // Drag up and right: (100, 100) -> (130, 70)
        bridge.touch_move(130.0, 70.0);

        assert!(bridge.steering() > 0.0);
        assert!(bridge.throttle() > 0.0);

        bridge.touch_end();
        assert_eq!(bridge.steering(), 0.0);
        assert_eq!(bridge.throttle(), 0.0);
    }
}
