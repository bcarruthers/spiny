use glam::*;
use indexmap::{IndexSet, IndexMap, map::Entry};
use std::hash::Hash;

use crate::{key::{KeyCode, KeyModifier}, mouse::MouseButton, ElementState};

#[derive(Clone, Debug)]
pub struct PressState<T> {
    just_down: IndexSet<T>,
    just_up: IndexSet<T>,
    down: IndexSet<T>,
}

impl<T> Default for PressState<T> {
    fn default() -> Self {
        Self {
            just_down: Default::default(),
            just_up: Default::default(),
            down: Default::default(),
        }
    }
}

impl<T: Hash + Eq + Copy> PressState<T> {
    pub fn iter_down(&self) -> impl Iterator<Item = &T> {
        self.down.iter()
    }

    pub fn iter_just_down(&self) -> impl Iterator<Item = &T> {
        self.just_down.iter()
    }

    pub fn up(&self, button: T) -> bool {
        !self.down.contains(&button)
    }

    pub fn down(&self, button: T) -> bool {
        self.down.contains(&button)
    }

    pub fn just_down(&self, button: T) -> bool {
        self.just_down.contains(&button)
    }

    pub fn just_up(&self, button: T) -> bool {
        self.just_up.contains(&button)
    }

    pub fn any_up(&self, buttons: &[T]) -> bool {
        buttons.iter().any(|button| self.up(*button))
    }

    pub fn any_down(&self, buttons: &[T]) -> bool {
        buttons.iter().any(|button| self.down(*button))
    }

    pub fn any_just_down(&self, buttons: &[T]) -> bool {
        buttons.iter().any(|button| self.just_down(*button))
    }

    pub fn any_just_up(&self, buttons: &[T]) -> bool {
        buttons.iter().any(|button| self.just_up(*button))
    }

    pub fn clear_events(&mut self) {
        self.just_down.clear();
        self.just_up.clear();
    }

    pub fn clear(&mut self) {
        self.clear_events();
        self.down.clear();
    }

    pub fn apply_down(&mut self, button: T) {
        self.down.insert(button);
        self.just_down.insert(button);
    }

    pub fn apply_up(&mut self, button: T) {
        self.down.remove(&button);
        self.just_up.insert(button);
    }

    pub fn apply(&mut self, button: T, state: ElementState) {
        let is_pressed = state == ElementState::Pressed;
        if is_pressed {
            self.apply_down(button);
        } else {
            self.apply_up(button);
        }
    }
}

#[derive(Default, Clone)]
pub struct MouseState {
    pub buttons: PressState<MouseButton>,
    pub cursor_pos: Vec2,
    pub cursor_delta: Vec2,
    pub delta: Vec2,
    pub norm_cursor_pos: Vec2,
    pub norm_cursor_delta: Vec2,
    pub norm_delta: Vec2,
    pub wheel_delta: Vec2,
}

impl MouseState {
    fn norm_pos(pos: Vec2, size: UVec2) -> Vec2 {
        let v = pos / size.as_ivec2().as_vec2() * 2.0 - Vec2::ONE;
        Vec2::new(v.x, -v.y)
    }

    pub fn clear_events(&mut self) {
        self.buttons.clear_events();
        self.cursor_delta = Vec2::ZERO;
        self.norm_cursor_delta = Vec2::ZERO;
        self.delta = Vec2::ZERO;
        self.norm_delta = Vec2::ZERO;
        self.wheel_delta = Vec2::ZERO;
    }

    pub fn update_pos(&mut self, new_pos: Vec2, size: UVec2) {
        self.cursor_delta = new_pos - self.cursor_pos;
        self.norm_cursor_delta = Self::norm_pos(self.cursor_delta, size);
        self.cursor_pos = new_pos;
        self.norm_cursor_pos = Self::norm_pos(new_pos, size);
        //log::info!("Update cursor: {:?}", self.cursor_delta);
    }

    pub fn update_delta(&mut self, delta: Vec2, size: UVec2) {
        self.delta = delta;
        self.norm_delta = Self::norm_pos(delta, size);
    }
}

#[derive(Clone)]
pub struct TouchPress {
    pub pos: Vec2,
    pub delta: Vec2,
    pub norm_pos: Vec2,
    pub norm_delta: Vec2,
}

#[derive(Default, Clone)]
pub struct TouchState {
    presses: IndexMap<u64, TouchPress>
}

impl TouchState {
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &TouchPress)> {
        self.presses.iter()
    }
    
    pub fn update(&mut self, event: &TouchEvent, size: UVec2) {
        //log::info!("Touch: {:?}", event);
        match self.presses.entry(event.id) {
            Entry::Occupied(entry) =>
                match event.phase {
                    TouchPhase::Started | 
                    TouchPhase::Moved => {
                        let norm_pos = event.pos / size.as_vec2();
                        let press = entry.into_mut();
                        press.delta = event.pos - press.pos;
                        press.norm_delta = norm_pos - press.norm_pos;
                        press.pos = event.pos;
                        press.norm_pos = norm_pos;
                    },
                    TouchPhase::Cancelled |
                    TouchPhase::Ended => {
                        entry.remove();
                    }
                },
            Entry::Vacant(entry) => 
                match event.phase {
                    TouchPhase::Started | 
                    TouchPhase::Moved => {
                        entry.insert(TouchPress {
                            pos: event.pos,
                            delta: Vec2::ZERO,
                            norm_pos: event.pos / size.as_vec2(),
                            norm_delta: Vec2::ZERO,
                        });
                    },
                    TouchPhase::Cancelled |
                    TouchPhase::Ended => ()
                }
        }
    }
}

