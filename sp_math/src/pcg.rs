use std::ops::Range;

use glam::{Quat, Vec2, Vec3};

pub struct PcgRng {
    inc: u64,
    state: u64,
}

fn get_u32(state: u64) -> u32 {
    let xor_shifted = (((state >> 18) ^ state) >> 27) as u32;
    let rot = (state >> 59) as i32;
    (xor_shifted >> rot) | (xor_shifted << ((-rot) & 31))
}

fn next_state(state: u64, inc: u64) -> u64 {
    state.wrapping_mul(6364136223846793005u64).wrapping_add(inc)
}

fn get_increment(seq: u64) -> u64 {
    (seq << 1) | 1u64
}

fn get_state(seed: u64, inc: u64) -> u64 {
    let s = next_state(0u64, inc).wrapping_add(seed);
    next_state(s, inc)
}

impl PcgRng {
    pub fn new(seed: u64, seq: u64) -> Self {
        let inc = get_increment(seq);
        let state = get_state(seed, inc);
        Self { inc, state }
    }

    pub fn next_u32(&mut self) -> u32 {
        let result = get_u32(self.state);
        self.state = next_state(self.state, self.inc);
        result
    }

    pub fn next_u32_to(&mut self, max_value: u32) -> u32 {
        if max_value <= 0 {
            0
        } else {
            Self::next_u32(self) % max_value
        }
    }

    pub fn next_u32_in(&mut self, range: Range<u32>) -> u32 {
        Self::next_u32_to(self, range.end - range.start) + range.start
    }

    pub fn next_u64(&mut self) -> u64 {
        let x0 = Self::next_u32(self) as u64;
        let x1 = Self::next_u32(self) as u64;
        x0 | (x1 << 32)
    }

    pub fn next_i32(&mut self) -> i32 {
        Self::next_u32(self) as i32
    }

    pub fn next_i32_to(&mut self, max_value: i32) -> i32 {
        if max_value <= 0 {
            0
        } else {
            (Self::next_u32(self) % max_value as u32) as i32
        }
    }

    pub fn next_i32_in(&mut self, range: Range<i32>) -> i32 {
        Self::next_i32_to(self, range.end - range.start) + range.start
    }

    pub fn next_i64(&mut self) -> i64 {
        Self::next_u64(self) as i64
    }

    pub fn next_i64_to(&mut self, max_value: i64) -> i64 {
        if max_value <= 0 {
            0
        } else {
            (Self::next_u64(self) % max_value as u64) as i64
        }
    }

    pub fn next_i64_in(&mut self, range: Range<i64>) -> i64 {
        Self::next_i64_to(self, range.end - range.start) + range.start
    }

    /// Returns value in closed range [0, 1]
    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 * (1.0 / 4294967295.0)
    }

    /// Returns value in closed range [0, 1]
    pub fn next_f32(&mut self) -> f32 {
        self.next_f64() as f32
    }

    pub fn next_f32_in(&mut self, range: Range<f32>) -> f32 {
        let t = self.next_f32();
        range.start * (1.0 - t) + range.end * t
    }

    /// Returns value in closed range [0, 1]
    pub fn next_vec2(&mut self) -> Vec2 {
        Vec2::new(self.next_f32(), self.next_f32())
    }

    /// Returns value in closed range [0, 1]
    pub fn next_vec3(&mut self) -> Vec3 {
        Vec3::new(self.next_f32(), self.next_f32(), self.next_f32())
    }

    pub fn next_vec2_in_unit_sphere(&mut self) -> Vec2 {
        for _ in 0..10 {
            let p = self.next_vec2() * 2.0 - Vec2::ONE;
            let dist_sqr = p.dot(p);
            if dist_sqr <= 1.0 && dist_sqr > 1e-7 {
                return p;
            }
        }
        Vec2::ZERO
    }

    pub fn next_vec3_in_unit_sphere(&mut self) -> Vec3 {
        for _ in 0..10 {
            let p = self.next_vec3() * 2.0 - Vec3::ONE;
            let dist_sqr = p.dot(p);
            if dist_sqr <= 1.0 && dist_sqr > 1e-7 {
                return p;
            }
        }
        Vec3::ZERO
    }

    pub fn next_vec2_unit_length(&mut self) -> Vec2 {
        self.next_vec2_in_unit_sphere().normalize()
    }

    pub fn next_vec3_unit_length(&mut self) -> Vec3 {
        self.next_vec3_in_unit_sphere().normalize()
    }

    pub fn next_vec3_unit_length_in_hemisphere(&mut self, dir: Vec3) -> Vec3 {
        loop {
            let p = self.next_vec3_in_unit_sphere();
            if p.dot(dir) >= 0.0 {
                return p.normalize()
            }
        }
    }

    pub fn next_vec2_radians(&mut self, radians_range: f32) -> Vec2 {
        let a = (self.next_f32() - 0.5) * radians_range;
        Vec2::new(a.cos(), a.sin())
    }
    
    pub fn next_vec2_degrees(&mut self, degrees_range: f32) -> Vec2 {
        self.next_vec2_radians(degrees_range.to_radians())
    }
    
    pub fn next_quat(&mut self) -> Quat {
        let v1 = self.next_vec2_in_unit_sphere();
        let v2 = self.next_vec2_in_unit_sphere();
        let s = ((1.0 - v1.length_squared()) / v2.length_squared()).sqrt();
        Quat::from_array([v1.x, v1.y, s * v2.x, s * v2.y])
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        if !items.is_empty() {
            for i in 0..items.len() - 1 {
                let j = self.next_u32_in(i as u32..items.len() as u32) as usize;
                items.swap(i, j);
            }
        }
    }
}

impl Default for PcgRng {
    fn default() -> Self {
        Self::new(0x853c49e6748fea9bu64, 0xda3e39cb94b95bdbu64)
    }
}
