pub struct CaptureImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl CaptureImage {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self, name: &str) {
        use chrono::Local;
        use image::ImageEncoder;
        use std::fs::File;

        let timestamp = Local::now().format("%Y%m%d-%H%M%S-%3f").to_string();
        let folder = "captures";
        let output_path = format!("{}/{}-{}.png", folder, name, timestamp);
        log::info!("Saving screen capture to {:?}", &output_path);
        std::fs::create_dir_all(&folder)
            .expect(&format!("Could not create folder {:?}", &folder));
        let file = File::create(&output_path)
            .expect(&format!("Could not open {:?}", &output_path));
        let encoder = image::codecs::png::PngEncoder::new(file);
        encoder
            .write_image(
                &self.pixels,
                self.width as u32,
                self.height as u32,
                image::ColorType::Rgba8,
            )
            .expect("Could not write PNG");
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save(data: sp_ren::CaptureImage) {
    }
}

struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = std::mem::size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

fn copy_bgra_to_rgba(src: &[u8], dest: &mut [u8]) {
    for i in 0..src.len() / 4 {
        let pi = i * 4;
        dest[pi + 0] = src[pi + 2];
        dest[pi + 1] = src[pi + 1];
        dest[pi + 2] = src[pi + 0];
        dest[pi + 3] = src[pi + 3];
    }
}

pub struct Capture {
    buffer_dimensions: BufferDimensions,
    texture_extent: wgpu::Extent3d,
    output_buffer: wgpu::Buffer,
    texture: wgpu::Texture,
}

impl Capture {
    pub fn new(
        device: &wgpu::Device,
        width: usize,
        height: usize,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        // It is a WebGPU requirement that ImageCopyBuffer.layout.bytes_per_row % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT == 0
        // So we calculate padded_bytes_per_row by rounding unpadded_bytes_per_row
        // up to the next multiple of wgpu::COPY_BYTES_PER_ROW_ALIGNMENT.
        // https://en.wikipedia.org/wiki/Data_structure_alignment#Computing_padding
        let buffer_dimensions = BufferDimensions::new(width, height);

        // The output buffer lets us retrieve the data as an array
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_extent = wgpu::Extent3d {
            width: buffer_dimensions.width as u32,
            height: buffer_dimensions.height as u32,
            depth_or_array_layers: 1,
        };

        // The render pipeline renders data into this texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            mip_level_count: 1,
            sample_count, //: 1,
            dimension: wgpu::TextureDimension::D2,
            format, //: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            label: None,
        });

        Self {
            buffer_dimensions,
            texture_extent,
            output_buffer,
            texture,
        }
    }

    pub fn view(&self) -> wgpu::TextureView {
        self.texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn copy_to_output(&self, encoder: &mut wgpu::CommandEncoder) {
        // Copy the data from the texture to the buffer
        encoder.copy_texture_to_buffer(
            self.texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        std::num::NonZeroU32::new(
                            self.buffer_dimensions.padded_bytes_per_row as u32,
                        )
                        .unwrap(),
                    ),
                    rows_per_image: None,
                },
            },
            self.texture_extent,
        )
    }

    pub fn to_image_data(&self, device: &wgpu::Device) -> CaptureImage {
        // Note that we're not calling `.await` here.
        let buffer_slice = self.output_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |result| {
            if let Result::Err(err) = result {
                log::error!("Failed to map buffer: {:?}", err);
            }
        });

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        device.poll(wgpu::Maintain::Wait);

        // #[cfg(not(target_arch = "wasm32"))]
        // pollster::block_on(buffer_future).expect("Could not map buffer");
        // #[cfg(target_arch = "wasm32")]
        // let result = wasm_bindgen_futures::spawn_local(buffer_future);

        let padded_buffer = buffer_slice.get_mapped_range();

        let bytes_per_pixel = std::mem::size_of::<u32>();
        let width = self.buffer_dimensions.width;
        let height = self.buffer_dimensions.height;
        let row_size = width * bytes_per_pixel;
        let size = row_size * height;
        let mut pixels = vec![0; size];
        let mut y = 0;
        for chunk in padded_buffer.chunks(self.buffer_dimensions.padded_bytes_per_row) {
            let src = &chunk[..self.buffer_dimensions.unpadded_bytes_per_row];
            let dest = &mut pixels[y * row_size..(y + 1) * row_size];
            copy_bgra_to_rgba(src, dest);
            //dest.copy_from_slice(src);
            y += 1;
        }

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(padded_buffer);

        self.output_buffer.unmap();

        CaptureImage {
            width,
            height,
            pixels,
        }
    }
}
