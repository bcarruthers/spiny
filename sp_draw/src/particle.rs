use glam::Vec2;
use indexmap::IndexMap;
use sp_asset::AssetRef;
use sp_math::color::IRgba;
use crate::AtlasDef;

use super::sprite::*;
use sp_math::range::Range2;
use sp_math::{
    color::{Hsva, Rgba},
    pcg::PcgRng,
};

pub struct ParticleAnimation {
    pub depth: i32,
    pub random_seed: u64,
    pub animation_id: i32,
    pub duration: f32,
    pub width: f32,
    pub height: f32,
    pub size_by_energy: f32,
    pub opacity_by_energy: f32,
    pub acceleration: Vec2,
    pub textures: Vec<String>,
    pub min_tint: Rgba,
    pub max_tint: Rgba,
    pub tint_weight: f32,
    pub min_hue: f32,
    pub max_hue: f32,
    pub min_saturation: f32,
    pub max_saturation: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub opacity: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub velocity_angle_range: f32,
    pub rotation_angle_range: f32,
    pub min_count: i32,
    pub max_count: i32,
    pub initial_distance: f32,
    pub initial_size: f32,
    pub initial_size_range: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParticleGroupId(pub u32);

pub struct ParticleEmission {
    pub group_id: ParticleGroupId,
    pub emit_delay: f32,
    pub emit_interval: f32,
    pub emit_count: i32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: Vec2,
    pub color: Rgba,
    pub energy: f32,
}

impl Default for ParticleEmission {
    fn default() -> Self {
        Self {
            group_id: ParticleGroupId(0),
            emit_delay: 0.0,
            emit_interval: 0.0,
            emit_count: 1,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            rotation: Vec2::X,
            color: Rgba::WHITE,
            energy: 1.0,
        }
    }
}

pub struct ParticleGroup {
    pub group_id: ParticleGroupId,
    pub animations: Vec<ParticleAnimation>,
}

impl Default for ParticleAnimation {
    fn default() -> Self {
        Self {
            depth: 0,
            random_seed: 1,
            animation_id: 0,
            duration: 1.0,
            width: 1.0,
            height: 1.0,
            size_by_energy: 0.0,
            opacity_by_energy: -1.0,
            acceleration: Vec2::ZERO,
            textures: Vec::new(),
            min_hue: 0.0,
            max_hue: 0.0,
            min_saturation: 0.0,
            max_saturation: 0.0,
            min_value: 1.0,
            max_value: 1.0,
            min_tint: Rgba::WHITE,
            max_tint: Rgba::WHITE,
            min_speed: 0.0,
            max_speed: 0.0,
            velocity_angle_range: 360.0,
            rotation_angle_range: 360.0,
            min_count: 1,
            max_count: 1,
            initial_distance: 0.0,
            initial_size: 1.0,
            initial_size_range: 0.0,
            tint_weight: 1.0,
            opacity: 1.0,
        }
    }
}

#[derive(Default)]
struct ParticleBuffers {
    end_times: Vec<f64>,
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    sizes: Vec<f32>,
    rotations: Vec<Vec2>,
    energies: Vec<f32>,
    colors: Vec<Rgba>,
    opacities: Vec<f32>,
    tex_indices: Vec<u32>,
}

impl ParticleBuffers {
    pub fn clear(&mut self) {
        self.end_times.clear();
        self.positions.clear();
        self.velocities.clear();
        self.sizes.clear();
        self.rotations.clear();
        self.energies.clear();
        self.colors.clear();
        self.opacities.clear();
        self.tex_indices.clear();
    }

    fn swap_remove(&mut self, index: usize) {
        self.end_times.swap_remove(index);
        self.positions.swap_remove(index);
        self.velocities.swap_remove(index);
        self.sizes.swap_remove(index);
        self.rotations.swap_remove(index);
        self.energies.swap_remove(index);
        self.colors.swap_remove(index);
        self.opacities.swap_remove(index);
        self.tex_indices.swap_remove(index);
    }
}

/// Cached for every kind of animation, not per emission
struct ParticleSet {
    anim: ParticleAnimation,
    tex_bounds: Vec<Range2>,
    rand: PcgRng,
    buf: ParticleBuffers,
    time: f64,
}

impl ParticleSet {
    pub fn new(anim: ParticleAnimation, atlas: &AtlasDef) -> Self {
        Self {
            rand: PcgRng::new(anim.random_seed, 1),
            tex_bounds: anim
                .textures
                .iter()
                .map(|key| atlas.get_from_ref(&AssetRef::from_str(&key)).lod(0).norm_rect)
                .collect(),
            anim,
            buf: Default::default(),
            time: 0.0,
        }
    }

