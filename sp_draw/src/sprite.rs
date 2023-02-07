use std::ops::{Deref, DerefMut};

use glam::{Vec2, Mat4, Mat3};
use sp_math::{range::Range2, color::{IRgba, Rgba}};

fn to_pos3(pos: Vec2) -> [f32; 3] {
    [pos.x, pos.y, 0.0]
}

#[derive(Default, Clone)]
pub struct SpriteStats {
    pub sprites: usize,
    pub batches: usize,
    pub vertices: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpriteBlend {
    Alpha = 0,
    Add = 1,
}

impl Default for SpriteBlend {
    fn default() -> Self {
        Self::Alpha
    }
}

impl SpriteBlend {
    pub const BLENDS: [Self; 2] = [Self::Alpha, Self::Add];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpritePrimitive {
    Triangle,
    Quad,
}

impl SpritePrimitive {
    pub const fn vertex_count(&self) -> usize {
        match self {
            Self::Triangle => 3,
            Self::Quad => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpriteBatchDescriptor {
    pub camera: u32,
    pub depth: i32,
    pub blend: SpriteBlend,
    pub primitive: SpritePrimitive,
}

#[derive(Debug, Copy, Clone)]
pub struct SpriteBatch {
    pub desc: SpriteBatchDescriptor,
    pub vertex_start: u32,
    pub vertex_count: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub pos: Vec2,
    pub tex_bounds: Range2,
    pub size: Vec2,
    pub color: [IRgba; 3],
    pub rot: Vec2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    pub pos: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [[u8; 4]; 3],
}

#[derive(Default)]
pub struct SpriteBatches {
    pub vertices: Vec<SpriteVertex>,
    pub batches: Vec<SpriteBatch>,
}

impl SpriteBatches {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            batches: Vec::new(),
        }
    }

    pub fn stats(&self) -> SpriteStats {
        let mut stats = SpriteStats::default();
        for batch in &self.batches {
            stats.batches += 1;
            stats.vertices += batch.vertex_count as usize;
            stats.sprites += batch.vertex_count as usize / batch.desc.primitive.vertex_count() as usize
        }
        stats
    }

    pub fn max_quad_count(&self) -> u32 {
        self.batches
        .iter()
        .filter_map(|b| 
            if b.desc.primitive == SpritePrimitive::Quad {
                Some(b.vertex_count / 4)
            } else {
                None
            }
        )
        .max()
        .unwrap_or(0)
    }

    pub fn count(&self) -> u32 {
        self.batches
        .iter()
        .map(|b| 
            b.desc.primitive.vertex_count() as u32 * b.vertex_count)
        .sum()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.batches.clear();
    }

    fn end(&mut self) {
        if !self.batches.is_empty() {
            let last = self.batches.len() - 1;
            let batch = &mut self.batches[last];
            batch.vertex_count = self.vertices.len() as u32 - batch.vertex_start;
            // Remove empty batches
            if batch.vertex_count == 0 {
                self.batches.pop();
            }
        }
    }

    pub fn begin(&mut self, desc: SpriteBatchDescriptor) {
        // Avoid adding contiguous duplicate state batches
        if self.batches.is_empty() ||
            self.batches[self.batches.len() - 1].desc != desc {
            self.end();
            let batch = SpriteBatch {
                desc,
                vertex_start: self.vertices.len() as u32,
                vertex_count: 0,
            };
            self.batches.push(batch);
        }
    }

    pub fn finalize(&mut self) {
        self.end();
        self.batches.sort_unstable_by_key(|b| b.desc);
    }
}

pub struct Camera {
    pub proj_view: Mat4,
    pub color: Rgba,
}

#[derive(Default)]
pub struct DrawOutput {
    pub clear_color: Rgba,
    pub cameras: Vec<Camera>,
    pub sprites: SpriteBatches,
}

#[derive(Copy, Clone, Debug)]
pub struct QuadSprite {
    pub pos: Vec2,
    pub tex_bounds: Range2,
    pub size: Vec2,
    pub color: [IRgba; 3],
    pub rot: Vec2,
}

impl Default for QuadSprite {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            tex_bounds: Range2::ZERO_TO_ONE,
            size: Vec2::ONE,
            color: [IRgba::WHITE; 3],
            rot: Vec2::X
        }
    }
}

pub struct TriWriter<'a> {
    xf: Mat3,
    verts: &'a mut Vec<SpriteVertex>
}

