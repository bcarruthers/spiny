use fyrox_sound::buffer::{DataSource, SoundBufferResource};
use indexmap::IndexMap;
use sp_asset::{archive::FileArchive, AssetId, AssetRef};
use std::{io::Read, path::PathBuf};

pub struct SoundCache {
    sounds: IndexMap<AssetId, SoundBufferResource>,
}

impl SoundCache {
    pub fn from_paths(archive: &mut FileArchive, paths: &[PathBuf]) -> SoundCache {
        let sounds = paths
            .iter()
            .filter_map(|path| {
                let mut file = archive.open(path).unwrap();
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                match SoundBufferResource::new_generic(DataSource::from_memory(buf)) {
                    Ok(buffer) => {
                        let asset_ref = AssetRef::from_path(&path);
                        log::debug!("Loaded sound {:?} ({})", &asset_ref.path, asset_ref.id.0);
                        Some((asset_ref.id, buffer))
                    }
                    Err(_err) => {
                        log::warn!("Could not read {}", path.as_os_str().to_string_lossy());
                        None
                    }
                }
            })
            .collect();
        Self { sounds }
    }

    pub fn get_sound(&self, sound_id: AssetId) -> &SoundBufferResource {
        self.sounds
            .get(&sound_id)
            .expect(&format!("Could not find sound {}", sound_id.0))
    }
}
