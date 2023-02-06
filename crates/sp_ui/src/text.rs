use glam::IVec2;
use glam::*;
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use sp_asset::AssetId;
use sp_math::range::Range2;
use sp_math::{color::IRgba, range::IRange2};
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FontCharDef {
    pub code: char,
    pub width: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub rect_x: i32,
    pub rect_y: i32,
    pub rect_width: i32,
    pub rect_height: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FontDef {
    pub size: i32,
    pub family: String,
    pub style: String,
    pub height: i32,
    pub chars: Vec<FontCharDef>,
}

pub struct FontDefLookup {
    pub fonts: IndexMap<String, FontDef>
}

#[derive(Copy, Clone)]
pub struct Style {
    pub fg: IRgba,
    pub bg: IRgba,
}

impl Style {
    pub const fn new(fg: IRgba, bg: IRgba) -> Self {
        Self { fg, bg }
    }

    pub const fn fg(fg: IRgba) -> Self {
        Self::new(fg, IRgba::ZERO)
    }

    pub const fn bg(bg: IRgba) -> Self {
        Self::new(IRgba::ZERO, bg)
    }

    pub const WHITE: Self = Self::fg(IRgba::WHITE);

    pub fn with_fg(&self, fg: IRgba) -> Self {
        Self::new(fg, self.bg)
    }

    pub fn with_bg(&self, bg: IRgba) -> Self {
        Self::new(self.fg, bg)
    }

    pub fn lerp_fg_to_bg(&self, alpha: f32) -> Self {
        Self::new(self.fg.to_rgba().lerp(self.bg.to_rgba(), alpha).to_irgba(), self.bg)
    }
}

#[derive(Copy, Clone)]
pub enum Align {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Copy, Clone)]
pub enum Valign {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

#[derive(Copy, Clone)]
pub enum TextWrap {
    NoWrap = 0,
    WordWrap = 1,
}

impl TextWrap {
    pub fn max_allowed_width(&self, size: i32) -> i32 {
        match self {
            Self::NoWrap => i32::MAX,
            Self::WordWrap => size,
        }
    }
}

#[derive(Clone)]
pub struct TextSpan<'a> {
    pub text: &'a str,
    pub style: Style,
    pub pos: IVec2,
    pub scale: i32,
}

impl<'a> Default for TextSpan<'a> {
    fn default() -> Self {
        Self {
            text: "",
            style: Style::WHITE,
            pos: IVec2::ZERO,
            scale: 1,
        }
    }
}

#[derive(Clone)]
pub struct TextBlock {
    pub text: String,
    pub style: Style,
    pub bounds: IRange2,
    pub scale: i32,
    pub align: Align,
    pub valign: Valign,
    pub wrap: TextWrap,
    pub spacing: IVec2,
    pub layer_id: u32,
    pub font_id: AssetId,
}

impl<'a> Default for TextBlock {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            style: Style::WHITE,
            bounds: IRange2::ZERO,
            scale: 1,
            align: Align::Left,
            valign: Valign::Top,
            wrap: TextWrap::NoWrap,
            spacing: IVec2::ZERO,
            layer_id: 0,
            font_id: AssetId::default(),
        }
    }
}

    // let getTexBounds (charRect : Range2i) (mapSize : Vector2i) texBounds =
    //     let scale = Vector2.One / (mapSize.ToVector2())
    //     let t0 = charRect.Min.ToVector2() * scale
    //     let t1 = charRect.Max.ToVector2() * scale       
    //     let p0 = Range2.Lerp(texBounds, t0) 
    //     let p1 = Range2.Lerp(texBounds, t1)
    //     Range2(p0, p1)
fn get_tex_bounds(char_rect: IRange2, map_size: IVec2, tex_bounds: &Range2) -> Range2 {
    let scale = Vec2::ONE / map_size.as_vec2();
    let t0 = char_rect.min.as_vec2() * scale;
    let t1 = char_rect.max.as_vec2() * scale;
    let p0 = tex_bounds.lerp(t0);
    let p1 = tex_bounds.lerp(t1);
    Range2::new(p0, p1)
}