    fn prune_by_timestamp(&mut self) {
        let mut i = 0;
        while i < self.buf.end_times.len() {
            if self.time >= self.buf.end_times[i] {
                self.buf.swap_remove(i)
            } else {
                i += 1
            }
        }
    }

    fn update_energy(&mut self, energy_unit_to_tick: f32) {
        let energy_rate = -energy_unit_to_tick;
        let mut i = 0;
        while i < self.buf.energies.len() {
            let e = self.buf.energies[i] + energy_rate;
            if e > 0.0 {
                self.buf.energies[i] = e;
                i += 1;
            } else {
                self.buf.swap_remove(i)
            }
        }
    }

    fn update_opacity(&mut self, delta: f32) {
        if delta.abs() > 0.0 {
            for value in self.buf.opacities.iter_mut() {
                *value += delta
            }
        }
    }

    fn update_size(&mut self, delta: f32) {
        if delta.abs() > 0.0 {
            for value in self.buf.sizes.iter_mut() {
                *value += delta
            }
        }
    }

    fn update_velocities(&mut self, delta_time: f32) {
        for vel in self.buf.velocities.iter_mut() {
            *vel += self.anim.acceleration * delta_time
        }
    }

    fn update_positions(&mut self, delta_time: f32) {
        for i in 0..self.buf.positions.len() {
            self.buf.positions[i] += self.buf.velocities[i] * delta_time
        }
    }

    pub fn particle_count(&self) -> usize {
        self.buf.positions.len()
    }

    pub fn clear(&mut self) {
        self.buf.clear()
    }

    pub fn update(&mut self, time: f64) {
        let delta_time = time - self.time;
        self.time = time;
        if !self.buf.positions.is_empty() {
            self.prune_by_timestamp();
            // Energy goes from 1.0 to 0.0 over duration
            let delta_sec = delta_time as f32;
            let relative_delta = delta_sec / self.anim.duration;
            let opacity_delta = self.anim.opacity_by_energy * relative_delta;
            let size_delta = self.anim.size_by_energy * relative_delta;
            self.update_energy(relative_delta);
            self.update_opacity(opacity_delta);
            self.update_size(size_delta);
            self.update_velocities(delta_sec);
            self.update_positions(delta_sec);
        }
    }

    pub fn translate(&mut self, offset: Vec2) {
        for pos in self.buf.positions.iter_mut() {
            *pos += offset;
        }
    }

