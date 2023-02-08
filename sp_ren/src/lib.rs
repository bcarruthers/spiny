//#![forbid(unsafe_code)]

pub mod atlas;
pub mod binding;
pub mod buffer;
pub mod camera;
pub mod capture;
pub mod context;
pub mod convert;
pub mod instance;
pub mod mesh;
pub mod mipmap;
pub mod model;
pub mod msaa;
pub mod pack;
pub mod quad;
pub mod ren2d;
pub mod shader;
pub mod sprite;
pub mod text;
pub mod texture;
pub mod tile;
pub mod vertex;

pub use atlas::{TextureAtlas, TextureAtlasInput};
pub use camera::CameraParams;
pub use capture::Capture;
pub use capture::CaptureImage;
pub use context::GraphicsContext;
pub use convert::*;
pub use mipmap::MipmapGenerator;
pub use model::*;
pub use msaa::MultisampleFramebuffer;
pub use sprite::SpriteRenderer;
pub use text::TextRenderer;
pub use texture::Texture;

#[cfg(test)]
mod tests;
