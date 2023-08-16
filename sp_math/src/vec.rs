use glam::{Vec3, Vec2};

pub fn truncate_or_zero_vec2(v: Vec2, max_length: f32) -> Vec2 {
    let length_sqr = v.length_squared();
    if length_sqr <= max_length * max_length {
        v
    } else {
        v.normalize_or_zero() * max_length
    }
}

pub fn truncate_or_zero_vec3(v: Vec3, max_length: f32) -> Vec3 {
    let length_sqr = v.length_squared();
    if length_sqr <= max_length * max_length {
        v
    } else {
        v.normalize_or_zero() * max_length
    }
}
