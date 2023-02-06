use std::sync::{Arc, Mutex};

use fyrox_sound::{
    buffer::SoundBufferResource,
    context::SoundContext,
    engine::SoundEngine,
    pool::Handle,
    source::{SoundSource, SoundSourceBuilder, Status},
};
use glam::Vec3;

fn as_vector3(v: Vec3) -> fyrox_sound::algebra::Vector3<f32> {
    fyrox_sound::algebra::Vector3::new(v.x, v.y, v.z)
}

pub struct AudioEngine {
    _engine: Arc<Mutex<SoundEngine>>,
    context: SoundContext,
}

impl AudioEngine {
    pub fn new() -> Self {
        // Initialize sound engine with default output device.
        let engine = SoundEngine::new();

        // Create new context.
        let context = SoundContext::new();

        // Register context in the engine.
        engine.lock().unwrap().add_context(context.clone());

        Self {
            _engine: engine,
            context,
        }
    }

    pub fn set_master_gain(&mut self, gain: f32) {
        self.context.state().set_master_gain(gain);
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.context
            .state()
            .listener_mut()
            .set_position(as_vector3(pos))
    }

    pub fn set_orientation(&mut self, look: Vec3, up: Vec3) {
        self.context
            .state()
            .listener_mut()
            .set_orientation_rh(as_vector3(look), as_vector3(up))
    }

    pub fn play(
        &mut self,
        sound_buffer: SoundBufferResource,
        pos: Vec3,
        gain: f32,
        spatial_blend: f32,
        min_radius: f32,
        max_radius: f32,
        rolloff: f32,
    ) {
        //log::info!("Playing sound at {:?} with gain {}, range {} to {}", pos, gain, min_radius, max_radius);

        // Create generic source (without spatial effects) using that buffer.
        let source = SoundSourceBuilder::new()
            .with_buffer(sound_buffer)
            .with_status(Status::Playing)
            .with_gain(gain)
            .with_position(as_vector3(pos))
            .with_spatial_blend_factor(spatial_blend)
            .with_rolloff_factor(rolloff)
            .with_radius(min_radius)
            .with_max_distance(max_radius)
            .build()
            .unwrap();

        let _source_handle: Handle<SoundSource> = self.context.state().add_source(source);
    }
}
