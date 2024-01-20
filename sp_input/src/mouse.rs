use glam::{Vec2, UVec2, IVec2};
use serde_derive::*;

use crate::press::{ElementState, PressState};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub state: ElementState,
}

pub enum MouseEvent {
    CursorMoved(IVec2),
    MouseMoved(Vec2),
    MouseWheel(Vec2),
    MouseInput(MouseButtonEvent),
}

#[derive(Default, Clone)]
pub struct MouseState {
    buttons: PressState<MouseButton>,
    cursor_pos: Vec2,
    cursor_delta: Vec2,
    delta: Vec2,
    norm_cursor_pos: Vec2,
    norm_cursor_delta: Vec2,
    norm_delta: Vec2,
    wheel_delta: Vec2,
}

impl MouseState {
    pub fn buttons(&self) -> &PressState<MouseButton> {
        &self.buttons
    }
    
    pub fn cursor_pos(&self) -> Vec2 {
        self.cursor_pos
    }

    pub fn cursor_delta(&self) -> Vec2 {
        self.cursor_delta
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    fn norm_pos(pos: Vec2, size: UVec2) -> Vec2 {
        let v = pos / size.as_ivec2().as_vec2() * 2.0 - Vec2::ONE;
        Vec2::new(v.x, -v.y)
    }

    fn update_pos(&mut self, new_pos: Vec2, size: UVec2) {
        self.cursor_delta = new_pos - self.cursor_pos;
        self.norm_cursor_delta = Self::norm_pos(self.cursor_delta, size);
        self.cursor_pos = new_pos;
        self.norm_cursor_pos = Self::norm_pos(new_pos, size);
        //log::info!("Update cursor: {:?}", self.cursor_delta);
    }

    fn update_delta(&mut self, delta: Vec2, size: UVec2) {
        self.delta = delta;
        self.norm_delta = Self::norm_pos(delta, size);
    }

    pub fn clear_events(&mut self) {
        self.buttons.clear_events();
        self.cursor_delta = Vec2::ZERO;
        self.norm_cursor_delta = Vec2::ZERO;
        self.delta = Vec2::ZERO;
        self.norm_delta = Vec2::ZERO;
        self.wheel_delta = Vec2::ZERO;
    }

    pub fn update(&mut self, event: MouseEvent, size: UVec2) {
        match event {
            MouseEvent::CursorMoved(pos) => self.update_pos(pos.as_vec2(), size),
            MouseEvent::MouseMoved(delta) => self.update_delta(delta, size),
            MouseEvent::MouseWheel(delta) => {
                log::debug!("Mouse wheel: {:?}", delta);
                self.wheel_delta = delta
            },
            MouseEvent::MouseInput(event) => {
                self.buttons.apply(event.button, event.state.clone())
            }
        }
    }
}
