use std::hash::Hash;
use indexmap::{IndexMap, map::Entry};
use serde_derive::*;

// From gilrs
#[repr(u16)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

impl GamepadButton {
    pub const COUNT: usize = 19;
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl GamepadAxis {
    pub const COUNT: usize = 8;
}

#[derive(Clone, Copy, Debug)]
pub struct GamepadCode(pub u32);

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct GamepadId(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct GamepadEvent {
    pub id: GamepadId,
    pub ty: GamepadEventType,
    pub time: i64,
}

#[derive(Clone, Default)]
pub struct GamepadState {
    just_pressed: [bool; GamepadButton::COUNT],
    just_released: [bool; GamepadButton::COUNT],
    pressed: [bool; GamepadButton::COUNT],
    button_values: [f32; GamepadButton::COUNT],
    axes: [f32; GamepadAxis::COUNT],
}

impl GamepadState {
    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        self.axes[axis as usize]
    }

    pub fn button_value(&self, button: GamepadButton) -> f32 {
        self.button_values[button as usize]
    }

    pub fn just_pressed(&self, button: GamepadButton) -> bool {
        self.just_pressed[button as usize]
    }

    pub fn just_released(&self, button: GamepadButton) -> bool {
        self.just_released[button as usize]
    }

    pub fn pressed(&self, button: GamepadButton) -> bool {
        self.pressed[button as usize]
    }

    pub fn any_pressed(&self, buttons: &[GamepadButton]) -> bool {
        buttons.iter().any(|&button| self.pressed(button))
    }

    pub fn any_just_pressed(&self, buttons: &[GamepadButton]) -> bool {
        buttons.iter().any(|&button| self.just_pressed(button))
    }

    pub fn clear_events(&mut self) {
        self.just_pressed = [false; GamepadButton::COUNT];
        self.just_released = [false; GamepadButton::COUNT];
    }

    pub fn update(&mut self, event: &GamepadEvent) {
        //log::info!("Gamepad: {:?}", event);
        match event.ty {
            GamepadEventType::ButtonPressed(button, _) => {
                self.just_pressed[button as usize] = true;
                self.pressed[button as usize] = true;
            }
            GamepadEventType::ButtonRepeated(button, _) => {
                self.just_pressed[button as usize] = true;
                self.pressed[button as usize] = true;
            }
            GamepadEventType::ButtonReleased(button, _) => {
                self.just_released[button as usize] = true;
                self.pressed[button as usize] = false;
            }
            GamepadEventType::ButtonChanged(button, value, _) => {
                self.button_values[button as usize] = value;
            }
            GamepadEventType::AxisChanged(axis, value, _) => {
                self.axes[axis as usize] = value;
            }
            GamepadEventType::Connected => {},
            GamepadEventType::Disconnected => {},
            GamepadEventType::Dropped => {},
        }
    }
}

pub struct GamepadCmdBinding<Cmd> {
    pub command: Cmd,
    pub button: GamepadButton,
}

pub struct CommandGamepadButtonMap<Cmd> {
    map: IndexMap<Cmd, Vec<GamepadButton>>,
}

impl<Cmd> Default for CommandGamepadButtonMap<Cmd> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<Cmd: Eq + Hash> CommandGamepadButtonMap<Cmd> {
    pub fn get_buttons(&self, cmd: Cmd) -> &[GamepadButton] {
        match self.map.get(&cmd) {
            Some(buttons) => &buttons,
            None => &[],
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn add(&mut self, binding: GamepadCmdBinding<Cmd>) {
        match self.map.entry(binding.command) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(binding.button);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![binding.button]);
            }
        }
    }
}

impl<Cmd: Eq + Hash + Copy> CommandGamepadButtonMap<Cmd> {
    pub fn button_down_cmd(&self, input: &GamepadState, cmd: Cmd) -> Option<Cmd> {
        if self.get_buttons(cmd).iter().any(|&button| input.pressed(button)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn button_just_down_cmd(&self, input: &GamepadState, cmd: Cmd) -> Option<Cmd> {
        if self.get_buttons(cmd).iter().any(|&button| input.just_pressed(button)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn button_up_cmd(&self, input: &GamepadState, cmd: Cmd) -> Option<Cmd> {
        if self.get_buttons(cmd).iter().any(|&button| !input.pressed(button)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn button_just_up_cmd(&self, input: &GamepadState, cmd: Cmd) -> Option<Cmd> {
        if self.get_buttons(cmd).iter().any(|&button| input.just_released(button)) {
            Some(cmd)
        } else {
            None
        }
    }

    pub fn button_down_cmds<'a>(
        &'a self,
        input: &'a GamepadState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter().filter_map(|cmd| self.button_down_cmd(input, *cmd))
    }

    pub fn button_just_down_cmds<'a>(
        &'a self,
        input: &'a GamepadState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter()
            .filter_map(|cmd| self.button_just_down_cmd(input, *cmd))
    }

    pub fn button_up_cmds<'a>(
        &'a self,
        input: &'a GamepadState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter().filter_map(|cmd| self.button_up_cmd(input, *cmd))
    }

    pub fn button_just_up_cmds<'a>(
        &'a self,
        input: &'a GamepadState,
        set: &'a [Cmd],
    ) -> impl Iterator<Item = Cmd> + 'a {
        set.iter()
            .filter_map(|cmd| self.button_just_up_cmd(input, *cmd))
    }
}
