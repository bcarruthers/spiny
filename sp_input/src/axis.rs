use crate::{keyboard::{KeyPress, KeyboardState}, GamepadState, GamepadButton, CommandGamepadButtonMap, InputState};

use super::keyboard::CommandKeyMap;
use glam::*;
use std::hash::Hash;

#[derive(Default)]
pub struct InputAxis {
    dir: f32,
}

impl InputAxis {
    fn update_from_keyboard(&mut self, input: &KeyboardState, pos_keys: &[KeyPress], neg_keys: &[KeyPress]) {
        // Note we ignore any modifiers, otherwise axes can get stuck or not activate
        // properly with modifier down
        let neg = input.any_down(neg_keys);
        let pos = input.any_down(pos_keys);
        self.dir = if neg || pos {
            if input.any_just_down(&neg_keys) {
                -1.0
            } else if input.any_just_down(&pos_keys) {
                1.0
            } else if neg != pos {
                if neg {
                    -1.0
                } else {
                    1.0
                }
            } else {
                self.dir
            }
        } else {
            0.0
        };
    }

    pub fn update_from_keyboard_map<Cmd: Eq + Hash + Copy>(
        &mut self,
        input: &KeyboardState,
        map: &CommandKeyMap<Cmd>,
        pos_cmd: Cmd,
        neg_cmd: Cmd,
    ) -> Option<Cmd> {
        let pos_keys = map.get_keys(pos_cmd);
        let neg_keys = map.get_keys(neg_cmd);
        self.update_from_keyboard(input, pos_keys, neg_keys);
        if self.dir > 0.0 {
            Some(pos_cmd)
        } else if self.dir < 0.0 {
            Some(neg_cmd)
        } else {
            None
        }
    }

    fn update_from_gamepad(&mut self, input: &GamepadState, pos_keys: &[GamepadButton], neg_keys: &[GamepadButton]) {
        // Note we ignore any modifiers, otherwise axes can get stuck or not activate
        // properly with modifier down
        let neg = input.any_pressed(neg_keys);
        let pos = input.any_pressed(pos_keys);
        self.dir = if neg || pos {
            if input.any_just_pressed(&neg_keys) {
                -1.0
            } else if input.any_just_pressed(&pos_keys) {
                1.0
            } else if neg != pos {
                if neg {
                    -1.0
                } else {
                    1.0
                }
            } else {
                self.dir
            }
        } else {
            0.0
        };
    }

    pub fn update_from_gamepad_map<Cmd: Eq + Hash + Copy>(
        &mut self,
        input: &GamepadState,
        map: &CommandGamepadButtonMap<Cmd>,
        pos_cmd: Cmd,
        neg_cmd: Cmd,
    ) -> Option<Cmd> {
        let pos_buttons = map.get_buttons(pos_cmd);
        let neg_buttons = map.get_buttons(neg_cmd);
        self.update_from_gamepad(input, pos_buttons, neg_buttons);
        if self.dir > 0.0 {
            Some(pos_cmd)
        } else if self.dir < 0.0 {
            Some(neg_cmd)
        } else {
            None
        }
    }

    pub fn update_from_maps<Cmd: Eq + Hash + Copy>(
        &mut self,
        state: &InputState,
        keyboard_map: &CommandKeyMap<Cmd>,
        gamepad_map: &CommandGamepadButtonMap<Cmd>,
        pos_cmd: Cmd,
        neg_cmd: Cmd,
    ) -> Option<Cmd> {
        let mut cmd = None;
        if let Some(c) = self.update_from_keyboard_map(state.keyboard(), keyboard_map, pos_cmd, neg_cmd) {
            cmd = Some(c);
        }
        if let Some(c) = self.update_from_gamepad_map(state.gamepad(), gamepad_map, pos_cmd, neg_cmd) {
            cmd = Some(c);
        }
        cmd
    }
}