    pub fn emit(&mut self, pe: &ParticleEmission) {
        let emit_count = {
            let x0 = self.anim.min_count * pe.emit_count;
            let x1 = self.anim.max_count * pe.emit_count;
            self.rand.next_i32_in(x0..x1)
        };
        let size = {
            let x0 = self.anim.initial_size - self.anim.initial_size_range * 0.5;
            let x1 = self.anim.initial_size + self.anim.initial_size_range * 0.5;
            self.rand.next_f32_in(x0..x1)
        };
        let min_tint = self.anim.min_tint;
        let max_tint = self.anim.max_tint;
        for _ in 0..emit_count {
            let dir = self.rand.next_vec2_degrees(self.anim.velocity_angle_range);
            let dir = sp_math::vec::rotate_vec2(dir, pe.rotation);
            let end_time = self.time + self.anim.duration as f64;
            self.buf.end_times.push(end_time);
            self.buf
                .positions
                .push(pe.position + (dir * self.anim.initial_distance));
            self.buf.velocities.push({
                let speed = self
                    .rand
                    .next_f32_in(self.anim.min_speed..self.anim.max_speed);
                dir * speed + pe.velocity
            });
            self.buf.sizes.push(size);
            let rot = self.rand.next_vec2_degrees(self.anim.rotation_angle_range);
            let rot = sp_math::vec::rotate_vec2(rot, dir);
            self.buf.rotations.push(rot.perp());
            self.buf.energies.push(pe.energy);
            self.buf.colors.push({
                let h = self.rand.next_f32_in(self.anim.min_hue..self.anim.max_hue);
                let s = self
                    .rand
                    .next_f32_in(self.anim.min_saturation..self.anim.max_saturation);
                let v = self
                    .rand
                    .next_f32_in(self.anim.min_value..self.anim.max_value);
                let c = Hsva::new(h, s, v, 1.0).to_rgba();
                let tint = {
                    let c = min_tint.lerp(max_tint, self.rand.next_f32());
                    let tint = Rgba::WHITE.lerp(pe.color, self.anim.tint_weight);
                    c * tint
                };
                c * tint
            });
            self.buf.opacities.push(self.anim.opacity);
            let tex_index = if self.tex_bounds.len() == 0 {
                0
            } else {
                self.rand.next_u32_to(self.tex_bounds.len() as u32)
            };
            self.buf.tex_indices.push(tex_index);
        }
    }

    pub fn draw(&self, scope: &mut SpriteBatchBuilder) {
        let mut scope = scope.push_blend(SpriteBlend::Add);
        let mut scope = scope.push_depth_delta(self.anim.depth);
        let mut w = scope.begin_quads();
        let base_size = Vec2::new(self.anim.width, self.anim.height) * 0.5;
        for i in 0..self.buf.positions.len() {
            let scaling = self.buf.sizes[i];
            let color = {
                let opacity = self.buf.opacities[i];
                let color = self.buf.colors[i];
                color.mul_alpha(opacity).to_irgba()
            };
            let size = base_size * scaling;
            let pc = self.buf.positions[i];
            let rot = self.buf.rotations[i];
            let tex_index = self.buf.tex_indices[i] as usize;
            let tb = &self.tex_bounds[tex_index];
            w.draw_sprite(&QuadSprite {
                pos: pc,
                rot,
                size,
                color: [color, IRgba::ZERO, IRgba::ZERO],
                //color: [IRgba::RED, IRgba::GREEN, IRgba::BLUE],
                tex_bounds: *tb,
            });
        }
    }
}

pub struct ParticleSystem {
    groups: IndexMap<ParticleGroupId, Vec<ParticleSet>>,
}

impl ParticleSystem {
    pub fn from_groups(atlas: &AtlasDef, descs: Vec<ParticleGroup>) -> Self {
        let groups = descs
            .into_iter()
            .map(|desc| {
                let group = desc
                    .animations
                    .into_iter()
                    .map(|anim| ParticleSet::new(anim, &atlas))
                    .collect();
                (desc.group_id, group)
            })
            .into_iter()
            .collect::<IndexMap<_, _>>();
        Self { groups }
    }

    pub fn translate(&mut self, offset: Vec2) {
        for (_, group) in self.groups.iter_mut() {
            for set in group.iter_mut() {
                set.translate(offset);
            }
        }
    }

    pub fn particle_count(&self) -> usize {
        let mut count = 0;
        for (_, group) in self.groups.iter() {
            for set in group.iter() {
                count += set.particle_count();
            }
        }
        count
    }

    pub fn clear(&mut self) {
        for (_, group) in self.groups.iter_mut() {
            for set in group.iter_mut() {
                set.clear();
            }
        }
    }

    pub fn emit(&mut self, emission: ParticleEmission) {
        let group = self.groups.get_mut(&emission.group_id).unwrap();
        for set in group.iter_mut() {
            set.emit(&emission)
        }
    }

    pub fn update(&mut self, time: f64) {
        for (_, group) in self.groups.iter_mut() {
            for set in group.iter_mut() {
                set.update(time);
            }
        }
    }

    pub fn draw(&self, scope: &mut SpriteBatchBuilder) {
        for (_, group) in self.groups.iter() {
            for set in group.iter() {
                set.draw(scope);
            }
        }
    }
}
