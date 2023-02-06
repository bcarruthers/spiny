#[derive(Clone, Copy)]
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

pub struct AnsiTextBuilder {
    text: String,
}

impl AnsiTextBuilder {
    pub fn push_str(&mut self, s: &str) {
        self.text.push_str(s)
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
        self.begin_seq();
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
        self.end_seq();
    }

    pub fn reset(&mut self) {
        self.text.push_str("\x1b[0m");
    }

    pub fn fg(&mut self, fg: AnsiColor) {
        self.begin_seq();
        self.push_color(fg, 0);
        self.end_seq();
    }

    pub fn bg(&mut self, bg: AnsiColor) {
        self.begin_seq();
        self.push_color(bg, 1);
        self.end_seq();
    }

    pub fn fg_bg(&mut self, fg: AnsiColor, bg: AnsiColor) {
        self.begin_seq();
        self.push_color(fg, 0);
        self.text.push(';');
        self.push_color(bg, 1);
        self.end_seq();
    }
}
