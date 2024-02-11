use glam::{Mat4, Vec3};

use crate::binding::PodBuffer;

#[derive(Clone, Copy)]
pub struct CameraParams {
    pub view: Mat4,
    pub proj: Mat4,
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub light_dir: Vec3,
    pub fog_color: [f32; 4],
    pub fog_depth: f32,
    pub tint_color: [f32; 4],
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            ambient: [0.1, 0.1, 0.1, 1.0],
            diffuse: [1.0, 1.0, 1.0, 1.0],
            light_dir: Vec3::new(1.0, 2.0, -4.0).normalize(),
            fog_color: [0.025, 0.05, 0.1, 1.0],
            fog_depth: 0.0,
            tint_color: [1.0; 4],
        }
    }
}

impl CameraParams {
    pub fn proj_view(&self) -> Mat4 {
        self.proj * self.view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuCameraParams {
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
    ambient: [f32; 4],
    diffuse: [f32; 4],
    light_dir: [f32; 4],
    fog_color: [f32; 4],
    fog_depth: [f32; 4],
    tint_color: [f32; 4],
}

impl GpuCameraParams {
    pub fn from_params(camera: &CameraParams) -> Self {
        Self {
            view: camera.view.to_cols_array_2d(),
            proj: camera.proj.to_cols_array_2d(),
            ambient: camera.ambient,
            diffuse: camera.diffuse,
            light_dir: camera.light_dir.extend(1.0).to_array(),
            fog_color: camera.fog_color,
            fog_depth: [camera.fog_depth; 4],
            tint_color: camera.tint_color,
        }
    }
}

impl Default for GpuCameraParams {
    fn default() -> Self {
        Self::from_params(&CameraParams::default())
    }
}

pub struct CameraBinding {
    layout: wgpu::BindGroupLayout,
    buffers: Vec<PodBuffer<GpuCameraParams>>,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl CameraBinding {
    pub fn new(device: &wgpu::Device, max_cameras: usize) -> Self {
        let mut buffers = Vec::new();
        for _ in 0..max_cameras {
            buffers.push(PodBuffer::new(device, 0,
                wgpu::ShaderStages::VERTEX_FRAGMENT, Default::default()));
        }
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                buffers[0].layout_entry,
            ],
            label: Some("block_transform_bind_group_layout"),
        });
        let mut bind_groups = Vec::new();
        for i in 0..max_cameras {
            let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    buffers[i].as_bind_group_entry(),
                ],
                label: Some("block_transform_bind_group"),
            });
            bind_groups.push(bindings);
        }
        Self {
            layout,
            buffers,
            bind_groups,
        }
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    pub fn bind_group(&self, index: usize) -> &wgpu::BindGroup {
        &self.bind_groups[index]
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        cameras: &[CameraParams],
    ) {
        for i in 0..cameras.len() {
            let camera = &cameras[i];
            self.buffers[i].write(queue, GpuCameraParams::from_params(camera));
        }
    }

}