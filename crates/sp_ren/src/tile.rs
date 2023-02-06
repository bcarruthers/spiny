use crate::text::TextRenderer;
use glam::{IVec2, UVec2};
use sp_math::range::IRange2;
use sp_ui::{tile::*, TextSpan};

pub struct TileRenderer<'a> {
    offset: IVec2,
    grid_size: IVec2,
    tile_size: IVec2,
    scale: i32,
    ren: &'a mut TextRenderer,
}

impl<'a> TileRenderer<'a> {
    pub fn new(
        ren: &'a mut TextRenderer,
        window_size: UVec2,
        tile_size: UVec2,
        scale: i32,
    ) -> Self {
        let size_in_pixels = window_size.as_ivec2();
        let tile_size = tile_size.as_ivec2();
        let scaled_tile_size = tile_size * scale;
        let grid_size = size_in_pixels / scaled_tile_size;
        let grid_size_in_pixels = grid_size * scaled_tile_size;
        let gap = size_in_pixels - grid_size_in_pixels;
        // Centered horizontally, anchored to bottom
        let offset = IVec2::new(gap.x as i32 / 2, gap.y as i32);
        Self {
            offset,
            grid_size,
            tile_size,
            scale,
            ren,
        }
    }

    pub fn grid_size(&self) -> IVec2 {
        self.grid_size
    }

    pub fn bounds(&self) -> IRange2 {
        IRange2::new(IVec2::ZERO, self.grid_size)
    }

    pub fn draw_tile(&mut self, pos: IVec2, tile: &Tile) {
        self.ren.draw_char(
            tile.ch,
            pos * self.scale * self.tile_size + self.offset,
            self.scale,
            tile.style,
            0,
        )
    }

    pub fn draw_tile_row(&mut self, pos: IVec2, row: &TileRow) {
        let pos = pos * self.scale * self.tile_size + self.offset;
        let mut x = pos.x;
        for span in row.spans.iter() {
            self.ren.draw_text_span(
                &TextSpan {
                    text: &span.text,
                    pos: IVec2::new(x, pos.y),
                    scale: self.scale,
                    style: span.style,
                },
                0,
            );
            x += span.len() * self.scale * self.tile_size.x;
        }
    }

    pub fn draw_tile_block(&mut self, pos: IVec2, block: &TileBlock) {
        let mut y = pos.y;
        for row in block.rows.iter() {
            self.draw_tile_row(IVec2::new(pos.x, y), row);
            y += 1;
        }
    }
}
