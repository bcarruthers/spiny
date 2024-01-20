use image::ImageResult;

use crate::mipmap::MipmapGenerator;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn next_log2(n: u64) -> u32 {
        if n <= 1 {
            0
        } else {
            64 - (n - 1).leading_zeros()
        }
    }

    pub fn mip_level_count(width: u32, height: u32) -> u32 {
        Self::next_log2(width.max(height) as u64).max(1)
    }

    pub fn write_rgba(
        &self,
        queue: &wgpu::Queue,
        image: &image::RgbaImage,
        x: u32,
        y: u32,
        mip_level: u32,
    ) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level,
                origin: wgpu::Origin3d { x, y, z: 0 },
            },
            image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width()),
                rows_per_image: Some(image.height()),
            },
            wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn create_depth(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        sample_count: u32,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });
        Self {
            texture,
            view,
            sampler,
            width: config.width,
            height: config.height,
        }
    }

    pub fn create_rgba(
        device: &wgpu::Device,
        label: Option<&str>,
        width: u32,
        height: u32,
        mip_level_count: u32,
        format: wgpu::TextureFormat,
        filter: wgpu::FilterMode,
    ) -> ImageResult<Self> {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            //mip_level_count: Some(NonZeroU32::new(mip_level_count).unwrap()),
            //mip_level_count: Some(NonZeroU32::new(1).unwrap()),
            ..Default::default()
        });
        // This needs to be linear to mipmaps to be selected
        let mipmap_filter = if mip_level_count > 1 {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        };
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter,
            ..Default::default()
        });
        Ok(Self {
            texture,
            view,
            sampler,
            width,
            height,
        })
    }

    pub fn from_rgba_mip_images(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: &[&image::RgbaImage],
        format: wgpu::TextureFormat,
        filter: wgpu::FilterMode,
        label: Option<&str>,
    ) -> ImageResult<Self> {
        let texture = Self::create_rgba(
            device,
            label,
            images[0].width(),
            images[0].height(),
            images.len() as u32,
            format,
            filter,
        )?;
        for i in 0..images.len() {
            texture.write_rgba(queue, &images[i], 0, 0, i as u32);
        }
        Ok(texture)
    }

    pub fn from_rgba_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::RgbaImage,
        format: wgpu::TextureFormat,
        filter: wgpu::FilterMode,
        mipmap: Option<&MipmapGenerator>,
        label: Option<&str>,
    ) -> ImageResult<Self> {
        let mip_level_count = if mipmap.is_some() {
            Self::mip_level_count(image.width(), image.height())
        } else {
            1
        };
        let texture = Self::create_rgba(
            device,
            label,
            image.width(),
            image.height(),
            mip_level_count,
            format,
            filter,
        )?;
        texture.write_rgba(queue, image, 0, 0, 0);
        if let Some(gen) = mipmap {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mipmap_encoder"),
            });
            gen.generate(&mut encoder, device, &texture.texture, mip_level_count)
        }
        Ok(texture)
    }

    pub fn from_image_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        format: wgpu::TextureFormat,
        filter: wgpu::FilterMode,
        mipmap: Option<&MipmapGenerator>,
        label: &str,
    ) -> ImageResult<Self> {
        let image = image::load_from_memory(bytes)?;
        match image.as_rgba8() {
            None => Self::from_rgba_image(
                device,
                queue,
                &image.to_rgba8(),
                format,
                filter,
                mipmap,
                Some(label),
            ),
            Some(rgba) => Self::from_rgba_image(device, queue, &rgba, format, filter, mipmap, Some(label)),
        }
    }
}
