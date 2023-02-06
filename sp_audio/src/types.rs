use glam::Vec3;
use serde_derive::{Serialize, Deserialize};
use sp_asset::AssetId;


#[derive(Clone)]
pub struct AudioListener {
    pub pos: Vec3,
    pub look: Vec3,
    pub up: Vec3,
}

impl Default for AudioListener {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            look: Vec3::Z,
            up: Vec3::Y,
        }
    }
}

#[derive(Clone)]
pub struct SoundPlayback {
    pub loops: u32,
    pub gain: f32,
    pub pitch: f32,
    pub position: Vec3,
    pub spatial_blend: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub rolloff: f32,
}

impl Default for SoundPlayback {
    fn default() -> Self {
        Self {
            loops: 1,
            gain: 1.0,
            pitch: 1.0,
            position: Vec3::ZERO,
            spatial_blend: 1.0,
            min_radius: 1.0,
            max_radius: f32::MAX,
            rolloff: 1.0,
        }
    }
}

#[derive(Default, Clone)]
pub struct SoundEffect {
    pub sound_id: AssetId,
    pub playback: SoundPlayback,
    pub pitch_range: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sound_volume: i32,
    pub music_volume: i32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sound_volume: 5,
            music_volume: 5,
        }
    }
}
