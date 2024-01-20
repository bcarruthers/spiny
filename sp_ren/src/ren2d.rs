use std::sync::{Arc, Mutex};
use glam::UVec2;
use sp_draw::*;
use sp_asset::{archive::FileArchive, AssetRef};
use crate::{quad::QuadRenderer, *};
use wgpu::{CommandEncoder, TextureView};

pub struct Renderer2d<'window> {
    ctx: GraphicsContext<'window>,
    tex_atlas: TextureAtlas,
    #[allow(dead_code)]
    shader: wgpu::ShaderModule,
    sprite_ren: SpriteRenderer,
    bg_ren: QuadRenderer,
    sample_count: u32,
}

impl<'window> Renderer2d<'window> {
    pub fn new(
        ctx: GraphicsContext<'window>,
        assets: Arc<Mutex<FileArchive>>,
        shader_path: &str,
        bg_path: &str,
        textures: Vec<TextureAtlasInput>,
        sample_count: u32,
    ) -> Self {
        let tex_atlas = {
            let mut assets = assets.lock().unwrap();
            TextureAtlas::from_paths(
                &mut assets,
                &textures,
                2048,
                1,
                wgpu::FilterMode::Linear,
                &ctx.device,
                &ctx.queue)
                .expect("Could not create texture atlas")
            };
        let shader = crate::shader::load_shader(&ctx.device, &assets, shader_path);
        let sprite_ren = {
            SpriteRenderer::new(
                &ctx.device,
                tex_atlas.texture(),
                ctx.config.format,
                &shader,
                sample_count,
                2,
            )
        };
        // This background quad is needed because Chrome clears the framebuffer to a darker
        // color than desktop and other browsers and also flickers. So instead we draw a quad
        // before other rendering as a workaround.
        let bg_ren = {
            QuadRenderer::new(
                &ctx.device,
                tex_atlas.texture(),
                ctx.config.format,
                &shader,
                sample_count,
                tex_atlas.def().get_from_ref(&AssetRef::from_str(bg_path)).lod(0).norm_rect
                    .clone(),
            )
        };
        Self {
            ctx,
            shader,
            tex_atlas,
            bg_ren,
            sprite_ren,
            sample_count
            //framebuffer,
        }
    }

    // pub fn stats(&self) -> RenderStats {
    //     RenderStats {
    //         //models: self.model_ren.count(),
    //         //volumes: self.volume_ren.count(),
    //         sprites: self.sprite_ren.count() as usize,
    //         //chunks: self.chunk_ren.stats(),
    //     }
    // }

    pub fn atlas(&self) -> &TextureAtlas {
        &self.tex_atlas
    }

    fn resize(&mut self, new_size: UVec2) {
        if new_size != self.ctx.size() && new_size != UVec2::ZERO {
            self.ctx.resize(new_size);
        }
    }

    pub fn update(&mut self, size: UVec2, frame: DrawOutput) {
        let span = tracing::span!(tracing::Level::DEBUG, "graphics_update");
        let _enter = span.enter();

        self.bg_ren.update(&self.ctx.queue, frame.clear_color.to_irgba());

        self.resize(size);
        let view_proj = frame.cameras.iter()
            .map(|c| c.proj_view)
            .collect::<Vec<_>>();
        self.sprite_ren.update(&self.ctx.device, &self.ctx.queue, frame.sprites, &view_proj);
    }

    fn draw_world(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mesh_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // Main world viewport covering whole screen
        self.bg_ren.draw(&mut render_pass);
        self.sprite_ren.draw(&mut render_pass);
    }

    fn draw_layers(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let span = tracing::span!(tracing::Level::DEBUG, "graphics_draw");
        let _enter = span.enter();
        self.draw_world(encoder, &view);
    }

    pub fn draw(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.ctx.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render_encoder"),
            });

        // Draw all content
        self.draw_layers(&mut encoder, &view);

        // Submit commands and present
        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Mark the end of primary frame
        #[cfg(tracy)]
        if tracy_client::Client::is_running() {
            tracy_client::frame_mark();
        }
        Ok(())
    }

    pub fn capture(&mut self) -> CaptureImage {
        let size = self.ctx.size();
        let capture = Capture::new(
            &self.ctx.device,
            size.x as usize,
            size.y as usize,
            self.ctx.config.format,
            self.sample_count,
        );
        let command_buffer = {
            let mut encoder =
                self.ctx
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("capture_encoder"),
                    });
            self.draw_layers(&mut encoder, &capture.view());
            capture.copy_to_output(&mut encoder);
            encoder.finish()
        };
        self.ctx.queue.submit(Some(command_buffer));
        capture.to_image_data(&self.ctx.device)
    }
}
