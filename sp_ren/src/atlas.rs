use std::{io::Read, path::PathBuf};

use glam::{IVec2, Vec2, UVec2};
use indexmap::IndexMap;
use image::{RgbaImage, DynamicImage};
use sp_asset::{archive::FileArchive, AssetRef, AssetId};
use sp_draw::{AtlasDef, AtlasEntry, AtlasEntryBounds};
use sp_math::range::{IRange2, Range2};

use crate::{pack::*, Texture};

pub fn read_image<R: Read>(reader: &mut R) -> image::ImageResult<DynamicImage> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    let image = image::load_from_memory(&buf)?;
    Ok(image)
}

pub fn copy_image(dest: &mut RgbaImage, src: &RgbaImage, rect: &IRange2, padding: u32) {
    // let mut buf = Vec::new();
    // reader.read_to_end(&mut buf).unwrap();
    // let image = image::load_from_memory(&buf).unwrap();
    // let image = image.as_rgba8().unwrap();
    let w = src.width().min(rect.size().x as u32);
    let h = src.height().min(rect.size().y as u32);
    let offset = rect.min + IVec2::ONE * padding as i32;
    for y in rect.min.y..rect.max.y {
        for x in rect.min.x..rect.max.x {
            let sx = (x - offset.x).clamp(0, w as i32 - 1);
            let sy = (y - offset.y).clamp(0, h as i32 - 1);
            let pixel = src.get_pixel(sx as u32, sy as u32);
            *dest.get_pixel_mut(x as u32, y as u32) = *pixel;
        }
    }
}

pub struct TextureAtlasInput {
    pub path: PathBuf,
    pub sizes: Option<Vec<UVec2>>,
}

fn to_atlas_images(image: DynamicImage, input: &TextureAtlasInput) -> Vec<RgbaImage> {
    if let Some(sizes) = &input.sizes {
        let filter = image::imageops::FilterType::Lanczos3;
        sizes.iter()
        .map(|&size| {
            image.resize(size.x, size.y, filter).into_rgba8()
        })
        .collect::<Vec<_>>()
    } else {
        vec![image.into_rgba8()]
    }
}

pub fn load_texture_atlas_images(
    archive: &mut FileArchive,
    paths: &[TextureAtlasInput],
    atlas_size: u32,
    padding: u32,
) -> Option<(AtlasDef, RgbaImage)> {
    // Read images
    let images = paths
        .iter()
        .filter_map(|input| {
            let mut file = archive.open(&input.path).unwrap();
            //log::info!("Loading {:?}", path.as_os_str().to_str());
            match read_image(&mut file) {
                Ok(image) => {
                    let asset_ref = AssetRef::from_path(&input.path);
                    log::trace!("Loaded texture {:?}", &asset_ref.path);
                    Some((asset_ref, to_atlas_images(image, input)))
                }
                Err(_err) => {
                    log::warn!("Could not read {}", input.path.as_os_str().to_string_lossy());
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    // Pack rects
    let container = Rect::of_size(atlas_size as usize, atlas_size as usize);
    let items = images.iter()
        .enumerate()
        .flat_map(|(i, (_, images))| {
            images.iter().enumerate().map(move |(j, image)| {
                Item::new(
                    (i, j),
                    image.width() as usize + padding as usize * 2,
                    image.height() as usize + padding as usize * 2,
                    Rotation::None,
                )
            })
        })
        .collect::<Vec<_>>();
    let result = pack(container, items);
    match result {
        Ok(all_packed) => {
            // Write resulting rects to original indices
            let rects = all_packed.into_iter()
                .map(|(r, (i, j))| {
                    let rect = IRange2::sized(
                        IVec2::new(r.x as i32, r.y as i32),
                        IVec2::new(r.w as i32, r.h as i32),
                    );
                    ((i, j), rect)
                }).collect::<IndexMap<_,_>>();
            // Create key->rect lookup
            let scale = Vec2::ONE / atlas_size as f32;
            let entries = images
                .iter()
                .enumerate()
                .map(|(i, (asset_ref, images))| {
                    let bounds =
                        images.iter().enumerate().map(|(j, _image)| {
                            let rect = rects.get(&(i, j)).unwrap();
                            let rect = rect.expand(-IVec2::ONE * padding as i32);
                            let norm_rect = rect.as_range2() * scale;
                            AtlasEntryBounds {
                                rect,
                                norm_rect,
                            }
                        })
                        .collect::<Vec<_>>();
                    log::trace!("Packed {:?} in atlas", &asset_ref.path);
                    (
                        asset_ref.id,
                        AtlasEntry::new(bounds),
                    )
                })
                .collect::<IndexMap<_, _>>();
            // Create atlas image
            let mut dest = image::RgbaImage::new(atlas_size as u32, atlas_size as u32);
            for i in 0..images.len() {
                let (_, entry) = &images[i];
                for j in 0..entry.len() {
                    let image = &entry[j];
                    let rect = rects.get(&(i, j)).unwrap();
                    copy_image(&mut dest, image, rect, padding);
                }
            }
            if log::log_enabled!(log::Level::Trace) {
                log::trace!("Packed {} tetures in atlas", entries.len());
            }
            Some((
                AtlasDef::new(UVec2::splat(atlas_size), entries),
                dest,
            ))
        }
        Err(some_packed) => {
            log::warn!(
                "Could only pack {}/{} textures in atlas",
                some_packed.0.len(),
                images.len()
            );
            None
        }
    }
}

pub struct TextureAtlas {
    desc: AtlasDef,
    texture: Texture,
}

impl TextureAtlas {
    pub fn size(&self) -> UVec2 {
        self.desc.size()
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn def(&self) -> &AtlasDef {
        &self.desc
    }

    pub fn norm_rect(&self, id: AssetId) -> Range2 {
        self.desc.norm_rect(id)
    }

    pub fn from_paths(
        archive: &mut FileArchive,
        paths: &[TextureAtlasInput],
        atlas_size: u32,
        padding: u32,
        filter: wgpu::FilterMode,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<TextureAtlas> {
        load_texture_atlas_images(archive, paths, atlas_size, padding).map(|(desc, image)| {
            let texture = super::texture::Texture::from_rgba_image(
                device,
                queue,
                &image,
                filter,
                None,
                Some("atlas"),
            )
            .unwrap();
            Self { desc, texture }
        })
    }
}
