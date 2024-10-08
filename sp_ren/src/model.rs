use std::ops::Range;

use crate::binding::TextureBinding;
use crate::camera::CameraBinding;
use crate::instance::InstanceRaw;
use crate::{texture, CameraParams, TextureAtlas};
use glam::{Quat, Vec3};
use indexmap::IndexMap;
use sp_asset::AssetId;
use wgpu::util::DeviceExt;
use wgpu::BindGroupLayout;

const MAX_CAMERAS: usize = 16;

pub struct ModelNode {
    pub mesh: Option<u32>,
    pub children: Vec<u32>,
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

pub struct RenderPrimitive {
    pub positions: wgpu::Buffer,
    pub normals: wgpu::Buffer,
    pub colors: wgpu::Buffer,
    pub tex_coords: wgpu::Buffer,
    pub joint_indices: wgpu::Buffer,
    pub joint_weights: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_count: u32,
    pub material: usize,
}

impl RenderPrimitive {
    pub fn from_buffers(
        device: &wgpu::Device,
        material: usize,
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        colors: &[[f32; 4]],
        tex_coords: &[[f32; 2]],
        joint_indices: &[[u32; 4]],
        joint_weights: &[[f32; 4]],
        indices: &[u32],
    ) -> Self {
        let vertex_count = positions.len() as u32;
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
        let joint_indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Index Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(joint_indices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let joint_weights = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, //Some(&format!("{:?} Index Buffer", path.as_ref())),
            contents: bytemuck::cast_slice(joint_weights),
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
            vertex_count,
            joint_indices,
            joint_weights,
            indices,
            index_count,
        }
    }
}

pub struct RenderMesh {
    pub primitives: Vec<RenderPrimitive>,
}

#[derive(Default)]
pub struct RenderModel {
    pub meshes: Vec<RenderMesh>,
    pub materials: Vec<Material>,
}

impl RenderModel {
    pub fn from_atlas_quad(device: &wgpu::Device, atlas: &TextureAtlas, asset_id: AssetId) -> Self {
        let tb = atlas.norm_rect(asset_id);
        Self {
            materials: vec![Material {
                emissive_texture: None,
                binding: TextureBinding::new(device, atlas.texture()),
                descriptor: MaterialDescriptor::default(),
            }],
            meshes: vec![RenderMesh {
                primitives: vec![RenderPrimitive::from_buffers(
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
                    &[],
                    &[],
                    &[0, 1, 2, 0, 2, 3, 0, 2, 1, 0, 3, 2],
                )],
            }],
        }
    }
}

pub struct RenderModelCache {
    models: IndexMap<AssetId, RenderModel>,
}

impl RenderModelCache {
    pub fn new() -> Self {
        Self {
            models: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.models.clear();
    }

    pub fn add(&mut self, model_id: AssetId, model: RenderModel) {
        self.models.insert(model_id, model);
    }

    pub fn model(&self, model_id: AssetId) -> Option<&RenderModel> {
        self.models.get(&model_id)
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
            alpha_cutoff: material
                .descriptor
                .alpha_cutoff
                .map(|x| (x * 65535.0) as u32),
            double_sided: material.descriptor.double_sided,
        }
    }
}

#[derive(Default, Clone)]
pub struct ModelInstanceRun {
    pub model_id: AssetId,
    pub mesh_id: u32,
    pub cull_cw: bool,
    pub range: Range<u32>,
}

pub struct ModelInstanceBuffer {
    runs: Vec<ModelInstanceRun>,
    instances: Vec<InstanceRaw>,
    max_instances: u32,
    #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
}

impl ModelInstanceBuffer {
    pub fn new(device: &wgpu::Device, max_instances: u32) -> Self {
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("model_instance_buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: max_instances as u64 * std::mem::size_of::<InstanceRaw>() as u64,
            mapped_at_creation: false,
        });
        Self {
            runs: Vec::new(),
            instances: Vec::new(),
            instance_buffer,
            max_instances,
        }
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }

    pub fn runs(&self) -> &[ModelInstanceRun] {
        &self.runs
    }

    pub fn device_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.runs.clear();
    }

    pub fn push_model_run(
        &mut self,
        model_id: AssetId,
        mesh_id: u32,
        cull_cw: bool,
        range: Range<u32>,
    ) {
        if range.start < self.max_instances {
            let range = range.start..range.end.min(self.max_instances);
            self.runs.push(ModelInstanceRun {
                model_id,
                mesh_id,
                cull_cw,
                range,
            });
        }
    }

    pub fn push_instance(&mut self, instance: InstanceRaw) {
        if self.instances.len() < self.max_instances as usize {
            self.instances.push(instance);
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }
}

