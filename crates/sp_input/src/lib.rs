#![forbid(unsafe_code)]

pub mod axis;
pub mod cmd;
pub mod key;
pub mod mouse;
pub mod state;

pub use cmd::*;
pub use mouse::*;
pub use state::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ElementState {
    Pressed,
    Released,
}