impl<'a> TriWriter<'a> {
    pub fn draw_tri(
        &mut self, 
        pos: [Vec2; 3],
        tex_coords: [Vec2; 3],
        color: [IRgba; 3],
    ) {
        let pos = pos.map(|p| to_pos3(self.xf.transform_point2(p)));
        let color = color.map(|c| c.to_array());
        let tex_coords = tex_coords.map(|p| p.to_array());
        self.verts.push(SpriteVertex {
            pos: pos[0],
            tex_coords: tex_coords[0],
            color,
        });
        self.verts.push(SpriteVertex {
            pos: pos[1],
            tex_coords: tex_coords[1],
            color,
        });
        self.verts.push(SpriteVertex {
            pos: pos[2],
            tex_coords: tex_coords[2],
            color,
        });
    }
}

pub struct QuadWriter<'a> {
    xf: Mat3,
    verts: &'a mut Vec<SpriteVertex>
}

impl<'a> QuadWriter<'a> {
    pub fn draw_quad_pos(
        &mut self,
        pos: [Vec2; 4],
        tb: &Range2,
        color: [IRgba; 3],
    ) {
        let pos = pos.map(|p| to_pos3(self.xf.transform_point2(p)));
        let color = color.map(|c| c.to_array());
        self.verts.push(SpriteVertex {
            pos: pos[0],
            tex_coords: tb.x0y0().to_array(),
            color,
        });
        self.verts.push(SpriteVertex {
            pos: pos[1],
            tex_coords: tb.x1y0().to_array(),
            color,
        });
        self.verts.push(SpriteVertex {
            pos: pos[2],
            tex_coords: tb.x1y1().to_array(),
            color,
        });
        self.verts.push(SpriteVertex {
            pos: pos[3],
            tex_coords: tb.x0y1().to_array(),
            color,
        });
    }

    pub fn draw_quad(
        &mut self,
        rect: Range2,
        tb: &Range2,
        color: [IRgba; 3],
    ) {
        self.draw_quad_pos(
            [
                rect.x0y0(),
                rect.x1y0(),
                rect.x1y1(),
                rect.x0y1(),
            ],
            tb,
            color);
    }

    pub fn draw_sprite_perp(&mut self, sprite: &QuadSprite) {
        let center = sprite.pos;
        let size = sprite.size * 0.5;
        let dx = sprite.rot * size.x;
        let dy = sprite.rot.perp() * size.y;
        self.draw_quad_pos(
            [
                center - dx + dy,
                center - dx - dy,
                center + dx - dy,
                center + dx + dy
            ],
            &sprite.tex_bounds,
            sprite.color,
        );
    }

    pub fn draw_sprite(&mut self, sprite: &QuadSprite) {
        let center = sprite.pos;
        let size = sprite.size * 0.5;
        let dx = sprite.rot * size.x;
        let dy = sprite.rot.perp() * size.y;
        self.draw_quad_pos(
            [
                center - dx - dy,
                center + dx - dy,
                center + dx + dy,
                center - dx + dy
            ],
            &sprite.tex_bounds,
            sprite.color,
        );
    }
    
    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, thickness: f32, tb: Range2, color: [IRgba; 3]) {
        let delta = p1 - p0;
        let length = delta.length();
        let dir = if length < 1e-5 { Vec2::ZERO } else { delta / length };
        self.draw_sprite_perp(&QuadSprite {
            pos: (p0 + p1) * 0.5,
            tex_bounds: tb,
            size: Vec2::new(thickness, length),
            color,
            rot: dir.perp()
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpriteBatchState {
    pub camera: u32,
    pub depth: i32,
    pub blend: SpriteBlend,
    pub transform: Mat3,
}

impl Default for SpriteBatchState {
    fn default() -> Self {
        Self {
            camera: 0,
            depth: 0,
            blend: SpriteBlend::default(),
            transform: Mat3::IDENTITY,
        }
    }
}

impl SpriteBatchState {
    pub fn to_descriptor(&self, primitive: SpritePrimitive) -> SpriteBatchDescriptor {
        SpriteBatchDescriptor {
            camera: self.camera,
            depth: self.depth,
            blend: self.blend,
            primitive,
        }
    }
}

pub struct SpriteBatchScope<'a> {
    builder: &'a mut SpriteBatchBuilder,
}

impl<'a> Drop for SpriteBatchScope<'a> {
    fn drop(&mut self) {
        self.builder.pop()
    }
}

impl<'a> Deref for SpriteBatchScope<'a> {
    type Target = SpriteBatchBuilder;
    fn deref(&self) -> &SpriteBatchBuilder {
        self.builder
    }
}

impl<'a> DerefMut for SpriteBatchScope<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.builder
    }
}