bitflags::bitflags! {
    /// Represents the current state of the keyboard modifiers
    ///
    /// Each flag represents a modifier and is set if this modifier is active.
    #[derive(Default)]
    pub struct ModifiersState: u32 {
        // left and right modifiers are currently commented out, but we should be able to support
        // them in a future release
        /// The "shift" key.
        const SHIFT = 0b100;
        // const LSHIFT = 0b010;
        // const RSHIFT = 0b001;
        /// The "control" key.
        const CTRL = 0b100 << 3;
        // const LCTRL = 0b010 << 3;
        // const RCTRL = 0b001 << 3;
        /// The "alt" key.
        const ALT = 0b100 << 6;
        // const LALT = 0b010 << 6;
        // const RALT = 0b001 << 6;
        /// This is the "windows" key on PC and "command" key on Mac.
        const LOGO = 0b100 << 9;
        // const LLOGO = 0b010 << 9;
        // const RLOGO = 0b001 << 9;
    }
}

impl ModifiersState {
    pub fn from_key_modifier(modifier: KeyModifier) -> Self {
        match modifier {
            KeyModifier::Shift => Self::SHIFT,
            KeyModifier::Ctrl => Self::CTRL,
            KeyModifier::Alt => Self::ALT,
            KeyModifier::Logo => Self::LOGO,
        }
    }

    pub fn from_key_code(code: KeyCode) -> Self {
        if let Some(modifier) = code.modifier() {
            Self::from_key_modifier(modifier)
        } else {
            Self::empty()
        }
    }

    pub fn from_key_modifiers(modifiers: &[KeyModifier]) -> Self {
        let mut state = Self::empty();
        for modifier in modifiers {
            state |= Self::from_key_modifier(*modifier);
        }
        state
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

pub struct MouseEvent {
    pub button: MouseButton,
    pub state: ElementState,
}

#[derive(Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub phase: TouchPhase,
    pub id: u64,
    pub pos: Vec2,
}

pub enum WindowEvent {
    Closing,
    Resized(UVec2),
    ScaleFactorChanged(f32),
    KeyboardInput(KeyboardEvent),
    ModifiersChanged(ModifiersState),
    CursorMoved(IVec2),
    MouseMoved(Vec2),
    MouseWheel(Vec2),
    MouseInput(MouseEvent),
    Touch(TouchEvent),
    PasteFromClipboard(String),
    FullScreenChanged(bool),
}

impl WindowEvent {
    pub fn is_interacting(&self) -> bool {
        match self {
            WindowEvent::KeyboardInput(_) |
            WindowEvent::MouseWheel(_) |
            WindowEvent::MouseInput(_) |
            WindowEvent::Touch(_) => true,
            _ => false,
        }
    }
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
            let mods = ModifiersState::from_key_code(event.key);
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

#[derive(Clone)]
pub struct WindowState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
    pub touch: TouchState,
    pub size: UVec2,
    pub scale: f32,
    pub fullscreen: bool,
    pub closing: bool,
    pub clipboard_data: Option<String>,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            keyboard: Default::default(),
            mouse: Default::default(),
            touch: Default::default(),
            size: UVec2::ZERO,
            scale: 1.0,
            fullscreen: false,
            closing: false,
            clipboard_data: None,
        }
    }

    pub fn norm_cursor_pos(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let x = self.mouse.cursor_pos.x as f32 * scale;
        let y = self.mouse.cursor_pos.y as f32 * scale;
        Vec2::new(x, y)
    }

    pub fn norm_cursor_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.mouse.cursor_delta.x as f32 * scale;
        let dy = -self.mouse.cursor_delta.y as f32 * scale;
        Vec2::new(dx, dy)
    }

    pub fn norm_mouse_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.mouse.delta.x as f32 * scale;
        let dy = -self.mouse.delta.y as f32 * scale;
        Vec2::new(dx, dy)
    }

    pub fn take_clipboard_data(&mut self) -> Option<String> {
        self.clipboard_data.take()
    }

    pub fn clear_events(&mut self) {
        self.keyboard.clear_events();
        self.mouse.clear_events();
        self.closing = false;
        self.clipboard_data = None;
    }

    pub fn apply(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Closing => self.closing = true,
            WindowEvent::Resized(size) => {
                log::debug!("Window size changed: {}", size);
                self.size = size
            }
            WindowEvent::ScaleFactorChanged(scale) => {
                log::debug!("Scale factor changed: {}", scale);
                self.scale = scale
            }
            WindowEvent::KeyboardInput(event) => {
                //log::info!("Key {:?}: {:?}", event.state, event.key);
                self.keyboard.push_event(event)
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                //log::info!("Modifiers changed: {:?}", modifiers);
                self.keyboard.push_modifiers(modifiers)
            }
            WindowEvent::CursorMoved(pos) => self.mouse.update_pos(pos.as_vec2(), self.size),
            WindowEvent::MouseMoved(delta) => self.mouse.update_delta(delta, self.size),
            WindowEvent::MouseWheel(delta) => {
                log::debug!("Mouse wheel: {:?}", delta);
                self.mouse.wheel_delta = delta
            },
            WindowEvent::MouseInput(event) => {
                self.mouse.buttons.apply(event.button, event.state.clone())
            }
            WindowEvent::Touch(event) => self.touch.update(&event, self.size),
            WindowEvent::PasteFromClipboard(s) => self.clipboard_data = Some(s),
            WindowEvent::FullScreenChanged(fullscreen) => self.fullscreen = fullscreen,
        }
    }

    pub fn flush(&mut self) {
        self.keyboard.flush();
    }
}
