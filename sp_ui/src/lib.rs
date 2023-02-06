pub mod ansi;
pub mod cmd;
pub mod draw;
pub mod pick;
pub mod text;
pub mod tile;

pub use cmd::*;
pub use draw::*;
use sp_math::range::IRange2;
pub use text::*;

bitflags::bitflags! {
    #[derive(Default)]
    pub struct UiElementFlags: u32 {
        const SELECTED = 1;
        const HOVERED = 2;
        const PRESSED = 4;
        const JUST_PRESSED = 8;
    }
}

#[derive(Clone)]
pub struct UiElementColors {
    pub normal: u32,
    pub select: u32,
    pub hover: u32,
    pub press: u32,
}

impl UiElementColors {
    pub fn flags_color(&self, flags: UiElementFlags) -> u32 {
        if flags.contains(UiElementFlags::PRESSED) {
            self.press
        } else if flags.contains(UiElementFlags::HOVERED) {
            self.hover
        } else if flags.contains(UiElementFlags::SELECTED) {
            self.select
        } else {
            self.normal
        }
    }
}

#[derive(Clone)]
pub struct UiElementStyle {
    pub fg: UiElementColors,
    pub bg: UiElementColors,
}

impl Default for UiElementStyle {
    fn default() -> Self {
        Self {
            fg: UiElementColors {
                normal: 0xffffffff,
                select: 0xffffffff,
                hover: 0xffffffff,
                press: 0xffffffff,
            },
            bg: UiElementColors {
                normal: 0,
                select: 0,
                hover: 0,
                press: 0,
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
pub struct UiGroup(u64);

#[derive(Clone)]
pub enum UiContent<Cmd> {
    Label(String),
    Image(u64),
    UiWidget(Cmd),
}

impl<Cmd> UiContent<Cmd> {
    pub fn command(&self) -> Option<&Cmd> {
        if let Self::UiWidget(cmd) = self {
            Some(cmd)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct UiElement<Cmd> {
    pub content: UiContent<Cmd>,
    pub group: Option<UiGroup>,
    pub bounds: IRange2,
    pub style: UiElementStyle,
}

// impl<Cmd> UiElement<Cmd> {
//     pub fn label(text: String, group: UiGroup, bounds: IRange2) -> Self {
//         Self {
//             content: UiContent::Label(text),
//             group,
//             bounds,
//             style: Default::default(),
//         }
//     }
// }

impl<Cmd> Default for UiElement<Cmd> {
    fn default() -> Self {
        Self {
            content: UiContent::Label("".to_string()),
            group: None,
            bounds: IRange2::ZERO,
            style: Default::default(),
        }
    }
}

pub enum Scroll {
    Previous,
    Next,
}

impl Scroll {
    pub fn next(&self, index: usize, size: usize) -> usize {
        match self {
            Scroll::Previous => {
                if index == 0 {
                    size - 1
                } else {
                    index - 1
                }
            }
            Scroll::Next => {
                if index == size - 1 {
                    0
                } else {
                    index + 1
                }
            }
        }
    }
}

pub enum UiNavigate {
    Scroll(Scroll),
    Select(UiGroup),
    Invoke,
    Cancel,
}
