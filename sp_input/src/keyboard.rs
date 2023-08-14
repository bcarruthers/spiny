use std::hash::Hash;
use indexmap::{IndexMap, map::Entry};
use serde_derive::*;

use crate::press::{ModifiersState, ElementState, PressState};

#[derive(
    Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,
)]
pub enum KeyModifier {
    Shift,
    Ctrl,
    Alt,
    Logo,
}

impl KeyModifier {
    pub fn to_modifiers(&self) -> ModifiersState {
        match self {
            Self::Shift => ModifiersState::SHIFT,
            Self::Ctrl => ModifiersState::CTRL,
            Self::Alt => ModifiersState::ALT,
            Self::Logo => ModifiersState::LOGO,
        }
    }

    pub fn to_modifiers_from_slice(modifiers: &[Self]) -> ModifiersState {
        let mut state = ModifiersState::empty();
        for modifier in modifiers {
            state |= modifier.to_modifiers();
        }
        state
    }
}

// From winit
#[derive(
    Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,
)]
#[repr(u32)]
pub enum KeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    /// The left alt key. Maps to left option on Mac.
    LAlt,
    LBracket,
    LControl,
    LShift,
    /// The left Windows key. Maps to left Command on Mac.
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Oem102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    /// The right alt key. Maps to right option on Mac.
    RAlt,
    RBracket,
    RControl,
    RShift,
    /// The right Windows key. Maps to right Command on Mac.
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl KeyCode {
    fn try_to_str(&self) -> Option<&str> {
        match self {
            Self::Key1 => Some("1"),
            Self::Key2 => Some("2"),
            Self::Key3 => Some("3"),
            Self::Key4 => Some("4"),
            Self::Key5 => Some("5"),
            Self::Key6 => Some("6"),
            Self::Key7 => Some("7"),
            Self::Key8 => Some("8"),
            Self::Key9 => Some("9"),
            Self::Key0 => Some("0"),
            Self::Escape => Some("ESC"),
            _ => None
        }
    }

    pub fn format(&self) -> String {
        match self.try_to_str() {
            Some(s) => s.to_string(),
            None => {
                let mut s = format!("{:?}", self).to_uppercase();
                s.truncate(3);
                s
            }
        }
    }

    pub fn modifier(&self) -> Option<KeyModifier> {
        match self {
            Self::LAlt | Self::RAlt => Some(KeyModifier::Alt),
            Self::LControl | Self::RControl => Some(KeyModifier::Ctrl),
            Self::LShift | Self::RShift => Some(KeyModifier::Shift),
            Self::LWin | Self::RWin => Some(KeyModifier::Logo),
            _ => None
        }
    }

    pub fn to_modifiers(&self) -> ModifiersState {
        if let Some(modifier) = self.modifier() {
            modifier.to_modifiers()
        } else {
            ModifiersState::empty()
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyPress {
    pub mods: ModifiersState,
    pub code: KeyCode,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
    pub key: KeyCode,
    pub state: ElementState,
}

#[derive(Default, Clone)]
pub struct KeyboardState {
    keys: PressState<KeyCode>,
    presses: PressState<KeyPress>,
    modifiers: ModifiersState,
    pending_modifiers: ModifiersState,
    pending_events: Vec<KeyboardEvent>,
}

impl KeyboardState {
    pub fn keys(&self) -> &PressState<KeyCode> {
        &self.keys
    }

    pub fn presses(&self) -> &PressState<KeyPress> {
        &self.presses
    }

    pub fn any_down(&self, buttons: &[KeyPress]) -> bool {
        buttons.iter().any(|button| self.keys.down(button.code))
    }
    
    pub fn any_just_down(&self, buttons: &[KeyPress]) -> bool {
        buttons.iter().any(|button| self.keys.just_down(button.code))
    }
    
    pub fn clear_events(&mut self) {
        // Note we clear all presses since they are per-frame events and may
        // include modifiers (which might not have a matching key release event)
        self.keys.clear_events();
        self.presses.clear();
    }

    pub fn push_modifiers(&mut self, modifiers: ModifiersState) {
        self.pending_modifiers = modifiers;
    }

    pub fn push_event(&mut self, key: KeyboardEvent) {
        self.pending_events.push(key);
    }

    pub fn flush(&mut self) {
        // Deferring events so we can ensure modifiers are always applied
        // consistently regardless of the order key and modifie events are
        // received from winit (macos differs from windows)
        self.modifiers = std::mem::take(&mut self.pending_modifiers);
        // For wasm, we don't get modifier events, so we need to apply them
        // from events
        for event in &self.pending_events {
            let mods = event.key.to_modifiers();
            match event.state {
                ElementState::Pressed => self.modifiers |= mods,
                ElementState::Released => self.modifiers &= !mods,
            }
        }
        // Now that modifiers are updated, apply events
        for event in self.pending_events.drain(..) {
            let press = KeyPress {
                mods: self.modifiers,
                code: event.key,
            };
            //log::info!("{:?}", press);
            self.keys.apply(event.key, event.state);
            self.presses.apply(press, event.state);    
        }
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
        let mods = binding.key.code.to_modifiers();
        let press = KeyPress {
            mods: binding.key.mods | mods,
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
