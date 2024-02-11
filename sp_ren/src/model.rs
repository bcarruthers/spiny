use std::ops::Range;

use crate::binding::TextureBinding;
use crate::camera::CameraBinding;
use crate::instance::{Instance, InstanceRaw};
use crate::{texture, TextureAtlas, CameraParams};
use glam::{Mat4, Quat, Vec3};
use indexmap::IndexMap;
use sp_asset::AssetId;
use wgpu::BindGroupLayout;
use wgpu::util::DeviceExt;

pub struct Node {

}

pub enum Interpolation {
    Linear,
    Step,
    CubicSpline,
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Linear
    }
}

pub enum Keyframes {
    Rotation(Vec<Quat>),
    Translation(Vec<Vec3>),
    Scale(Vec<Vec3>),
    Weights(Vec<f32>),
}

pub struct AnimationSampler {
    pub interpolation: Interpolation,
    pub timestamps: Vec<f32>,
    pub keyframes: Keyframes,
}

pub struct AnimationChannel {
    pub sampler: AnimationSampler,
    pub target: usize,
}

pub struct AnimationClip {
    pub duration: f32,
    pub channels: Vec<AnimationChannel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialAlphaMode {
    Opaque,
    Mask,
    Blend,
}

impl Default for MaterialAlphaMode {
    fn default() -> Self {
        MaterialAlphaMode::Opaque
    }
}

pub struct MaterialDescriptor {
    pub double_sided: bool,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_cutoff: Option<f32>,
}

impl Default for MaterialDescriptor {
    fn default() -> Self {
        Self {
            double_sided: false,
            alpha_mode: MaterialAlphaMode::Opaque,
            alpha_cutoff: None,
        }
    }
}

pub struct Material {
    pub emissive_texture: Option<texture::Texture>,
    pub binding: TextureBinding,
    pub descriptor: MaterialDescriptor,
}

pub struct Primitive {
    pub positions: wgpu::Buffer,
    pub normals: wgpu::Buffer,
    pub colors: wgpu::Buffer,
    pub tex_coords: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub index_count: u32,
    pub material: usize,
}

impl Primitive {
    pub fn from_buffers(
        device: &wgpu::Device,
        material: usize,
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        colors: &[[f32; 4]],
        tex_coords: &[[f32; 2]],
        indices: &[u32],
    ) -> Self {
        let index_count = indices.len() as u32;
        let positions = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Vertex Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(positions),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let colors = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Vertex Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(colors),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Vertex Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(normals),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let tex_coords = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Vertex Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(tex_coords),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Index Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            material,
            positions,
            normals,
            colors,
            tex_coords,
            indices,
            index_count,
        }
    }
}

pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub animations: Vec<AnimationClip>,
}

