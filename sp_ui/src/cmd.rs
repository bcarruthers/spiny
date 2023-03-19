use super::*;
use glam::{IVec2, UVec2};
use sp_asset::AssetId;
use sp_math::color::IRgba;

#[derive(Debug, Clone)]
pub enum NavigateCmd {
    Previous,
    Next,
    First,
    Last,
    Invoke,
    Cancel,
    Press,
    Release,
}

#[derive(Clone)]
pub enum CanvasOrigin {
    Window,
    Cursor,
}

#[derive(Clone)]
pub struct TileCanvas<Cmd> {
    pub origin: CanvasOrigin,
    pub align: Align,
    pub valign: Valign,
    pub size: UVec2,
    pub on_press: Option<Cmd>,
    pub on_release: Option<Cmd>,
}

impl<Cmd> Default for TileCanvas<Cmd> {
    fn default() -> Self {
        Self {
            origin: CanvasOrigin::Window,
            align: Align::Left,
            valign: Valign::Top,
            size: UVec2::ZERO,
            on_press: None,
            on_release: None,
        }
    }
}

impl<Cmd> TileCanvas<Cmd> {
    /// Returns bounds of canvas in window
    pub fn bounds(&self, tile_size: UVec2, window_size: UVec2, cursor_pos: IVec2) -> IRange2 {
        let size = (self.size * tile_size).as_ivec2();
        let pos = match self.origin {
            CanvasOrigin::Window => {
                let p0 = IVec2::ZERO;
                let p1 = window_size.as_ivec2();
                let box_size = p1 - p0;
                let x0 = match self.align {
                    Align::Left => p0.x,
                    Align::Right => p1.x - size.x,
                    Align::Center => p0.x + (box_size.x - size.x) / 2,
                };
                let y0 = match self.valign {
                    Valign::Top => p0.y,
                    Valign::Bottom => p1.y - size.y,
                    Valign::Center => p0.y + (box_size.y - size.y) / 2,
                };
                IVec2::new(x0, y0)
            }
            CanvasOrigin::Cursor => cursor_pos,
        };
        IRange2::sized(pos, size)
    }
}

pub mod border {
    pub const SINGLE: [char; 16] = [
        ' ',        // ....
        '\u{00c4}', //'─', // ...x
        '\u{00c4}', //'─', // ..x.
        '\u{00c4}', //'─', // ..xx
        '\u{00b3}', //'│', // .x..
        '\u{00d9}', //'┘', // .x.x
        '\u{00c0}', //'└', // .xx.
        '\u{00c1}', //'┴', // .xxx
        '\u{00b3}', //'│', // x...
        '\u{00bf}', //'┐', // x..x
        '\u{00da}', //'┌', // x.x.
        '\u{00c2}', //'┬', // x.xx
        '\u{00b3}', //'│', // xx..
        '\u{00b4}', //'┤', // xx.x
        '\u{00c3}', //'├', // xxx.
        '\u{00c5}', //'┼', // xxxx
    ];

    pub const DOUBLE: [char; 16] = [
        ' ',        // ....
        '\u{00cd}', //'─', // ...x
        '\u{00cd}', //'─', // ..x.
        '\u{00cd}', //'─', // ..xx
        '\u{00ba}', //'│', // .x..
        '\u{00bc}', //'┘', // .x.x
        '\u{00c8}', //'└', // .xx.
        '\u{00ca}', //'┴', // .xxx
        '\u{00ba}', //'│', // x...
        '\u{00bb}', //'┐', // x..x
        '\u{00c9}', //'┌', // x.x.
        '\u{00cb}', //'┬', // x.xx
        '\u{00ba}', //'│', // xx..
        '\u{00b9}', //'┤', // xx.x
        '\u{00cc}', //'├', // xxx.
        '\u{00ce}', //'┼', // xxxx
    ];
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct BorderAdjacency: u8 {
        const NONE = 0;
        const XMIN = 1;
        const XMAX = 2;
        const YMIN = 4;
        const YMAX = 8;
    }
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    Uniform(char),
    Single,
    Double,
}

