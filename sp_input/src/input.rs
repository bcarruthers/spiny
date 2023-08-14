use glam::UVec2;

use crate::{KeyboardState, MouseState, TouchState, GamepadState, InputEvent};

#[derive(Clone, Default)]
pub struct InputState {
    keyboard: KeyboardState,
    mouse: MouseState,
    touch: TouchState,
    gamepad: GamepadState,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keyboard: Default::default(),
            mouse: Default::default(),
            touch: Default::default(),
            gamepad: Default::default(),
        }
    }

    pub fn keyboard(&self) -> &KeyboardState {
        &self.keyboard
    }

    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }

    pub fn touch(&self) -> &TouchState {
        &self.touch
    }

    pub fn gamepad(&self) -> &GamepadState {
        &self.gamepad
    }

    pub fn clear_events(&mut self) {
        self.keyboard.clear_events();
        self.mouse.clear_events();
        self.gamepad.clear_events();
    }

    pub fn flush(&mut self) {
        self.keyboard.flush();
    }

    pub fn apply(&mut self, event: InputEvent, size: UVec2) {
        match event {
            InputEvent::Keyboard(event) => {
                //log::info!("Key {:?}: {:?}", event.state, event.key);
                self.keyboard.push_event(event)
            }
            InputEvent::ModifiersChanged(modifiers) => {
                //log::info!("Modifiers changed: {:?}", modifiers);
                self.keyboard.push_modifiers(modifiers)
            }
            InputEvent::Mouse(event) => self.mouse.update(event, size),
            InputEvent::Touch(event) => self.touch.update(&event, size),
            InputEvent::Gamepad(event) => self.gamepad.update(&event),
        }
    }
}