use std::sync::Arc;

use glam::*;

fn default_backends() -> wgpu::Backends {
    #[cfg(target_os = "windows")]
    {
        wgpu::Backends::DX11 | wgpu::Backends::DX12
    }
    #[cfg(target_family = "unix")]
    {
        wgpu::Backends::PRIMARY
    }
    #[cfg(target_arch = "wasm32")]
    {
        wgpu::Backends::all()
    }
}

#[derive(Clone, Debug)]
pub struct GraphicsError {
    pub message: String,
}

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self, self.message)
    }
}

pub struct GraphicsContext<'window> {
    pub window: Arc<winit::window::Window>,
    pub surface: wgpu::Surface<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl<'window> GraphicsContext<'window> {
    pub async fn new(
        window: Arc<winit::window::Window>,
        backends: Option<wgpu::Backends>,
    ) -> Result<Self, GraphicsError> {
        let size = UVec2::new(window.inner_size().width, window.inner_size().height)
            .max(UVec2::ONE);

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = backends.unwrap_or_else(default_backends);
        log::debug!("Creating graphics context for {:?}", backends);

        let desc = wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        };
        let instance = wgpu::Instance::new(desc);
        // log::trace!("Created graphics device, adapters: {:?}",
        //     instance.enumerate_adapters(backends).collect::<Vec<_>>());

        let surface = instance.create_surface(window.clone())
            .map_err(|e| GraphicsError {
                message: format!("Could not create surface: {:?}", e),
            })?;
        log::debug!("Created {:?}", surface);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| GraphicsError {
                message: format!("Could not create adapter for {:?}", backends),
            })?;
        log::debug!("Created {:?}", adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // Note web does not support POLYGON_MODE_LINE
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter,
                    // so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None, // Trace path
            )
            .await
            .map_err(|e| GraphicsError {
                message: format!("Could not create device: {:?}", e),
            })?;
        log::debug!("Created {:?}", device);

        // log::info!("Supported present modes: {:?}", surface.get_supported_present_modes(&adapter));
        // log::info!("Supported formats: {:?}", surface.get_supported_formats(&adapter));
        // log::info!("Supported alpha modes: {:?}", surface.get_supported_alpha_modes(&adapter));
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        log::debug!("Configuring surface {:?}", config);
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
        })
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.config.width, self.config.height)
    }

    pub fn resize(&mut self, new_size: UVec2) {
        if new_size != self.size() && new_size != UVec2::ZERO {
            log::debug!("Resizing surface to {:?}", new_size);
            self.config.width = new_size.x;
            self.config.height = new_size.y;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
