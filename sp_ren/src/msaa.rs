pub struct MultisampleFramebuffer {
    view: Option<wgpu::TextureView>,
}

impl MultisampleFramebuffer {
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        if sample_count == 1 {
            Self { view: None }
        } else {
            let multisampled_texture_extent = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };
            let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
                size: multisampled_texture_extent,
                mip_level_count: 1,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: Some("multisample_buffer"),
                view_formats: &[],
            };
            let view = device
                .create_texture(multisampled_frame_descriptor)
                .create_view(&wgpu::TextureViewDescriptor::default());
            Self { view: Some(view) }
        }
    }

    pub fn color_attachment<'a>(
        &'a self,
        view: &'a wgpu::TextureView,
        color: Option<wgpu::Color>,
    ) -> wgpu::RenderPassColorAttachment<'a> {
        let load = match color {
            Some(color) => wgpu::LoadOp::Clear(color),
            None => wgpu::LoadOp::Load
        };
        if let Some(msaa_view) = &self.view {
            wgpu::RenderPassColorAttachment {
                view: msaa_view,
                resolve_target: Some(view),
                ops: wgpu::Operations {
                    load,
                    // Storing pre-resolve MSAA data is unnecessary if it isn't used later.
                    // On tile-based GPU, avoid store can reduce your app's memory footprint.
                    store: wgpu::StoreOp::Discard,
                },
            }
        } else {
            wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            }
        }
    }
}