#[derive(Default)]
pub struct SpriteBatchBuilder {
    states: Vec<SpriteBatchState>,
    data: SpriteBatches,
}

impl SpriteBatchBuilder {
    pub fn new() -> Self {
        Self {
            states: vec![Default::default()],
            data: SpriteBatches::new(),
        }
    }

    pub fn state(&self) -> &SpriteBatchState {
        &self.states[self.states.len() - 1]
    }

    fn begin(&mut self, primitive: SpritePrimitive) {
        let state = self.state();
        let desc = state.to_descriptor(primitive);
        self.data.begin(desc);
    }

    fn push(&mut self, state: SpriteBatchState) -> SpriteBatchScope<'_> {
        self.states.push(state);
        SpriteBatchScope { builder: self }
    }

    fn pop(&mut self) {
        self.states.pop();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.states.clear();
        self.states.push(Default::default());
    }

    pub fn push_depth(&mut self, depth: i32) -> SpriteBatchScope<'_> {
        self.push(SpriteBatchState {
            depth,
            ..*self.state()
        })
    }

    pub fn push_depth_delta(&mut self, depth: i32) -> SpriteBatchScope<'_> {
        let prior = *self.state();
        self.push(SpriteBatchState {
            depth: depth + prior.depth,
            ..prior
        })
    }

    pub fn push_camera(&mut self, camera: u32) -> SpriteBatchScope<'_> {
        let prior = *self.state();
        self.push(SpriteBatchState {
           camera,
            ..prior
        })
    }

    pub fn push_blend(&mut self, blend: SpriteBlend) -> SpriteBatchScope<'_> {
        let prior = *self.state();
        self.push(SpriteBatchState {
            blend,
            ..prior
        })
    }

    pub fn push_transform(&mut self, xf: Mat3) -> SpriteBatchScope<'_> {
        let prior = *self.state();
        self.push(SpriteBatchState {
            transform: prior.transform * xf,
            ..prior
        })
    }

    pub fn push_pos(&mut self, pos: Vec2) -> SpriteBatchScope<'_> {
        self.push_transform(Mat3::from_translation(pos))
    }

    pub fn push_scale(&mut self, scale: Vec2) -> SpriteBatchScope<'_> {
        self.push_transform(Mat3::from_scale(scale))
    }

    pub fn begin_tris(&mut self) -> TriWriter {
        self.begin(SpritePrimitive::Triangle);
        TriWriter {
            xf: self.state().transform,
            verts: &mut self.data.vertices
        }
    }

    pub fn begin_quads(&mut self) -> QuadWriter {
        self.begin(SpritePrimitive::Quad);
        QuadWriter {
            xf: self.state().transform,
            verts: &mut self.data.vertices
        }
    }

    pub fn write_batches(&mut self, batches: &SpriteBatches) {
        let state = self.state().clone();
        for batch in batches.batches.iter() {
            let desc = SpriteBatchDescriptor {
                depth: batch.desc.depth + state.depth,
                ..batch.desc
            };
            self.data.begin(desc);
            let start = batch.vertex_start as usize;
            let end = start + batch.vertex_count as usize;
            let verts = &batches.vertices[start..end];
            for v in verts.iter() {
                let pos = Vec2::new(v.pos[0], v.pos[1]);
                let pos = to_pos3(state.transform.transform_point2(pos));
                self.data.vertices.push(SpriteVertex {
                    pos,
                    tex_coords: v.tex_coords,
                    color: v.color,
                });
            }
        }
    }

    pub fn finalize(&mut self) -> &SpriteBatches {
        self.data.finalize();
        &self.data
    }

    pub fn build(mut self) -> SpriteBatches {
        self.finalize();
        self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn draw_quads() {
        // Note we intentionally can't begin more than one batch at a time
        let mut b = SpriteBatchBuilder::new();
        let mut w = b.begin_quads();
        //let mut q2 = b.begin_quads();
        w.draw_sprite(&QuadSprite::default());
        let v = b.build();
        assert_eq!(v.batches.len(), 1);
        assert_eq!(v.vertices.len(), 4);
    }
}