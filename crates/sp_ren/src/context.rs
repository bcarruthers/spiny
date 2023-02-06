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

pub struct GraphicsContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GraphicsContext {
    pub async fn new(
        window: &winit::window::Window,
        size: UVec2,
        backends: Option<wgpu::Backends>,
    ) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = backends.unwrap_or_else(default_backends);
        let instance = wgpu::Instance::new(backends);
        // log::trace!("Created graphics device, adapters: {:?}",
        //     instance.enumerate_adapters(backends).collect::<Vec<_>>());

        let surface = unsafe { instance.create_surface(window) };
        log::trace!("Created surface {:?}", surface);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect(&format!("Could not create adapter for {:?}", backends));
        log::trace!("Created adapter {:?}", adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // Note web does not support POLYGON_MODE_LINE
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter,
                    // so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None, // Trace path
            )
            .await
            .expect("Could not create graphics device");
        log::trace!("Created device {:?}", device);

        // log::info!("Supported present modes: {:?}", surface.get_supported_present_modes(&adapter));
        // log::info!("Supported formats: {:?}", surface.get_supported_formats(&adapter));
        // log::info!("Supported alpha modes: {:?}", surface.get_supported_alpha_modes(&adapter));
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        log::trace!("Configuring surface {:?}", config);
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
        }
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.config.width, self.config.height)
    }

    pub fn resize(&mut self, new_size: UVec2) {
        if new_size != self.size() && new_size != UVec2::ZERO {
            self.config.width = new_size.x;
            self.config.height = new_size.y;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
