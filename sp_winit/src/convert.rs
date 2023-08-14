use glam::{IVec2, UVec2, Vec2};
use winit::event::MouseScrollDelta;
use sp_input::*;

pub fn convert_element_state(element_state: winit::event::ElementState) -> ElementState {
    match element_state {
        winit::event::ElementState::Pressed => ElementState::Pressed,
        winit::event::ElementState::Released => ElementState::Released,
    }
}

pub fn convert_mouse_button(mouse_button: winit::event::MouseButton) -> MouseButton {
    match mouse_button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Other(val) => MouseButton::Other(val),
    }
}

pub fn convert_virtual_key_code(virtual_key_code: winit::event::VirtualKeyCode) -> KeyCode {
    match virtual_key_code {
        winit::event::VirtualKeyCode::Key1 => KeyCode::Key1,
        winit::event::VirtualKeyCode::Key2 => KeyCode::Key2,
        winit::event::VirtualKeyCode::Key3 => KeyCode::Key3,
        winit::event::VirtualKeyCode::Key4 => KeyCode::Key4,
        winit::event::VirtualKeyCode::Key5 => KeyCode::Key5,
        winit::event::VirtualKeyCode::Key6 => KeyCode::Key6,
        winit::event::VirtualKeyCode::Key7 => KeyCode::Key7,
        winit::event::VirtualKeyCode::Key8 => KeyCode::Key8,
        winit::event::VirtualKeyCode::Key9 => KeyCode::Key9,
        winit::event::VirtualKeyCode::Key0 => KeyCode::Key0,
        winit::event::VirtualKeyCode::A => KeyCode::A,
        winit::event::VirtualKeyCode::B => KeyCode::B,
        winit::event::VirtualKeyCode::C => KeyCode::C,
        winit::event::VirtualKeyCode::D => KeyCode::D,
        winit::event::VirtualKeyCode::E => KeyCode::E,
        winit::event::VirtualKeyCode::F => KeyCode::F,
        winit::event::VirtualKeyCode::G => KeyCode::G,
        winit::event::VirtualKeyCode::H => KeyCode::H,
        winit::event::VirtualKeyCode::I => KeyCode::I,
        winit::event::VirtualKeyCode::J => KeyCode::J,
        winit::event::VirtualKeyCode::K => KeyCode::K,
        winit::event::VirtualKeyCode::L => KeyCode::L,
        winit::event::VirtualKeyCode::M => KeyCode::M,
        winit::event::VirtualKeyCode::N => KeyCode::N,
        winit::event::VirtualKeyCode::O => KeyCode::O,
        winit::event::VirtualKeyCode::P => KeyCode::P,
        winit::event::VirtualKeyCode::Q => KeyCode::Q,
        winit::event::VirtualKeyCode::R => KeyCode::R,
        winit::event::VirtualKeyCode::S => KeyCode::S,
        winit::event::VirtualKeyCode::T => KeyCode::T,
        winit::event::VirtualKeyCode::U => KeyCode::U,
        winit::event::VirtualKeyCode::V => KeyCode::V,
        winit::event::VirtualKeyCode::W => KeyCode::W,
        winit::event::VirtualKeyCode::X => KeyCode::X,
        winit::event::VirtualKeyCode::Y => KeyCode::Y,
        winit::event::VirtualKeyCode::Z => KeyCode::Z,
        winit::event::VirtualKeyCode::Escape => KeyCode::Escape,
        winit::event::VirtualKeyCode::F1 => KeyCode::F1,
        winit::event::VirtualKeyCode::F2 => KeyCode::F2,
        winit::event::VirtualKeyCode::F3 => KeyCode::F3,
        winit::event::VirtualKeyCode::F4 => KeyCode::F4,
        winit::event::VirtualKeyCode::F5 => KeyCode::F5,
        winit::event::VirtualKeyCode::F6 => KeyCode::F6,
        winit::event::VirtualKeyCode::F7 => KeyCode::F7,
        winit::event::VirtualKeyCode::F8 => KeyCode::F8,
        winit::event::VirtualKeyCode::F9 => KeyCode::F9,
        winit::event::VirtualKeyCode::F10 => KeyCode::F10,
        winit::event::VirtualKeyCode::F11 => KeyCode::F11,
        winit::event::VirtualKeyCode::F12 => KeyCode::F12,
        winit::event::VirtualKeyCode::F13 => KeyCode::F13,
        winit::event::VirtualKeyCode::F14 => KeyCode::F14,
        winit::event::VirtualKeyCode::F15 => KeyCode::F15,
        winit::event::VirtualKeyCode::F16 => KeyCode::F16,
        winit::event::VirtualKeyCode::F17 => KeyCode::F17,
        winit::event::VirtualKeyCode::F18 => KeyCode::F18,
        winit::event::VirtualKeyCode::F19 => KeyCode::F19,
        winit::event::VirtualKeyCode::F20 => KeyCode::F20,
        winit::event::VirtualKeyCode::F21 => KeyCode::F21,
        winit::event::VirtualKeyCode::F22 => KeyCode::F22,
        winit::event::VirtualKeyCode::F23 => KeyCode::F23,
        winit::event::VirtualKeyCode::F24 => KeyCode::F24,
        winit::event::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
        winit::event::VirtualKeyCode::Scroll => KeyCode::Scroll,
        winit::event::VirtualKeyCode::Pause => KeyCode::Pause,
        winit::event::VirtualKeyCode::Insert => KeyCode::Insert,
        winit::event::VirtualKeyCode::Home => KeyCode::Home,
        winit::event::VirtualKeyCode::Delete => KeyCode::Delete,
        winit::event::VirtualKeyCode::End => KeyCode::End,
        winit::event::VirtualKeyCode::PageDown => KeyCode::PageDown,
        winit::event::VirtualKeyCode::PageUp => KeyCode::PageUp,
        winit::event::VirtualKeyCode::Left => KeyCode::Left,
        winit::event::VirtualKeyCode::Up => KeyCode::Up,
        winit::event::VirtualKeyCode::Right => KeyCode::Right,
        winit::event::VirtualKeyCode::Down => KeyCode::Down,
        winit::event::VirtualKeyCode::Back => KeyCode::Back,
        winit::event::VirtualKeyCode::Return => KeyCode::Return,
        winit::event::VirtualKeyCode::Space => KeyCode::Space,
        winit::event::VirtualKeyCode::Compose => KeyCode::Compose,
        winit::event::VirtualKeyCode::Caret => KeyCode::Caret,
        winit::event::VirtualKeyCode::Numlock => KeyCode::Numlock,
        winit::event::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
        winit::event::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
        winit::event::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
        winit::event::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
        winit::event::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
        winit::event::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
        winit::event::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
        winit::event::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
        winit::event::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
        winit::event::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
        winit::event::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
        winit::event::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
        winit::event::VirtualKeyCode::NumpadAdd => KeyCode::NumpadAdd,
        winit::event::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
        winit::event::VirtualKeyCode::Apps => KeyCode::Apps,
        winit::event::VirtualKeyCode::Asterisk => KeyCode::Asterisk,
        winit::event::VirtualKeyCode::Plus => KeyCode::Plus,
        winit::event::VirtualKeyCode::At => KeyCode::At,
        winit::event::VirtualKeyCode::Ax => KeyCode::Ax,
        winit::event::VirtualKeyCode::Backslash => KeyCode::Backslash,
        winit::event::VirtualKeyCode::Calculator => KeyCode::Calculator,
        winit::event::VirtualKeyCode::Capital => KeyCode::Capital,
        winit::event::VirtualKeyCode::Colon => KeyCode::Colon,
        winit::event::VirtualKeyCode::Comma => KeyCode::Comma,
        winit::event::VirtualKeyCode::Convert => KeyCode::Convert,
        winit::event::VirtualKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
        winit::event::VirtualKeyCode::NumpadDivide => KeyCode::NumpadDivide,
        winit::event::VirtualKeyCode::Equals => KeyCode::Equals,
        winit::event::VirtualKeyCode::Grave => KeyCode::Grave,
        winit::event::VirtualKeyCode::Kana => KeyCode::Kana,
        winit::event::VirtualKeyCode::Kanji => KeyCode::Kanji,
        winit::event::VirtualKeyCode::LAlt => KeyCode::LAlt,
        winit::event::VirtualKeyCode::LBracket => KeyCode::LBracket,
        winit::event::VirtualKeyCode::LControl => KeyCode::LControl,
        winit::event::VirtualKeyCode::LShift => KeyCode::LShift,
        winit::event::VirtualKeyCode::LWin => KeyCode::LWin,
        winit::event::VirtualKeyCode::Mail => KeyCode::Mail,
        winit::event::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
        winit::event::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
        winit::event::VirtualKeyCode::Minus => KeyCode::Minus,
        winit::event::VirtualKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
        winit::event::VirtualKeyCode::Mute => KeyCode::Mute,
        winit::event::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
        winit::event::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
        winit::event::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
        winit::event::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
        winit::event::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
        winit::event::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
        winit::event::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
        winit::event::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
        winit::event::VirtualKeyCode::OEM102 => KeyCode::Oem102,
        winit::event::VirtualKeyCode::Period => KeyCode::Period,
        winit::event::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
        winit::event::VirtualKeyCode::Power => KeyCode::Power,
        winit::event::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
        winit::event::VirtualKeyCode::RAlt => KeyCode::RAlt,
        winit::event::VirtualKeyCode::RBracket => KeyCode::RBracket,
        winit::event::VirtualKeyCode::RControl => KeyCode::RControl,
        winit::event::VirtualKeyCode::RShift => KeyCode::RShift,
        winit::event::VirtualKeyCode::RWin => KeyCode::RWin,
        winit::event::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
        winit::event::VirtualKeyCode::Slash => KeyCode::Slash,
        winit::event::VirtualKeyCode::Sleep => KeyCode::Sleep,
        winit::event::VirtualKeyCode::Stop => KeyCode::Stop,
        winit::event::VirtualKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
        winit::event::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
        winit::event::VirtualKeyCode::Tab => KeyCode::Tab,
        winit::event::VirtualKeyCode::Underline => KeyCode::Underline,
        winit::event::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
        winit::event::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
        winit::event::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
        winit::event::VirtualKeyCode::Wake => KeyCode::Wake,
        winit::event::VirtualKeyCode::WebBack => KeyCode::WebBack,
        winit::event::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
        winit::event::VirtualKeyCode::WebForward => KeyCode::WebForward,
        winit::event::VirtualKeyCode::WebHome => KeyCode::WebHome,
        winit::event::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
        winit::event::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
        winit::event::VirtualKeyCode::WebStop => KeyCode::WebStop,
        winit::event::VirtualKeyCode::Yen => KeyCode::Yen,
        winit::event::VirtualKeyCode::Copy => KeyCode::Copy,
        winit::event::VirtualKeyCode::Paste => KeyCode::Paste,
        winit::event::VirtualKeyCode::Cut => KeyCode::Cut,
    }
}

