use glam::{Vec3, Vec2};

pub fn rotate_vec2(rot: Vec2, a: Vec2) -> Vec2 {
    Vec2::new(a.x * rot.x - a.y * rot.y, a.x * rot.y + a.y * rot.x)
}    

pub fn inv_rotate_vec2(rot: Vec2, a: Vec2) -> Vec2 {
    Vec2::new(a.x * rot.x + a.y * rot.y, a.y * rot.x - a.x * rot.y)
}

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
