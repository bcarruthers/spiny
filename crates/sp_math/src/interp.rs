use std::f32::consts::PI;
use glam::Vec2;

pub fn hermite_interpolate_vec2(p0: Vec2, m0: Vec2, p1: Vec2, m1: Vec2, t: f32) -> Vec2 {
    (2.0 * t * t * t - 3.0 * t * t + 1.0) * p0 +
    (t * t * t - 2.0 * t * t + t) * m0 +
    (-2.0 * t * t * t + 3.0 * t * t) * p1 +
    (t * t * t - t * t) * m1
}

pub fn repeat(t: f32, m: f32) -> f32 {
    let x = t - (t / m).floor() * m;
    x.clamp(0.0, m)
}

pub fn lerp(x0: f32, x1: f32, t: f32) -> f32 {
    (1.0 - t) * x0 + t * x1
}

pub fn lerp_f64(x0: f64, x1: f64, t: f64) -> f64 {
    (1.0 - t) * x0 + t * x1
}

pub fn lerp_rads(a0: f32, a1: f32, t: f32) -> f32 {
    let dt = repeat(a1 - a0, PI * 2.0);
    let dt = if dt > PI { dt - PI * 2.0 } else { dt };
    lerp(a0, a0 + dt, t)
}

pub fn linear_step(x0: f32, x1: f32, t: f32) -> f32 {
    ((t - x0) / (x1 - x0)).clamp(0.0, 1.0)
}

pub fn linear_step_vec2(x0: Vec2, x1: Vec2, t: f32) -> Vec2 {
    Vec2::new(
        linear_step(x0.x, x1.x, t),
        linear_step(x0.y, x1.y, t))
}

pub fn smooth_step(min: f32, max: f32, s: f32) -> f32 {
    let x = linear_step(min, max, s);
    x * x * (3.0 - 2.0 * x)
}
    
pub fn smoother_step(min: f32, max: f32, s: f32) -> f32 {
    let x = linear_step(min, max, s);
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

pub fn smooth_step_vec2(min: Vec2, max: Vec2, s: f32) -> Vec2 {
    let x = linear_step_vec2(min, max, s);
    x * x * (3.0 - 2.0 * x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn lerp_rads_halfway() {
        assert_eq!(lerp_rads(0.0, PI * 0.5, 0.5), PI * 0.25);
    }
}