fn convert_scroll_delta(delta: &MouseScrollDelta) -> Vec2 {
    match delta {
        MouseScrollDelta::LineDelta(dx, dy) => 
            Vec2::new(*dx, *dy),
        MouseScrollDelta::PixelDelta(delta) => 
            Vec2::new(delta.x as f32, delta.y as f32),
    }
}

fn convert_touch_phase(phase: winit::event::TouchPhase) -> TouchPhase {
    match phase {
        winit::event::TouchPhase::Started => TouchPhase::Started,
        winit::event::TouchPhase::Moved => TouchPhase::Moved,
        winit::event::TouchPhase::Ended => TouchPhase::Ended,
        winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
    }
}

pub fn convert_device_event(event: &winit::event::DeviceEvent) -> Option<WindowEvent> {
    match event {
        winit::event::DeviceEvent::Added => None,
        winit::event::DeviceEvent::Removed => None,
        winit::event::DeviceEvent::MouseMotion { delta } => Some(WindowEvent::Input(InputEvent::Mouse(MouseEvent::MouseMoved(
            Vec2::new(delta.0 as f32, delta.1 as f32),
        )))),
        winit::event::DeviceEvent::MouseWheel { delta: _ } =>
            // Ignore event here to avoid duplicate with window event
            None,
        winit::event::DeviceEvent::Motion { axis: _, value: _ } => None,
        winit::event::DeviceEvent::Button {
            button: _,
            state: _,
        } => None,
        winit::event::DeviceEvent::Key(_) => None,
        winit::event::DeviceEvent::Text { codepoint: _ } => None,
    }
}

