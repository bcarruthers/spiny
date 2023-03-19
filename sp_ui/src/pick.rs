use glam::Vec2;
use sp_math::range::Range2;

pub fn is_point_in_triangle(v: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> bool {
    let a = 0.5f32 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
    let sign = if a < 0.0f32 { -1.0f32 } else { 1.0f32 };
    let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * v.x + (p0.x - p2.x) * v.y) * sign;
    let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * v.x + (p1.x - p0.x) * v.y) * sign;
    s >= 0.0f32 && t >= 0.0f32 && (s + t) <= 2.0f32 * a * sign
}

/// Returns index of first vertex contained in rect
pub fn pick_point(verts: &[Vec2], rect: Range2) -> Option<usize> {
    for i in 0..verts.len() {
        if rect.contains(verts[i]) {
            return Some(i);
        }
    }
    None
}

/// Returns index of first triangle containing point
pub fn pick_triangle(verts: &[Vec2], p: Vec2) -> Option<usize> {
    for i in 0..verts.len() / 3 {
        let vi = i * 3;
        let v0 = verts[vi + 0];
        let v1 = verts[vi + 1];
        let v2 = verts[vi + 2];
        if is_point_in_triangle(p, v0, v1, v2) {
            return Some(i);
        }
    }
    None
}

/// Returns index of first quad containing point
pub fn pick_quad(verts: &[Vec2], p: Vec2) -> Option<usize> {
    for i in 0..verts.len() / 4 {
        let vi = i * 4;
        let v0 = verts[vi + 0];
        let v1 = verts[vi + 1];
        let v2 = verts[vi + 2];
        let v3 = verts[vi + 3];
        if is_point_in_triangle(p, v0, v1, v2) || is_point_in_triangle(p, v0, v2, v3) {
            return Some(i);
        }
    }
    None
}
