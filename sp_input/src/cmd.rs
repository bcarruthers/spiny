use super::state::*;
use indexmap::{IndexMap, map::Entry};
use std::hash::Hash;

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
        match self.map.entry(binding.command) {
            Entry::Occupied(entry) => entry.into_mut().push(binding.key),
            Entry::Vacant(entry) => {
                entry.insert(vec![binding.key]);
            }
        }
    }
}

impl<Cmd: Eq + Hash + Copy> CommandKeyMap<Cmd> {
    pub fn key_down_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses.any_down(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_just_down_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses.any_just_down(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_up_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses.any_up(self.get_keys(cmd)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn key_just_up_cmd(&self, input: &KeyboardState, cmd: Cmd) -> Option<Cmd> {
        if input.presses.any_just_up(self.get_keys(cmd)) {
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
