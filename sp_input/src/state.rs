use glam::*;
use crate::*;

pub enum WindowEvent {
    Closing,
    Resized(UVec2),
    ScaleFactorChanged(f32),
    KeyboardInput(KeyboardEvent),
    ModifiersChanged(ModifiersState),
    Mouse(MouseEvent),
    Touch(TouchEvent),
    Gamepad(GamepadEvent),
    PasteFromClipboard(String),
    FullScreenChanged(bool),
}

impl WindowEvent {
    pub fn is_interacting(&self) -> bool {
        match self {
            WindowEvent::KeyboardInput(_) |
            WindowEvent::Mouse(MouseEvent::MouseWheel(_)) |
            WindowEvent::Mouse(MouseEvent::MouseInput(_)) |
            WindowEvent::Touch(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct WindowState {
    keyboard: KeyboardState,
    mouse: MouseState,
    touch: TouchState,
    gamepad: GamepadState,
    size: UVec2,
    scale: f32,
    fullscreen: bool,
    closing: bool,
    clipboard_data: Option<String>,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            keyboard: Default::default(),
            mouse: Default::default(),
            touch: Default::default(),
            gamepad: Default::default(),
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

    pub fn keyboard(&self) -> &KeyboardState {
        &self.keyboard
    }

    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }

    pub fn touch(&self) -> &TouchState {
        &self.touch
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
        let x = self.mouse.cursor_pos().x as f32 * scale;
        let y = self.mouse.cursor_pos().y as f32 * scale;
        Vec2::new(x, y)
    }

    pub fn norm_cursor_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.mouse.cursor_delta().x as f32 * scale;
        let dy = -self.mouse.cursor_delta().y as f32 * scale;
        Vec2::new(dx, dy)
    }

    pub fn norm_mouse_delta(&self) -> Vec2 {
        let scale = 1.0f32 / self.size.x.min(self.size.y) as f32;
        let dx = -self.mouse.delta().x as f32 * scale;
        let dy = -self.mouse.delta().y as f32 * scale;
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
            WindowEvent::Mouse(event) => self.mouse.update(event, self.size),
            WindowEvent::Touch(event) => self.touch.update(&event, self.size),
            WindowEvent::Gamepad(event) => self.gamepad.update(&event, self.size),
            WindowEvent::PasteFromClipboard(s) => self.clipboard_data = Some(s),
            WindowEvent::FullScreenChanged(fullscreen) => self.fullscreen = fullscreen,
        }
    }

    pub fn flush(&mut self) {
        self.keyboard.flush();
    }
}
