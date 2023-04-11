use crate::key::KeyCode;

use super::state::*;
use indexmap::{IndexMap, map::Entry};
use std::hash::Hash;

fn get_key_modifiers(key: KeyCode) -> ModifiersState {
    match key {
        KeyCode::LShift | KeyCode::RShift => ModifiersState::SHIFT,
        KeyCode::LControl | KeyCode::RControl => ModifiersState::CTRL,
        KeyCode::LAlt | KeyCode::RAlt => ModifiersState::ALT,
        KeyCode::LWin | KeyCode::RWin => ModifiersState::LOGO,
        _ => ModifiersState::empty(),
    }
}

pub struct KeyCmdBinding<Cmd> {
    pub command: Cmd,
    pub key: KeyPress,
}

pub struct CommandKeyMap<Cmd> {
    map: IndexMap<Cmd, Vec<KeyPress>>,
}

impl<Cmd> Default for CommandKeyMap<Cmd> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<Cmd: Eq + Hash> CommandKeyMap<Cmd> {
    pub fn get_keys(&self, cmd: Cmd) -> &[KeyPress] {
        match self.map.get(&cmd) {
            Some(keys) => &keys,
            None => &[],
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn add(&mut self, binding: KeyCmdBinding<Cmd>) {
        // For keys which are modifiers, automatically add modifiers to mapping
        // since they will always occur with key
        let press = KeyPress {
            mods: binding.key.mods | get_key_modifiers(binding.key.code),
            code: binding.key.code,
        };
        match self.map.entry(binding.command) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(press);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![press]);
            }
        }
    }
}

impl<Cmd: Eq + Hash + Copy> CommandKeyMap<Cmd> {
    pub fn key_down_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses().any_down(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_just_down_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses().any_just_down(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_up_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses().any_up(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_just_up_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses().any_just_up(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_down_cmds<'a>(
        &'a self,
        input: &'a KeyboardState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter().filter_map(|cmd| self.key_down_cmd(input, *cmd))
    }

    pub fn key_just_down_cmds<'a>(
        &'a self,
        input: &'a KeyboardState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter()
            .filter_map(|cmd| self.key_just_down_cmd(input, *cmd))
    }

    pub fn key_up_cmds<'a>(
        &'a self,
        input: &'a KeyboardState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter().filter_map(|cmd| self.key_up_cmd(input, *cmd))
    }

    pub fn key_just_up_cmds<'a>(
        &'a self,
        input: &'a KeyboardState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter()
            .filter_map(|cmd| self.key_just_up_cmd(input, *cmd))
    }
}
