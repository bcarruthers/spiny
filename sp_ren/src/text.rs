use std::ops::Range;

use crate::{
    binding::{TextureBinding, TransformBinding},
    buffer::QuadBuffer,
    texture::Texture,
};
use glam::{IVec2, Mat4, UVec2};
use sp_math::range::{IRange2, Range2};
use sp_ui::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub fg: [u8; 4],
    pub bg: [u8; 4],
}

impl TextVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
            2 => Unorm8x4,
            3 => Unorm8x4];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn draw_quad(verts: &mut Vec<TextVertex>, r: Range2, tb: &Range2, style: Style) {
        let fg = style.fg.to_array();
        let bg = style.bg.to_array();
        verts.push(TextVertex {
            position: [r.min.x, r.min.y, 0.0],
            tex_coords: [tb.min.x, tb.min.y],
            fg,
            bg,
        });
        verts.push(TextVertex {
            position: [r.max.x, r.min.y, 0.0],
            tex_coords: [tb.max.x, tb.min.y],
            fg,
            bg,
        });
        verts.push(TextVertex {
            position: [r.max.x, r.max.y, 0.0],
            tex_coords: [tb.max.x, tb.max.y],
            fg,
            bg,
        });
        verts.push(TextVertex {
            position: [r.min.x, r.max.y, 0.0],
            tex_coords: [tb.min.x, tb.max.y],
            fg,
            bg,
        });
    }

    pub fn draw_char(
        verts: &mut Vec<TextVertex>,
        desc: &FontCharInfo,
        pos: IVec2,
        scale: i32,
        style: Style,
    ) {
        if desc.size.x > 0 {
            let r = {
                let offset = desc.offset * scale as i32;
                let p0 = pos + offset;
                let p1 = p0 + desc.size * scale as i32;
                IRange2::new(p0, p1)
            }
            .as_range2();
            Self::draw_quad(verts, r, &desc.rect, style);
        }
    }

    pub fn draw_chars<I: Iterator<Item = char>>(
        verts: &mut Vec<TextVertex>,
        font: &Font,
        chars: I,
        pos: IVec2,
        scale: i32,
        style: Style,
    ) {
        let mut x = pos.x;
        for ch in chars {
            let desc = font.char_info(ch);
            let pos = IVec2::new(x, pos.y);
            Self::draw_char(verts, desc, pos, scale, style);
            x += desc.width as i32 * scale as i32;
        }
    }

    pub fn draw_text_block(verts: &mut Vec<TextVertex>, font: &Font, block: &TextBlock) {
        let chars = block.text.chars().collect::<Vec<_>>();
        let text_bounds = font.measure_block_chars(&block.text.chars().collect::<Vec<_>>(), block);
        verts.reserve(block.text.len() * 4);
        let max_unscaled_width = match block.wrap {
            TextWrap::NoWrap => i32::MAX,
            TextWrap::WordWrap => block.bounds.size().x / block.scale,
        };
        let mut run_opt = next_run(&chars, font.widths(), 0, max_unscaled_width);
        let mut row = 0;
        while let Some(run) = run_opt {
            let end = run.end;
            let x = text_bounds.min.x;
            let y = text_bounds.min.y + (row * font.height() * block.scale);
            let pos = IVec2::new(x, y);
            let char_run = &chars[run];
            Self::draw_chars(
                verts,
                font,
                char_run.iter().cloned(),
                pos,
                block.scale,
                block.style,
            );
            run_opt = next_run(&chars, font.widths(), end, max_unscaled_width);
            row += 1;
        }
    }
}

struct SpriteLayer {
    vertex_buffer: wgpu::Buffer,
    verts: Vec<TextVertex>,
    index_count: u32,
}

impl SpriteLayer {
    pub fn new(device: &wgpu::Device, quad_count: u32) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text_vertex_buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: quad_count as u64 * std::mem::size_of::<TextVertex>() as u64 * 4,
            mapped_at_creation: false,
        });
        Self {
            vertex_buffer,
            verts: Vec::new(),
            index_count: 0,
        }
    }

    pub fn index_range(&self) -> Range<u32> {
        0..self.index_count
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.index_count = self.verts.len() as u32 / 4 * 6;
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.verts));
        self.verts.clear();
    }
}

