use crate::{press::PressState, keyboard::{KeyCode, KeyPress, KeyboardState}};

use super::{cmd::CommandKeyMap};
use glam::*;
use std::hash::Hash;

fn any_down(state: &PressState<KeyCode>, buttons: &[KeyPress]) -> bool {
    buttons.iter().any(|button| state.down(button.code))
}

fn any_just_down(state: &PressState<KeyCode>, buttons: &[KeyPress]) -> bool {
    buttons.iter().any(|button| state.just_down(button.code))
}

#[derive(Default)]
pub struct InputAxis {
    dir: f32,
}

impl InputAxis {
    pub fn update(&mut self, input: &KeyboardState, pos_keys: &[KeyPress], neg_keys: &[KeyPress]) {
        // Note we ignore any modifiers, otherwise axes can get stuck or not activate
        // properly with modifier down
        let neg = any_down(input.keys(), neg_keys);
        let pos = any_down(input.keys(), pos_keys);
        self.dir = if neg || pos {
            if any_just_down(input.keys(), &neg_keys) {
                -1.0
            } else if any_just_down(input.keys(), &pos_keys) {
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

    pub fn update_mapped<Cmd: Eq + Hash + Copy>(
        &mut self,
        input: &KeyboardState,
        map: &CommandKeyMap<Cmd>,
        pos_cmd: Cmd,
        neg_cmd: Cmd,
    ) -> Option<Cmd> {
        let pos_keys = map.get_keys(pos_cmd);
        let neg_keys = map.get_keys(neg_cmd);
        self.update(input, pos_keys, neg_keys);
        if self.dir > 0.0 {
            Some(pos_cmd)
        } else if self.dir < 0.0 {
            Some(neg_cmd)
        } else {
            None
        }
    }
}
