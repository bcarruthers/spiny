use super::cmd::*;
use glam::{IVec2, UVec2};
use sp_math::range::IRange2;

#[derive(Clone, Default)]
pub struct UiTextGrid {
    pub size: UVec2,
    pub chars: Vec<char>,
    pub overlay: bool,
}

impl UiTextGrid {
    pub fn filled(size: UVec2, ch: char) -> Self {
        Self {
            size,
            chars: vec![ch; (size.x * size.y) as usize],
            ..Default::default()
        }
    }

    pub fn from_str(s: &str) -> Self {
        let rows = s
            .split('\n')
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = UVec2::new(
            rows.iter().map(|row| row.len()).max().unwrap_or(0) as u32,
            rows.len() as u32,
        );
        let mut grid = Self::filled(size, ' ');
        for y in 0..size.y {
            let row = &rows[y as usize];
            for x in 0..row.len() {
                grid.draw_char(IVec2::new(x as i32, y as i32), row[x as usize])
            }
        }
        grid
    }

    pub fn range(&self) -> IRange2 {
        IRange2::new(IVec2::ZERO, self.size.as_ivec2())
    }

    pub fn cell(&self, pos: IVec2) -> char {
        let i = pos.y * self.size.x as i32 + pos.x;
        self.chars[i as usize]
    }

    pub fn draw_str(&mut self, pos: IVec2, s: &str) {
        let mut i = pos.y * self.size.x as i32 + pos.x;
        for ch in s.chars() {
            self.chars[i as usize] = ch;
            i += 1;
        }
    }

    pub fn draw_cutout_str_to(&mut self, add_grid: &mut UiTextGrid, pos: IVec2, text: &str) {
        self.draw_rect(IRange2::sized(pos, IVec2::new(text.len() as i32, 1)), '\0');
        add_grid.draw_str(pos, text);
    }

    pub fn draw_char(&mut self, pos: IVec2, ch: char) {
        if self.range().contains(pos) {
            let i = pos.y * self.size.x as i32 + pos.x;
            self.chars[i as usize] = ch;
        }
    }

    pub fn draw_char_xy(&mut self, x: i32, y: i32, ch: char) {
        self.draw_char(IVec2::new(x, y), ch);
    }

    pub fn draw_corners(&mut self, range: IRange2, style: BorderStyle) {
        let r = IRange2 {
            max: range.max - 1,
            ..range
        };
        self.draw_char(
            r.x0y0(),
            style.get_char(BorderAdjacency::XMAX | BorderAdjacency::YMAX),
        );
        self.draw_char(
            r.x1y0(),
            style.get_char(BorderAdjacency::XMIN | BorderAdjacency::YMAX),
        );
        self.draw_char(
            r.x0y1(),
            style.get_char(BorderAdjacency::XMAX | BorderAdjacency::YMIN),
        );
        self.draw_char(
            r.x1y1(),
            style.get_char(BorderAdjacency::XMIN | BorderAdjacency::YMIN),
        );
    }

    pub fn draw_corners_sized(&mut self, range: IRange2, style: BorderStyle, corner_size: IVec2) {
        self.draw_corners(range, style);
        let r = IRange2 {
            max: range.max - 1,
            ..range
        };
        let ch = style.get_char(BorderAdjacency::XMIN | BorderAdjacency::XMAX);
        // Left horizontal
        for x in r.x0() + 1..r.x0() + corner_size.x {
            self.draw_char(IVec2::new(x, r.y0()), ch);
            self.draw_char(IVec2::new(x, r.y1()), ch);
        }
        // Right horizontal
        for x in r.x1() - corner_size.x + 1..r.x1() {
            self.draw_char(IVec2::new(x, r.y0()), ch);
            self.draw_char(IVec2::new(x, r.y1()), ch);
        }
        let ch = style.get_char(BorderAdjacency::YMIN | BorderAdjacency::YMAX);
        // Top vertical
        for y in r.y0() + 1..r.y0() + corner_size.y {
            self.draw_char(IVec2::new(r.x0(), y), ch);
            self.draw_char(IVec2::new(r.x1(), y), ch);
        }
        // Bottom vertical
        for y in r.y1() - corner_size.y + 1..r.y1() {
            self.draw_char(IVec2::new(r.x0(), y), ch);
            self.draw_char(IVec2::new(r.x1(), y), ch);
        }
    }

    pub fn draw_border(&mut self, range: IRange2, style: BorderStyle) {
        let corner_size = (range.size() + 1) / 2; 
        self.draw_corners_sized(range, style, corner_size)
    }

    pub fn draw_grid_cell_borders(
        &mut self,
        grid_pos: IVec2,
        cell_size: IVec2,
        grid_size_in_cells: IVec2,
        style: BorderStyle,
    ) {
        for y in 0..grid_size_in_cells.y as i32 {
            for x in 0..grid_size_in_cells.x as i32 {
                let cell_pos = IVec2::new(x, y);
                let pos = cell_pos * cell_size + grid_pos;
                let range = IRange2::sized(pos, cell_size + 1);
                self.draw_border(range, style);
            }
        }
    }

    pub fn draw_rect(&mut self, range: IRange2, ch: char) {
        for pos in range.iter() {
            self.draw_char(pos, ch)
        }
    }

    pub fn extract(&self, range: IRange2) -> Self {
        let range = range & self.range();
        let size = range.size().as_uvec2();
        let mut subset = Self::filled(size, ' ');
        for sp in range.iter() {
            let dp = sp - range.min;
            let ch = self.cell(sp);
            subset.draw_char(dp, ch);
        }
        subset
    }

    /// Replace all existing cells of a specific char to connected border chars
    pub fn to_filled_border(&self, ch: char, use_double: bool) -> Self {
        let stride = self.size.x as usize;
        let borders = if use_double {
            &border::DOUBLE
        } else {
            &border::SINGLE
        };
        let mut chars = self.chars.clone();
        let mut i = 0;
        for y in 0..self.size.y as i32 {
            for x in 0..self.size.x as i32 {
                let pos = IVec2::new(x, y);
                let bc = self.cell(pos);
                if bc == ch {
                    let bx0 = x > 0 && self.chars[i - 1] == bc;
                    let by0 = y > 0 && self.chars[i - stride] == bc;
                    let bx1 = x + 1 < self.size.x as i32 && self.chars[i + 1] == bc;
                    let by1 = y + 1 < self.size.y as i32 && self.chars[i + stride] == bc;
                    let i = if bx0 { 1 } else { 0 }
                        | if bx1 { 2 } else { 0 }
                        | if by0 { 4 } else { 0 }
                        | if by1 { 8 } else { 0 };
                    let di = y * self.size.x as i32 + x;
                    chars[di as usize] = borders[i as usize];
                }
                i += 1;
            }
        }
        Self { chars, ..*self }
    }

    pub fn to_rows(&self) -> impl Iterator<Item = String> + '_ {
        (0..self.size.y).map(|y| {
            let start = y * self.size.x;
            let end = start + self.size.x;
            self.chars[start as usize..end as usize]
                .iter()
                .collect::<String>()
        })
    }

    pub fn format(&self) -> String {
        self.to_rows().collect::<Vec<_>>().join("\n")
    }

    pub fn to_ui_text(&self) -> UiText {
        UiText {
            size: self.size,
            content: self.format(),
            overlay: self.overlay,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_borders() {
        let g = UiTextGrid::from_str(concat!("#### ", "# ####", "# #  #", "####"))
            .to_filled_border('#', false);
        println!("{:?}", g.format())
    }
}
