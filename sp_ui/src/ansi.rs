use glam::IVec2;
use sp_math::{color::IRgba, range::IRange2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Palette(u8),
    Rgb(u8, u8, u8),
}

impl AnsiColor {
    pub fn from_irgba(c: IRgba) -> Self {
        let r = (c.r as u32 * c.a as u32 / 255) as u8;
        let g = (c.g as u32 * c.a as u32 / 255) as u8;
        let b = (c.b as u32 * c.a as u32 / 255) as u8;
        Self::Rgb(r, g, b)
    }
}

#[derive(Clone, Default)]
pub struct AnsiTextBuilder {
    text: String,
    fg: Option<AnsiColor>,
    bg: Option<AnsiColor>,
}

impl AnsiTextBuilder {
    pub fn push(&mut self, ch: char) {
        self.text.push(ch)
    }

    pub fn push_str(&mut self, s: &str) {
        self.text.push_str(s)
    }

    pub fn into_string(self) -> String {
        self.text
    }

    fn begin_seq(&mut self) {
        self.text.push_str("\x1b[")
    }

    fn end_seq(&mut self) {
        self.text.push('m')
    }

    //     Dark Light
    // FG:   3x    4x
    // BG:   9x   10x
    const CODES: [&'static str; 4 * 8] = [
        "30", "90", "40", "100", "31", "91", "41", "101", "32", "92", "42", "102", "33", "93",
        "43", "103", "34", "94", "44", "104", "35", "95", "45", "105", "36", "96", "46", "106",
        "37", "97", "47", "107",
    ];

    fn push_code(&mut self, i: usize) {
        self.text.push_str(&Self::CODES[i])
    }

    fn push_color(&mut self, color: AnsiColor, bg: usize) {
        match color {
            AnsiColor::Black => self.push_code(0 + bg),
            AnsiColor::Red => self.push_code(4 + bg),
            AnsiColor::Green => self.push_code(8 + bg),
            AnsiColor::Yellow => self.push_code(12 + bg),
            AnsiColor::Blue => self.push_code(16 + bg),
            AnsiColor::Magenta => self.push_code(20 + bg),
            AnsiColor::Cyan => self.push_code(24 + bg),
            AnsiColor::White => self.push_code(28 + bg),
            AnsiColor::BrightBlack => self.push_code(2 + bg),
            AnsiColor::BrightRed => self.push_code(6 + bg),
            AnsiColor::BrightGreen => self.push_code(10 + bg),
            AnsiColor::BrightYellow => self.push_code(14 + bg),
            AnsiColor::BrightBlue => self.push_code(18 + bg),
            AnsiColor::BrightMagenta => self.push_code(22 + bg),
            AnsiColor::BrightCyan => self.push_code(26 + bg),
            AnsiColor::BrightWhite => self.push_code(30 + bg),
            AnsiColor::Palette(i) => {
                let code = if bg == 0 { "38;5;" } else { "48;5;" };
                self.text.push_str(&format!("{}{}", code, i));
            }
            AnsiColor::Rgb(r, g, b) => {
                let code = if bg == 0 { "38;2;" } else { "48;2;" };
                self.text.push_str(&format!("{}{};{};{}", code, r, g, b))
            }
        }
    }

    pub fn reset(&mut self) {
        if self.fg.is_some() || self.bg.is_some() {
            self.fg = None;
            self.bg = None;
            self.text.push_str("\x1b[0m");
        }
    }

    pub fn fg(&mut self, fg: AnsiColor) {
        if self.fg != Some(fg) {
            self.fg = Some(fg);
            self.begin_seq();
            self.push_color(fg, 0);
            self.end_seq();
        }
    }

    pub fn bg(&mut self, bg: AnsiColor) {
        if self.bg != Some(bg) {
            self.bg = Some(bg);
            self.begin_seq();
            self.push_color(bg, 1);
            self.end_seq();
        }
    }

    pub fn fg_bg(&mut self, fg: AnsiColor, bg: AnsiColor) {
        if self.fg != Some(fg) || self.bg != Some(bg) {
            self.fg = Some(fg);
            self.bg = Some(bg);
            self.begin_seq();
            self.push_color(fg, 0);
            self.text.push(';');
            self.push_color(bg, 1);
            self.end_seq();
        }
    }

    pub fn set_style(&mut self, fg: Option<AnsiColor>, bg: Option<AnsiColor>) {
        if self.fg != fg || self.bg != bg {
            match (fg, bg) {
                (Some(fg), Some(bg)) => {
                    self.fg_bg(fg, bg);
                }
                (Some(fg), None) => {
                    self.reset();
                    self.fg(fg);
                }
                (None, Some(bg)) => {
                    self.reset();
                    self.bg(bg);
                }
                (None, None) => self.reset(),
            }
        }
    }
}

fn index_of(range: IRange2, stride: usize, p: IVec2) -> usize {
    let rp = p - range.min;
    rp.y as usize * stride + rp.x as usize
}

pub fn format_grid_half_char<T: Into<IRgba> + Copy>(
    cells: &[T],
    range: IRange2,
    stride: usize,
) -> String {
    let y_range = range.y_range();
    let y_size = y_range.size();
    let rows = (y_size + 1) / 2;
    // Top bar
    //let ch = char::from_u32(0x2580).unwrap();
    // Bottom bar: Use this if +Y is up since we may hide the first row for odd size
    let ch = char::from_u32(0x2584).unwrap();
    let mut s = AnsiTextBuilder::default();
    for r in 0..rows {
        // Reverse rows so y or z increases up
        let yr = rows - 1 - r;
        for x in range.x() {
            let c1 = {
                let y = y_range.min + yr * 2 + 1;
                if y < y_range.max {
                    let p = IVec2::new(x, y);
                    let i = index_of(range, stride, p);
                    Some(AnsiColor::from_irgba(cells[i].into()))
                } else {
                    None
                }
            };
            let c2 = {
                let y = y_range.min + yr * 2 + 0;
                if y < y_range.max {
                    let p = IVec2::new(x, y);
                    let i = index_of(range, stride, p);
                    Some(AnsiColor::from_irgba(cells[i].into()))
                } else {
                    None
                }
            };
            s.set_style(c2, c1);
            s.push(ch);
        }
        // Reset before newline to avoid stretching color
        s.reset();
        if r < rows - 1 {
            s.push('\n');
        }
    }
    s.into_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        println!("\033[31;1;4mHello\033[0m");
    }

    fn test(b: AnsiTextBuilder) {
        let s = b.into_string();
        println!("{}", s);
        println!("{:?}", s.chars());
    }

    #[test]
    fn test_ansi() {
        let mut b = AnsiTextBuilder::default();
        b.fg(AnsiColor::Red);
        b.push_str("red");
        b.bg(AnsiColor::Blue);
        b.push_str(" blue");
        b.fg(AnsiColor::Green);
        b.push_str(" green");
        b.reset();
        b.push_str(" default");
        b.reset();
        test(b);
    }
}