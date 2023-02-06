use glam::Mat4;
use sp_draw::SpriteVertex;
use sp_math::{color::IRgba, range::Range2};

use crate::{
    binding::{TextureBinding, TransformBinding},
    texture::Texture,
};

pub struct QuadRenderer {
    texture_binding: wgpu::BindGroup,
    camera_binding: TransformBinding,
    vertex_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    tex_bounds: Range2,
}

impl QuadRenderer {
    pub fn new(
        device: &wgpu::Device,
        texture: &Texture,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        multisample_count: u32,
        tex_bounds: Range2,
    ) -> Self {
        let texture_binding = TextureBinding::new(&device, &texture);
        let camera_binding = TransformBinding::new(&device, Mat4::IDENTITY);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("quad_pipeline_layout"),
                bind_group_layouts: &[&texture_binding.layout, &camera_binding.layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("quad_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[super::sprite::vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
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
                count: multisample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("quad_vertex_buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: std::mem::size_of::<SpriteVertex>() as u64 * 6,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            texture_binding: texture_binding.group,
            camera_binding,
            tex_bounds,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, color: IRgba) {
        let color = [color, IRgba::ZERO, IRgba::ZERO].map(|c| c.to_array());
        let quad_vertices: [SpriteVertex; 4] = [
            SpriteVertex {
                pos: [-1.0, -1.0, 0.0],
                tex_coords: self.tex_bounds.x0y0().to_array(),
                color,
            },
            SpriteVertex {
                pos: [1.0, -1.0, 0.0],
                tex_coords: self.tex_bounds.x1y0().to_array(),
                color,
            },
            SpriteVertex {
                pos: [1.0, 1.0, 0.0],
                tex_coords: self.tex_bounds.x1y1().to_array(),
                color,
            },
            SpriteVertex {
                pos: [-1.0, 1.0, 0.0],
                tex_coords: self.tex_bounds.x0y1().to_array(),
                color,
            },
        ];
        let vertices: [SpriteVertex; 6] = [
            quad_vertices[0],
            quad_vertices[1],
            quad_vertices[2],
            quad_vertices[0],
            quad_vertices[2],
            quad_vertices[3],
        ];
        let byte_count = vertices.len() * std::mem::size_of::<SpriteVertex>();
        let slice = bytemuck::cast_slice(&vertices);
        queue.write_buffer(&self.vertex_buffer, 0, &slice[0..byte_count]);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_binding, &[]);
        render_pass.set_bind_group(1, &self.camera_binding.group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}