pub struct TextRenderer {
    font: Font,
    texture_binding: wgpu::BindGroup,
    camera_binding: TransformBinding,
    quads: QuadBuffer,
    pipeline: wgpu::RenderPipeline,
    layers: Vec<SpriteLayer>,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        font: Font,
        texture: &Texture,
        format: wgpu::TextureFormat,
        shader: wgpu::ShaderModule,
    ) -> Self {
        let texture_binding = TextureBinding::new(&device, texture);
        let camera_binding = TransformBinding::new(&device, Mat4::IDENTITY);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("text_pipeline_layout"),
                bind_group_layouts: &[&texture_binding.layout, &camera_binding.layout],
                push_constant_ranges: &[],
            });

        log::trace!("Creating text pipeline");
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let layer_count = 8;
        let quad_count = 4096;
        let quads = QuadBuffer::new(device, quad_count);

        Self {
            font,
            camera_binding,
            pipeline,
            quads,
            texture_binding: texture_binding.group,
            layers: (0..layer_count)
                .map(|_| SpriteLayer::new(device, quad_count))
                .collect(),
        }
    }

    pub fn draw_char(&mut self, ch: char, pos: IVec2, scale: i32, style: Style, layer_id: u32) {
        let desc = self.font.char_info(ch);
        let verts = &mut self.layers[layer_id as usize].verts;
        TextVertex::draw_char(verts, desc, pos, scale, style);
    }

    pub fn draw_text_span(&mut self, span: &TextSpan, layer_id: u32) {
        let verts = &mut self.layers[layer_id as usize].verts;
        TextVertex::draw_chars(
            verts,
            &self.font,
            span.text.chars().into_iter(),
            span.pos,
            span.scale,
            span.style,
        );
    }

    pub fn draw_text_block(&mut self, block: &TextBlock) {
        let verts = &mut self.layers[block.layer_id as usize].verts;
        TextVertex::draw_text_block(verts, &self.font, block);
    }

    pub fn draw_quad(&mut self, rect: IRange2, tex_rect: &Range2, style: Style, layer_id: u32) {
        let verts = &mut self.layers[layer_id as usize].verts;
        TextVertex::draw_quad(verts, rect.as_range2(), tex_rect, style);
    }

    pub fn update(&mut self, queue: &wgpu::Queue, size: UVec2) {
        // Update camera
        let proj = Mat4::orthographic_lh(0.0, size.x as f32, size.y as f32, 0.0, -100.0, 100.0);
        self.camera_binding.update(&queue, proj);
        // Update verts
        //let quad_count = self.verts.len() as u32 / 4;
        //self.quads.update(quad_count);
        for layer in self.layers.iter_mut() {
            layer.update(queue);
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_binding, &[]);
        render_pass.set_bind_group(1, &self.camera_binding.group, &[]);
        render_pass.set_index_buffer(self.quads.slice(), wgpu::IndexFormat::Uint32);
        for layer in self.layers.iter() {
            render_pass.set_vertex_buffer(0, layer.vertex_buffer.slice(..));
            render_pass.draw_indexed(layer.index_range(), 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use sp_math::range::Range2;

    use super::*;

    #[test]
    fn draw_block_single_line() {
        let font = Font::monospaced(IVec2::new(128, 256), Range2::ZERO_TO_ONE, IVec2::new(0, 0));
        let mut verts = Vec::new();
        TextVertex::draw_text_block(
            &mut verts,
            &font,
            &TextBlock {
                text: "ABC".to_string(),
                ..Default::default()
            },
        );
        assert_eq!(verts.len(), 12);
        assert_eq!(verts[10].position[0], 24.0);
    }

    #[test]
    fn draw_block_alignments() {
        let font = Font::monospaced(IVec2::new(128, 256), Range2::ZERO_TO_ONE, IVec2::new(0, 0));
        for align in [Align::Left, Align::Center, Align::Right] {
            for valign in [Valign::Top, Valign::Center, Valign::Bottom] {
                let mut verts = Vec::new();
                TextVertex::draw_text_block(
                    &mut verts,
                    &font,
                    &TextBlock {
                        text: "ABC\n123456".to_string(),
                        align,
                        valign,
                        scale: 2,
                        bounds: IRange2::new(IVec2::ZERO, IVec2::new(200, 100)),
                        ..Default::default()
                    },
                );
                assert_eq!(verts.len(), 40);
            }
        }
    }
}
