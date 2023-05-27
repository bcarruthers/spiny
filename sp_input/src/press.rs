use glam::*;
use indexmap::IndexSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementState {
    Pressed,
    Released,
}

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

// From winit
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
