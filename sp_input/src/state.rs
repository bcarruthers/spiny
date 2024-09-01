use glam::*;

use crate::{GamepadEvent, InputState, KeyboardEvent, ModifiersState, MouseEvent, TouchEvent};

pub enum InputEvent {
    Keyboard(KeyboardEvent),
    ModifiersChanged(ModifiersState),
    Mouse(MouseEvent),
    Touch(TouchEvent),
    Gamepad(GamepadEvent),
}

impl InputEvent {
    pub fn is_interacting(&self) -> bool {
        match self {
            InputEvent::Keyboard(_) |
            InputEvent::Gamepad(_) |
            InputEvent::Mouse(MouseEvent::MouseScroll(_)) |
            InputEvent::Mouse(MouseEvent::MouseInput(_)) |
            InputEvent::Touch(_) => true,
            _ => false,
        }
    }
}

pub enum WindowEvent {
    Closing,
    Resized(UVec2),
    Input(InputEvent),
    ScaleFactorChanged(f32),
    PasteFromClipboard(String),
    FullScreenChanged(bool),
}

impl WindowEvent {
    pub fn is_interacting(&self) -> bool {
        match self {
            WindowEvent::Input(event) => event.is_interacting(),
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct WindowState {
    input: InputState,
    size: UVec2,
    scale: f32,
    fullscreen: bool,
    closing: bool,
    clipboard_data: Option<String>,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            size: UVec2::ZERO,
            scale: 1.0,
            fullscreen: false,
            closing: false,
            clipboard_data: None,
        }
    }

    pub fn sized(size: UVec2, scale: f32) -> Self {
        Self {
            size,
            scale,
            ..Self::new()
        }
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn closing(&self) -> bool {
        self.closing
    }

    pub fn norm_cursor_pos(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let x = self.input.mouse().cursor_pos().x as f32 * scale;
        let y = self.input.mouse().cursor_pos().y as f32 * scale;
        Vec2::new(x, y)
    }

    pub fn norm_cursor_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.input.mouse().cursor_delta().x as f32 * scale;
        let dy = -self.input.mouse().cursor_delta().y as f32 * scale;
        Vec2::new(dx, dy)
    }

    pub fn norm_mouse_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.input.mouse().delta().x as f32 * scale;
        let dy = -self.input.mouse().delta().y as f32 * scale;
        Vec2::new(dx, dy)
    }

    pub fn take_clipboard_data(&mut self) -> Option<String> {
        self.clipboard_data.take()
    }

    pub fn clear_events(&mut self) {
        self.input.clear_events();
        self.closing = false;
        self.clipboard_data = None;
    }

    pub fn apply(&mut self, event: WindowEvent, scroll_pixel_factor: f32) {
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
            WindowEvent::Input(event) => {
                self.input.apply(event, self.size, scroll_pixel_factor);
            }
            WindowEvent::PasteFromClipboard(s) => self.clipboard_data = Some(s),
            WindowEvent::FullScreenChanged(fullscreen) => self.fullscreen = fullscreen,
        }
    }

    pub fn flush(&mut self) {
        self.input.flush();
    }
}