impl BorderStyle {
    pub fn get_char(&self, adj: BorderAdjacency) -> char {
        match self {
            BorderStyle::Uniform(ch) => *ch,
            BorderStyle::Single => border::SINGLE[adj.bits as usize],
            BorderStyle::Double => border::DOUBLE[adj.bits as usize],
        }
    }
}

#[derive(Clone)]
pub struct UiImage {
    pub size: UVec2,
    pub scale: f32,
    pub asset_id: AssetId,
}

#[derive(Clone, Default)]
pub struct UiText {
    pub size: UVec2,
    pub content: String,
    pub overlay: bool,
}

impl UiText {
    pub fn measure_str(s: &str) -> UVec2 {
        if s.is_empty() {
            UVec2::ZERO
        } else {
            let mut x = 0;
            let mut y = 1;
            let mut x_max = 0;
            for ch in s.chars() {
                match ch {
                    '\n' => {
                        y += 1;
                        x_max = x_max.max(x);
                        x = 0;
                    }
                    '\r' => (),
                    _ => x += 1,
                }
            }
            UVec2::new(x_max.max(x), y)
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            size: Self::measure_str(s),
            content: s.to_string(),
            overlay: false,
        }
    }
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct UiButtonFlags: u32 {
        const NONE = 0;
        const SELECTED = 1;
        const CANCEL = 2;
        const DEFAULT = 4;
        const HOVERABLE = 8;
    }
}

#[derive(Clone)]
pub struct UiWidget<Cmd> {
    pub on_press: Option<Cmd>,
    pub on_release: Option<Cmd>,
    pub flags: UiButtonFlags,
}

impl<Cmd> Default for UiWidget<Cmd> {
    fn default() -> Self {
        Self {
            on_press: None,
            on_release: None,
            flags: UiButtonFlags::NONE,
        }
    }
}

impl<Cmd> UiWidget<Cmd> {
    pub fn can_hover(&self) -> bool {
        self.on_press.is_some()
            || self.on_release.is_some()
            || self.flags.contains(UiButtonFlags::HOVERABLE)
    }

    pub fn press(cmd: Cmd) -> Self {
        Self {
            on_press: Some(cmd),
            ..Default::default()
        }
    }

    pub fn release(cmd: Cmd) -> Self {
        Self {
            on_release: Some(cmd),
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct UiStyles {
    pub normal: Style,
    pub hover: Style,
}

impl UiStyles {
    pub const DEFAULT: Self = Self {
        normal: Style::new(IRgba::WHITE, IRgba::BLACK),
        hover: Style::new(IRgba::BLACK, IRgba::WHITE),
    };

    pub fn lerp_fg_to_bg(&self, alpha: f32) -> Self {
        Self {
            normal: self.normal.lerp_fg_to_bg(alpha),
            hover: self.hover.lerp_fg_to_bg(alpha),
        }
    }
}

impl Default for UiStyles {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone)]
pub enum UiDrawCmd<Cmd> {
    BeginCanvas(TileCanvas<Cmd>),
    EndCanvas,
    BeginStyle(UiStyles),
    EndStyle,
    BeginLayer(u32),
    EndLayer,
    Position(IVec2),
    BeginWidget(UiWidget<Cmd>),
    EndWidget,
    Region(UVec2),
    Text(UiText),
    Image(UiImage),
}

impl<Cmd: Clone> UiDrawCmd<Cmd> {
    pub fn size(&self) -> Option<UVec2> {
        match self {
            UiDrawCmd::Region(size) => Some(*size),
            UiDrawCmd::Text(text) => Some(text.size),
            UiDrawCmd::Image(image) => Some(image.size),
            _ => None,
        }
    }
}