pub struct ModelRenderer {
    format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    multisample_count: u32,
    shader: wgpu::ShaderModule,
    camera_binding: CameraBinding,
    render_pipelines: IndexMap<RenderPipelineKey, wgpu::RenderPipeline>,
    instances: ModelInstanceBuffer,
}

impl ModelRenderer {
    pub fn new(
        device: &wgpu::Device,
        shader: wgpu::ShaderModule,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        multisample_count: u32,
        max_instances: u32,
    ) -> Self {
        let camera_binding = CameraBinding::new(device, MAX_CAMERAS);
        let instances = ModelInstanceBuffer::new(device, max_instances);
        Self {
            format,
            depth_format,
            multisample_count,
            instances,
            shader,
            camera_binding,
            render_pipelines: Default::default(),
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        multisample_count: u32,
        camera_layout: &BindGroupLayout,
        key: RenderPipelineKey,
        lights_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("model_pipeline_layout"),
                bind_group_layouts: &[
                    &TextureBinding::create_layout(device, wgpu::TextureViewDimension::D2),
                    camera_layout,
                    lights_layout,
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
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: match key.alpha_mode {
                        MaterialAlphaMode::Opaque => Some(wgpu::BlendState::REPLACE),
                        MaterialAlphaMode::Mask => Some(wgpu::BlendState::ALPHA_BLENDING),
                        MaterialAlphaMode::Blend => Some(wgpu::BlendState::ALPHA_BLENDING),
                    },
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: if key.double_sided {
                    None
                } else {
                    Some(wgpu::Face::Back)
                },
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
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
            cache: None,
        })
    }

    pub fn instances(&self) -> &ModelInstanceBuffer {
        &self.instances
    }

    pub fn instances_mut(&mut self) -> &mut ModelInstanceBuffer {
        &mut self.instances
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cameras: &[CameraParams],
        models: &RenderModelCache,
        lights_layout: &wgpu::BindGroupLayout,
    ) {
        self.camera_binding.update(queue, cameras);
        self.instances.update(queue);
        // Create material pipelines
        for run in self.instances.runs().iter() {
            if let Some(model) = models.model(run.model_id) {
                for mesh in model.meshes.iter() {
                    for primitive in mesh.primitives.iter() {
                        let material = &model.materials[primitive.material];
                        let key = RenderPipelineKey::from_material(material);
                        self.render_pipelines.entry(key).or_insert_with(|| {
                            Self::create_pipeline(
                                device,
                                &self.shader,
                                self.format,
                                self.depth_format,
                                self.multisample_count,
                                self.camera_binding.layout(),
                                key,
                                lights_layout,
                            )
                        });
                    }
                }
            }
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_index: usize,
        models: &'a RenderModelCache,
        lights_binding: &'a wgpu::BindGroup,
    ) {
        render_pass.set_bind_group(1, &self.camera_binding.bind_group(camera_index), &[]);
        render_pass.set_bind_group(2, lights_binding, &[]);
        render_pass.set_vertex_buffer(4, self.instances.device_buffer().slice(..));
        for run in self.instances.runs().iter() {
            if let Some(model) = models.model(run.model_id) {
                let mesh = &model.meshes[run.mesh_id as usize];
                for primitive in mesh.primitives.iter() {
                    let material = &model.materials[primitive.material];
                    let key = RenderPipelineKey::from_material(material);
                    if let Some(render_pipeline) = self.render_pipelines.get(&key) {
                        render_pass.set_pipeline(render_pipeline);
                        render_pass.set_bind_group(0, &material.binding.group, &[]);
                        render_pass.set_vertex_buffer(0, primitive.positions.slice(..));
                        render_pass.set_vertex_buffer(1, primitive.normals.slice(..));
                        render_pass.set_vertex_buffer(2, primitive.colors.slice(..));
                        render_pass.set_vertex_buffer(3, primitive.tex_coords.slice(..));
                        render_pass.set_index_buffer(
                            primitive.indices.slice(..),
                            wgpu::IndexFormat::Uint32,
                        );
                        render_pass.draw_indexed(0..primitive.index_count, 0, run.range.clone());
                    }
                }
            }
        }
    }
}