#[derive(Default, Clone)]
pub struct FontCharInfo {
    pub width: i32,
    pub offset: IVec2,
    pub size: IVec2,
    pub rect: Range2,
}

impl FontCharInfo {

    // let fromCharDescriptor (mapSize : Vector2i) (p : FontCharDescriptor) (texBounds : Range2) = 
    //     let r = Range2i.Sized(Vector2i(p.RectX, p.RectY), Vector2i(p.RectWidth, p.RectHeight))
    //     {
    //         Width = p.Width
    //         Offset = Vector2i(p.OffsetX, p.OffsetY)
    //         Size = Vector2i(p.RectWidth, p.RectHeight)
    //         Rect = getTexBounds r mapSize texBounds
    //     }
    pub fn from_char_def(map_size: UVec2, p: &FontCharDef, tex_bounds: &Range2) -> Self {
        let r = IRange2::sized(IVec2::new(p.rect_x, p.rect_y), IVec2::new(p.rect_width, p.rect_height));
        Self {
            width: p.width,
            offset: IVec2::new(p.offset_x, p.offset_y),
            size: IVec2::new(p.rect_width, p.rect_height),
            rect: get_tex_bounds(r, map_size.as_ivec2(), tex_bounds),
        }
    }
}

fn char_width(ch: char, widths: &[i32]) -> i32 {
    let index = ch as usize;
    if index < widths.len() {
        widths[index]
    } else {
        0
    }
}

fn run_width(str: &[char], widths: &[i32], run: Range<usize>) -> i32 {
    let mut width = 0;
    for i in run {
        width += char_width(str[i], widths);
    }
    width
}

pub fn next_run(
    str: &[char],
    widths: &[i32],
    start: usize,
    max_allowed_width: i32,
) -> Option<Range<usize>> {
    let mut run_width = 0;
    let mut i = start;
    let mut result = None;
    while result.is_none() && i < str.len() {
        let ch = str[i];
        if ch == '\n' {
            result = Some(start..(i + 1));
        }
        run_width += char_width(str[i], widths);
        if run_width.max(0) > max_allowed_width {
            // Backtrack to start of word
            let mut stop = i;
            while stop > start + 1 && !str[stop].is_whitespace() {
                stop -= 1;
            }
            result = Some(start..(stop + 1));
        }
        i += 1;
    }
    // If no newline or limit reached, return remainder
    if result.is_none() {
        let remaining = str.len() - start;
        if remaining > 0 {
            result = Some(start..str.len());
        }
    }
    result
}

fn measure(str: &[char], widths: &[i32], char_height: i32, max_allowed_width: i32) -> IVec2 {
    let mut max_width = 0;
    let mut count = 0;
    let mut run_opt = next_run(str, widths, 0, max_allowed_width);
    while let Some(run) = run_opt {
        let end = run.end;
        let width = run_width(str, widths, run);
        max_width = max_width.max(width);
        count += 1;
        run_opt = next_run(str, widths, end, max_allowed_width);
    }
    IVec2::new(max_width as i32, count * char_height as i32)
}

fn aligned_bounds(size: IVec2, bounds: IRange2, align: Align, valign: Valign) -> IRange2 {
    let p0 = bounds.min;
    let p1 = bounds.max;
    let box_size = p1 - p0;
    let x0 = match align {
        Align::Left => p0.x,
        Align::Right => p1.x - size.x,
        Align::Center => p0.x + (box_size.x - size.x) / 2,
    };
    let y0 = match valign {
        Valign::Top => p0.y,
        Valign::Bottom => p1.y - size.y,
        Valign::Center => p0.y + (box_size.y - size.y) / 2,
    };
    IRange2::sized(IVec2::new(x0, y0), size)
}

