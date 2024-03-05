use super::texture::*;
use glam::*;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;

pub struct PodBuffer<U> {
    pub uniform: PhantomData<U>,
    pub layout_entry: wgpu::BindGroupLayoutEntry,
    pub buffer: wgpu::Buffer,
}

impl<U: bytemuck::Pod> PodBuffer<U> {
    pub fn new(
        device: &wgpu::Device,
        binding: u32,
        visibility: wgpu::ShaderStages,
        label: Option<&str>,
        uniform: U,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        Self {
            uniform: PhantomData::default(),
            layout_entry,
            buffer,
        }
    }

    pub fn as_bind_group_entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: self.layout_entry.binding,
            resource: self.buffer.as_entire_binding(),
        }
    }

    pub fn write(&self, queue: &wgpu::Queue, uniform: U) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Mat4Uniform {
    value: [[f32; 4]; 4],
}

pub struct TransformBinding {
    uniform: Mat4Uniform,
    pub layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
}

impl TransformBinding {
    pub fn new(device: &wgpu::Device, value: Mat4) -> Self {
        let uniform = Mat4Uniform {
            value: value.to_cols_array_2d(),
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("transform_buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("transform_bind_group_layout"),
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("transform_bind_group"),
        });

        Self {
            uniform,
            layout,
            buffer,
            group,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, value: Mat4) {
        self.uniform.value = value.to_cols_array_2d();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

pub struct TextureBinding {
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
}

impl TextureBinding {
    pub fn create_layout_multisampled(
        device: &wgpu::Device,
        view_dimension: wgpu::TextureViewDimension,
        multisampled: bool,
    ) -> wgpu::BindGroupLayout {
        let filterable = !multisampled;
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled,
                        view_dimension,
                        sample_type: wgpu::TextureSampleType::Float { filterable },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }

    pub fn create_layout(
        device: &wgpu::Device,
        view_dimension: wgpu::TextureViewDimension,
    ) -> wgpu::BindGroupLayout {
        Self::create_layout_multisampled(device, view_dimension, false)
    }

    pub fn new_multisampled(device: &wgpu::Device, texture: &Texture, multisampled: bool) -> Self {
        let layout =
            Self::create_layout_multisampled(device, wgpu::TextureViewDimension::D2, multisampled);
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });
        Self { layout, group }
    }

    pub fn new(device: &wgpu::Device, texture: &Texture) -> Self {
        Self::new_multisampled(device, texture, false)
    }
}
