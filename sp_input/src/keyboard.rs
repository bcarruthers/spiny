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
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>0</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Quote,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
    Unknown,
}

impl KeyCode {
    fn try_to_str(&self) -> Option<&str> {
        match self {
            Self::Digit1 => Some("1"),
            Self::Digit2 => Some("2"),
            Self::Digit3 => Some("3"),
            Self::Digit4 => Some("4"),
            Self::Digit5 => Some("5"),
            Self::Digit6 => Some("6"),
            Self::Digit7 => Some("7"),
            Self::Digit8 => Some("8"),
            Self::Digit9 => Some("9"),
            Self::Digit0 => Some("0"),
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
            Self::AltLeft | Self::AltRight => Some(KeyModifier::Alt),
            Self::ControlLeft | Self::ControlRight => Some(KeyModifier::Ctrl),
            Self::ShiftLeft | Self::ShiftRight => Some(KeyModifier::Shift),
            Self::SuperLeft | Self::SuperRight => Some(KeyModifier::Logo),
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
