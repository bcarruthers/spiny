use super::text::*;
use glam::IVec2;

#[derive(Clone)]
pub struct Tile {
    pub ch: char,
    pub style: Style,
}

impl Tile {
    pub fn new(ch: char, style: Style) -> Self {
        Self { ch, style }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct TileSpanId(pub u64);

#[derive(Clone)]
pub struct TileSpan {
    pub text: String,
    pub style: Style,
    pub id: TileSpanId,
}

impl TileSpan {
    pub fn new(text: String, style: Style, id: TileSpanId) -> Self {
        Self { text, style, id }
    }

    pub fn from_style(text: String, style: Style) -> Self {
        Self {
            text,
            style,
            id: TileSpanId(0),
        }
    }

    pub fn from_text(text: String) -> Self {
        Self::from_style(text, Style::WHITE)
    }

    pub fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    pub fn with_id(self, id: TileSpanId) -> Self {
        Self { id, ..self }
    }

    pub fn len(&self) -> i32 {
        self.text.len() as i32
    }

    pub fn into_row(self) -> TileRow {
        TileRow::new(vec![self])
    }

    pub fn into_block(self) -> TileBlock {
        self.into_row().into_block()
    }
}

#[derive(Clone)]
pub struct TileRow {
    pub spans: Vec<TileSpan>,
}

impl TileRow {
    pub fn new(spans: Vec<TileSpan>) -> Self {
        Self { spans }
    }

    pub fn len(&self) -> i32 {
        self.spans.iter().map(|span| span.text.len()).sum::<usize>() as i32
    }

    pub fn into_block(self) -> TileBlock {
        TileBlock::new(vec![self])
    }
}

#[derive(Default, Clone)]
pub struct TileBlock {
    pub rows: Vec<TileRow>,
}

impl TileBlock {
    pub fn new(rows: Vec<TileRow>) -> Self {
        Self { rows }
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.width(), self.height())
    }

    pub fn width(&self) -> i32 {
        self.rows.iter().map(|row| row.len()).max().unwrap_or(0) as i32
    }

    pub fn height(&self) -> i32 {
        self.rows.len() as i32
    }
}
