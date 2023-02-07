use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use sp_asset::archive::FileArchive;
use sp_sound::{AudioConfig, AudioListener, SoundEffect, AudioFrame};
use crate::{AudioEngine, SoundCache};

pub struct AudioSystem {
    engine: Option<AudioEngine>,
    sounds: SoundCache,
    config: AudioConfig,
}

impl AudioSystem {
    pub fn new(assets: Arc<Mutex<FileArchive>>) -> Self {
        let mut assets = assets.lock().unwrap();
        let paths = assets
            .files_in(Path::new("sounds"))
            .expect("Could not load sounds");
        let sounds = SoundCache::from_paths(&mut assets, &paths);
        Self {
            engine: None,
            sounds,
            config: Default::default(),
        }
    }

    pub fn enable(&mut self) {
        // Browsers require user interaction before audio can play, so we
        // deferring engine creation until the first audio frame, which
        // presumably happens as a result of user interaction
        if self.engine.is_none() {
            self.engine = Some(AudioEngine::new());
        }
    }

    fn enable_on_demand(&mut self) {
        // Enable on demand for desktop without requiring user interaction
        #[cfg(not(target_arch = "wasm32"))]
        self.enable();
    }

    pub fn set_listener(&mut self, listener: AudioListener) {
        self.enable_on_demand();
        if let Some(engine) = &mut self.engine {
            engine.set_master_gain(self.config.sound_volume as f32 / 10.0);
            engine.set_position(listener.pos);
            engine.set_orientation(listener.look, listener.up);
        }
    }

    pub fn play_sound(&mut self, play: &SoundEffect) {
        self.enable_on_demand();
        if let Some(engine) = &mut self.engine {
            //log::info!("Playing sound {} at {:?}", play.sound_id.0, play.playback.position);
            let sound = self.sounds.get_sound(play.sound_id).clone();
            engine.play(
                sound,
                play.playback.position,
                play.playback.gain,
                play.playback.spatial_blend,
                play.playback.min_radius,
                play.playback.max_radius,
                play.playback.rolloff,
            );
        }
    }

    pub fn handle(&mut self, frame: AudioFrame) {
        if !frame.sounds.is_empty() {
            self.set_listener(frame.listener);
            for play in frame.sounds.iter() {
                self.play_sound(play);
            }
        }
    }
}