fn monospace_tex_bounds(char_rect: IRange2, map_size: IVec2, tex_bounds: &Range2) -> Range2 {
    let scale = Vec2::ONE / map_size.as_vec2();
    let t0 = char_rect.min.as_vec2() * scale;
    let t1 = char_rect.max.as_vec2() * scale;
    let p0 = tex_bounds.lerp(t0);
    let p1 = tex_bounds.lerp(t1);
    Range2::new(p0, p1)
}

pub struct Font {
    height: i32,
    chars: Vec<FontCharInfo>,
    widths: Vec<i32>,
    spacing: IVec2,
}

impl Font {
    pub fn new(height: i32, spacing: IVec2, chars: Vec<FontCharInfo>) -> Self {
        Self {
            height,
            widths: chars.iter().map(|entry| entry.width).collect(),
            chars,
            spacing,
        }
    }

    // static member FromDescriptor(desc, mapSize, texBounds) =
    //     let table = Array.zeroCreate 256
    //     for ch in desc.Chars do
    //         let code = int ch.Code
    //         if code < table.Length then
    //             let coords = FontCharInfo.fromCharDescriptor mapSize ch texBounds
    //             table.[code] <- coords
    //     Font(desc.Height, table)
    pub fn from_def(def: &FontDef, map_size: UVec2, tex_bounds: &Range2) -> Self {
        let mut chars = vec![FontCharInfo::default(); 256];
        for ch in &def.chars {
            let code = ch.code as usize;
            if code < chars.len() {
                let coords = FontCharInfo::from_char_def(map_size, ch, tex_bounds);
                chars[code] = coords;
            }
        }
        Self::new(def.height, IVec2::new(0, 0), chars)
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn widths(&self) -> &[i32] {
        &self.widths
    }

    pub fn char_info(&self, ch: char) -> &FontCharInfo {
        &self.chars[ch as usize]
    }

    pub fn mono_size(&self) -> IVec2 {
        IVec2::new(self.widths[0], self.height)
    }

    pub fn monospaced(sheet_size: IVec2, tex_bounds: Range2, spacing: IVec2) -> Self {
        // Assume this is a 16x16 char tile sheet
        let tile_extent = 16;
        let char_size = sheet_size / tile_extent;
        let mut lookup = Vec::new();
        for y in 0..tile_extent {
            for x in 0..tile_extent {
                let tp = IVec2::new(x, y);
                let p = tp * char_size;
                let char_rect = IRange2::sized(p, char_size);
                lookup.push(FontCharInfo {
                    width: char_size.x + spacing.x,
                    offset: IVec2::ZERO,
                    // Avoid rendering null/zero char by making it size zero
                    size: if tp == IVec2::ZERO {
                        IVec2::ZERO
                    } else {
                        char_size
                    },
                    rect: monospace_tex_bounds(char_rect, sheet_size, &tex_bounds),
                });
            }
        }
        let height = char_size.y + spacing.y;
        Self::new(height, spacing, lookup)
    }

    pub fn measure_with_limit(&self, str: &[char], max_allowed_width: i32) -> IVec2 {
        let size = measure(str, &self.widths, self.height, max_allowed_width);
        IVec2::new(
            if size.x > 0 {
                size.x - self.spacing.x
            } else {
                0
            },
            if size.y > 0 {
                size.y - self.spacing.y
            } else {
                0
            },
        )
    }
    pub fn measure_block_chars(&self, chars: &[char], block: &TextBlock) -> IRange2 {
        let max_allowed_width = block
            .wrap
            .max_allowed_width(block.bounds.size().x * block.scale);
        let size = self.measure_with_limit(chars, max_allowed_width) * block.scale;
        aligned_bounds(size, block.bounds, block.align, block.valign)
    }

    pub fn measure(&self, str: &[char]) -> IVec2 {
        self.measure_with_limit(str, i32::MAX)
    }

    pub fn measure_block(&self, block: &TextBlock) -> IRange2 {
        self.measure_block_chars(&block.text.chars().collect::<Vec<_>>(), block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_run() {
        assert_eq!(next_run(&['a'; 3], &[10; 256], 0, 100), Some(0..3));
    }
}
