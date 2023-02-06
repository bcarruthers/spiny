use crate::convert::*;
use glam::{IVec2, UVec2, Vec2};
use sp_input::state::*;
use winit::event::MouseScrollDelta;

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
        winit::event::DeviceEvent::MouseMotion { delta } => Some(WindowEvent::MouseMoved(
            Vec2::new(delta.0 as f32, delta.1 as f32),
        )),
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
        } => input.virtual_keycode.map(|key| {
            WindowEvent::KeyboardInput(KeyboardEvent {
                state: convert_element_state(input.state),
                key: convert_virtual_key_code(key),
            })
        }),
        winit::event::WindowEvent::ModifiersChanged(state) => Some(WindowEvent::ModifiersChanged(
            ModifiersState::from_bits(state.bits()).unwrap(),
        )),
        winit::event::WindowEvent::CursorMoved {
            device_id: _,
            position,
            modifiers: _,
        } => Some(WindowEvent::CursorMoved(IVec2::new(
            position.x as i32,
            position.y as i32,
        ))),
        winit::event::WindowEvent::CursorEntered { device_id: _ } => None,
        winit::event::WindowEvent::CursorLeft { device_id: _ } => None,
        winit::event::WindowEvent::MouseWheel {
            device_id: _,
            delta,
            phase: _,
            modifiers: _,
        } => Some(WindowEvent::MouseWheel(convert_scroll_delta(delta))),
        winit::event::WindowEvent::MouseInput {
            device_id: _,
            state,
            button,
            modifiers: _,
        } => Some(WindowEvent::MouseInput(MouseEvent {
            button: convert_mouse_button(*button),
            state: convert_element_state(*state),
        })),
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
            Some(WindowEvent::Touch(TouchEvent {
                phase: convert_touch_phase(touch.phase),
                pos: Vec2::new(touch.location.x as f32, touch.location.y as f32),
                id: touch.id
            })),
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
