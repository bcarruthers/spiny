use glam::UVec2;

// From gilrs
#[repr(u16)]
#[derive(Debug)]
pub enum GamepadButton {
    South,
    East,
    North,
    West,
    C,
    Z,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Unknown,
}

#[repr(u16)]
#[derive(Debug)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
    Unknown,
}

#[derive(Debug)]
pub struct GamepadCode(u32);

#[derive(Debug)]
pub enum GamepadEventType {
    ButtonPressed(GamepadButton, GamepadCode),
    ButtonRepeated(GamepadButton, GamepadCode),
    ButtonReleased(GamepadButton, GamepadCode),
    ButtonChanged(GamepadButton, f32, GamepadCode),
    AxisChanged(GamepadAxis, f32, GamepadCode),
    Connected,
    Disconnected,
    Dropped,
}

#[derive(Debug)]
pub struct GamepadEvent {
    //pub id: GamepadId,
    pub event: GamepadEventType,
    //pub time: SystemTime,
}

#[derive(Clone, Default)]
pub struct GamepadState {

}

impl GamepadState {
    pub fn update(&mut self, event: &GamepadEvent, size: UVec2) {
        log::info!("Gamepad: {:?}", event);
    }
}