#[allow(deprecated)]
pub fn convert_window_event(event: &winit::event::WindowEvent) -> Option<WindowEvent> {
    match event {
        winit::event::WindowEvent::Moved(_) => None,
        winit::event::WindowEvent::CloseRequested => Some(WindowEvent::Closing),
        winit::event::WindowEvent::Destroyed => None,
        winit::event::WindowEvent::DroppedFile(_) => None,
        winit::event::WindowEvent::HoveredFile(_) => None,
        winit::event::WindowEvent::HoveredFileCancelled => None,
        winit::event::WindowEvent::ReceivedCharacter(_) => None,
        winit::event::WindowEvent::Focused(_) => None,
        winit::event::WindowEvent::KeyboardInput {
            device_id: _,
            input,
            is_synthetic: _,
        } => {
            //log::info!("Key: {:?}", input);
            input.virtual_keycode.map(|key| {
                WindowEvent::Input(InputEvent::Keyboard(KeyboardEvent {
                    state: convert_element_state(input.state),
                    key: convert_virtual_key_code(key),
                }))
            })
        },
        winit::event::WindowEvent::ModifiersChanged(state) => Some(WindowEvent::Input(InputEvent::ModifiersChanged(
            ModifiersState::from_bits(state.bits()).unwrap()),
        )),
        winit::event::WindowEvent::CursorMoved {
            device_id: _,
            position,
            modifiers: _,
        } => Some(WindowEvent::Input(InputEvent::Mouse(MouseEvent::CursorMoved(IVec2::new(
            position.x as i32,
            position.y as i32,
        ))))),
        winit::event::WindowEvent::CursorEntered { device_id: _ } => None,
        winit::event::WindowEvent::CursorLeft { device_id: _ } => None,
        winit::event::WindowEvent::MouseWheel {
            device_id: _,
            delta,
            phase: _,
            modifiers: _,
        } => Some(WindowEvent::Input(InputEvent::Mouse(MouseEvent::MouseWheel(convert_scroll_delta(delta))))),
        winit::event::WindowEvent::MouseInput {
            device_id: _,
            state,
            button,
            modifiers: _,
        } => Some(WindowEvent::Input(InputEvent::Mouse(MouseEvent::MouseInput(MouseButtonEvent {
            button: convert_mouse_button(*button),
            state: convert_element_state(*state),
        })))),
        winit::event::WindowEvent::TouchpadPressure {
            device_id: _,
            pressure: _,
            stage: _,
        } => None,
        winit::event::WindowEvent::AxisMotion {
            device_id: _,
            axis: _,
            value: _,
        } => None,
        winit::event::WindowEvent::Touch(touch) => 
            Some(WindowEvent::Input(InputEvent::Touch(TouchEvent {
                phase: convert_touch_phase(touch.phase),
                pos: Vec2::new(touch.location.x as f32, touch.location.y as f32),
                id: touch.id
            }))),
        winit::event::WindowEvent::Resized(size) => {
            Some(WindowEvent::Resized(UVec2::new(size.width, size.height)))
        }
        winit::event::WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size: _,
        } => Some(WindowEvent::ScaleFactorChanged(*scale_factor as f32)),
        winit::event::WindowEvent::ThemeChanged(_) => None,
        winit::event::WindowEvent::Ime(_) => None,
        winit::event::WindowEvent::Occluded(_) => None,
    }
}
