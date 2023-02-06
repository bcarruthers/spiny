use crate::{
    binding::{TextureBinding, TransformBinding},
    buffer::QuadBuffer,
    Texture,
};
use glam::Mat4;
use sp_draw::*;

const INITIAL_VERTICES: u64 = 4096;
const INITIAL_QUADS: u64 = 512;

pub mod vertex {
    use super::*;

    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Unorm8x4,
        3 => Unorm8x4,
        4 => Unorm8x4,
        5 => Unorm8x4,
        ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    blend: SpriteBlend,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    let blend =
        match blend {
            SpriteBlend::Alpha => wgpu::BlendState::ALPHA_BLENDING,
            SpriteBlend::Add => wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            },
        };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("sprite_pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,//Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: sample_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}

fn create_vertex_buffer(device: &wgpu::Device, vertex_count: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sprite_vertex_buffer"),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size: vertex_count * std::mem::size_of::<SpriteVertex>() as u64,
        mapped_at_creation: false,
    })
}

pub struct SpriteRenderer {
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: u64,
    texture_binding: wgpu::BindGroup,
    camera_bindings: Vec<TransformBinding>,
    quads: QuadBuffer,
    batches: SpriteBatches,
    pipelines: [wgpu::RenderPipeline; 2],
}

impl SpriteRenderer {
    pub fn new(
        device: &wgpu::Device,
        texture: &Texture,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        sample_count: u32,
        camera_count: usize,
    ) -> Self {
        let vertex_capacity = INITIAL_VERTICES;
        let vertex_buffer = create_vertex_buffer(device, vertex_capacity);
        let texture_binding = TextureBinding::new(&device, &texture);

        let camera_bindings = (0..camera_count)
            .map(|_| TransformBinding::new(&device, Mat4::IDENTITY))
            .collect();

        let camera_binding = TransformBinding::new(&device, Mat4::IDENTITY);

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sprite_pipeline_layout"),
                bind_group_layouts: &[&texture_binding.layout, &camera_binding.layout],
                push_constant_ranges: &[],
            });

        let pipelines = SpriteBlend::BLENDS.map(|blend| create_pipeline(
            device,
            format,
            shader,
            &layout,
            blend,
            sample_count));

        let quads = QuadBuffer::new(device, INITIAL_QUADS as u32);
        
        Self {
            vertex_capacity,
            vertex_buffer,
            camera_bindings,
            pipelines,
            batches: SpriteBatches::new(),
            texture_binding: texture_binding.group,
            quads,
        }
    }

    pub fn count(&self) -> u32 {
        self.batches.count()
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        batches: SpriteBatches,
        view_proj: &[Mat4]
    ) {
        for i in 0..view_proj.len() {
            self.camera_bindings[i].update(&queue, view_proj[i]);
        }

        if batches.vertices.len() as u64 > self.vertex_capacity {
            self.vertex_capacity =  batches.vertices.len().next_power_of_two() as u64;
            self.vertex_buffer = create_vertex_buffer(device, self.vertex_capacity);
        }
        let byte_count = batches.vertices.len() * std::mem::size_of::<SpriteVertex>();
        let slice = bytemuck::cast_slice(&batches.vertices);
        queue.write_buffer(&self.vertex_buffer, 0, &slice[0..byte_count]);

        self.quads.update(device, batches.max_quad_count());
        self.batches = batches;
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // Note GL ES does not support non-zero base vertex
        let use_base_vertex = false;
        let vert_size = std::mem::size_of::<SpriteVertex>() as u64;
        render_pass.set_bind_group(0, &self.texture_binding, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quads.slice(), wgpu::IndexFormat::Uint32);
        for batch in self.batches.batches.iter() {
            let start = batch.vertex_start;
            let end = start + batch.vertex_count;
            if !use_base_vertex {
                let start = start as u64 * vert_size;
                let end = end as u64 * vert_size;
                let bounds = start..end;
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(bounds));
            }
            let camera_group = &self.camera_bindings[batch.desc.camera as usize].group;
            render_pass.set_pipeline(&self.pipelines[batch.desc.blend as usize]);
            render_pass.set_bind_group(1, camera_group, &[]);
            match batch.desc.primitive {
                SpritePrimitive::Triangle => {
                    let bounds = if use_base_vertex { start..end } else { 0..batch.vertex_count };
                    render_pass.draw(bounds, 0..1);
                }
                SpritePrimitive::Quad => {
                    let base_vertex = if use_base_vertex { start as i32 } else { 0 };
                    let quad_count = batch.vertex_count / 4;
                    render_pass.draw_indexed(0..quad_count * 6, base_vertex, 0..1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_vertex_size() {
        assert_eq!(std::mem::size_of::<SpriteVertex>(), 32);
    }
}