impl Model {
    pub fn from_atlas_quad(device: &wgpu::Device, atlas: &TextureAtlas, asset_id: AssetId) -> Self {
        let tb = atlas.norm_rect(asset_id);
        Self {
            materials: vec![Material {
                emissive_texture: None,
                binding: TextureBinding::new(device, atlas.texture()),
                descriptor: MaterialDescriptor::default(),
            }],
            meshes: vec![Mesh {
                primitives: vec![Primitive::from_buffers(
                    device,
                    0,
                    &[
                        [-1.0, -1.0, 0.0],
                        [1.0, -1.0, 0.0],
                        [1.0, 1.0, 0.0],
                        [-1.0, 1.0, 0.0],
                    ],
                    &[
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                    ],
                    &[
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    &[
                        tb.x0y0().to_array(),
                        tb.x1y0().to_array(),
                        tb.x1y1().to_array(),
                        tb.x0y1().to_array(),
                    ],
                    &[0, 1, 2, 0, 2, 3, 0, 2, 1, 0, 3, 2],
                )],
            }],
            animations: Vec::new(),
        }
    }
}

pub struct ModelCache {
    models: IndexMap<AssetId, Model>,
}

impl ModelCache {
    pub fn new() -> Self {
        ModelCache {
            models: IndexMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.models.clear();
    }

    pub fn add(&mut self, model_id: AssetId, model: Model) {
        self.models.insert(model_id, model);
    }

    pub fn iter_primitives(
        &self,
        model_id: AssetId,
    ) -> impl Iterator<Item = (&Primitive, &Material)> {
        self.models.get(&model_id).into_iter().flat_map(|model| {
            model
                .meshes
                .iter()
                .flat_map(|mesh| mesh.primitives.iter())
                .map(|prim| (prim, &model.materials[prim.material]))
        })
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct RenderPipelineKey {
    alpha_mode: MaterialAlphaMode,
    alpha_cutoff: Option<u32>,
    double_sided: bool,
}

impl RenderPipelineKey {
    fn from_material(material: &Material) -> Self {
        Self {
            alpha_mode: material.descriptor.alpha_mode,
            // Quantize cutoff
            alpha_cutoff: material.descriptor.alpha_cutoff.map(|x| (x * 65535.0) as u32),
            double_sided: material.descriptor.double_sided,
        }
    }
}

#[derive(Default, Clone)]
pub struct ModelInstanceRun {
    pub model_id: AssetId,
    pub cull_cw: bool,
    pub range: Range<u32>,
}

pub struct ModelRenderer {
    format: wgpu::TextureFormat,
    multisample_count: u32,
    models: ModelCache,
    shader: wgpu::ShaderModule,
    //tex_binding: TextureBinding,
    camera_binding: CameraBinding,
    render_pipelines: IndexMap<RenderPipelineKey, wgpu::RenderPipeline>,
    runs: Vec<ModelInstanceRun>,
    instances: Vec<InstanceRaw>,
    #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
}

impl ModelRenderer {
    pub fn new(
        device: &wgpu::Device,
        shader: wgpu::ShaderModule,
        format: wgpu::TextureFormat,
        models: ModelCache,
        multisample_count: u32,
    ) -> Self {
        let camera_binding = CameraBinding::new(device, 1);

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("model_instance_buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: 1024 as u64 * std::mem::size_of::<Instance>() as u64,
            mapped_at_creation: false,
        });

        Self {
            format,
            multisample_count,
            models,
            shader,
            runs: Vec::new(),
            instances: Vec::new(),
            instance_buffer,
            camera_binding,
            render_pipelines: IndexMap::new(),
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        format: wgpu::TextureFormat,
        multisample_count: u32,
        camera_layout: &BindGroupLayout,
        key: RenderPipelineKey,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("model_pipeline_layout"),
                bind_group_layouts: &[
                    &TextureBinding::create_layout(device, wgpu::TextureViewDimension::D2),
                    camera_layout,
                ],
                push_constant_ranges: &[],
            });
        log::trace!("Creating model pipeline {:?}", render_pipeline_layout);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("model_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 1,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 2,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 3,
                        }],
                    },
                    InstanceRaw::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
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
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: if key.double_sided { None } else { Some(wgpu::Face::Back) },
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: multisample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        })
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.runs.clear();
    }

    pub fn push_model_run(&mut self, model_id: AssetId, cull_cw: bool, range: Range<u32>) {
        self.runs.push(ModelInstanceRun {
            model_id,
            cull_cw,
            range,
        });
    }

    pub fn push_instance(&mut self, instance: InstanceRaw) {
        self.instances.push(instance);
    }

    pub fn clear_models(&mut self) {
        self.models.clear();
    }

    pub fn add_model(&mut self, model_id: AssetId, model: Model) {
        self.models.add(model_id, model);
    }

    // pub fn extend_instances_from_slice(&mut self, instances: &[InstanceRaw]) {
    //     self.instances.extend_from_slice(instances);
    // }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: Mat4, proj: Mat4, light_dir: Vec3) {
        self.camera_binding.update(queue, &[
            CameraParams {
                view,
                proj,
                light_dir,
                ..Default::default()
            }
        ]);
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
        // Create material pipelines
        for run in self.runs.iter() {
            for (_primitive, material) in self.models.iter_primitives(run.model_id) {
                let key = RenderPipelineKey::from_material(material);
                self.render_pipelines.entry(key).or_insert_with(|| {
                    Self::create_pipeline(
                        device,
                        &self.shader,
                        self.format,
                        self.multisample_count,
                        self.camera_binding.layout(),
                        key,
                    )
                });
            }
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.camera_binding.bind_group(0), &[]);
        render_pass.set_vertex_buffer(4, self.instance_buffer.slice(..));
        for run in self.runs.iter() {
            for (primitive, material) in self.models.iter_primitives(run.model_id) {
                let key = RenderPipelineKey::from_material(material);
                if let Some(render_pipeline) = self.render_pipelines.get(&key) {
                    render_pass.set_pipeline(render_pipeline);
                    render_pass.set_bind_group(0, &material.binding.group, &[]);
                    render_pass.set_vertex_buffer(0, primitive.positions.slice(..));
                    render_pass.set_vertex_buffer(1, primitive.normals.slice(..));
                    render_pass.set_vertex_buffer(2, primitive.colors.slice(..));
                    render_pass.set_vertex_buffer(3, primitive.tex_coords.slice(..));
                    render_pass
                        .set_index_buffer(primitive.indices.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..primitive.index_count, 0, run.range.clone());
                }
            }
        }
    }
}
