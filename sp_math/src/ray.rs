use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray3 {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    /// Expect ray to have unit length
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let m = self.origin - center;
        let b = m.dot(self.dir);
        let c = m.dot(m) - radius * radius;
        if c > 0.0 && b > 0.0 {
            None
        } else {
            let d = b * b - c;
            if d < 0.0 {
                None
            } else {
                let t = -b - d.sqrt();
                Some(t.max(0.0))
            }
        }
    }

    // Adapted from:
    // http://geomalgorithms.com/a07-_distance.html
    // Copyright 2001 softSurfer, 2012 Dan Sunday
    // This code may be freely used, distributed and modified for any purpose
    // providing that this copyright notice is included with it.
    // SoftSurfer makes no warranty for this code, and cannot be held
    // liable for any real or imagined damage resulting from its use.
    // Users of this code must verify correctness for their application.
    pub fn nearest_points(&self, other: Ray3) -> (f32, f32) {
        let u = self.dir;
        let v = other.dir;
        let w = self.origin - other.origin;
        let a = u.dot(u);
        let b = u.dot(v);
        let c = v.dot(v);
        let d = u.dot(w);
        let e = v.dot(w);
        let dd = a * c - b * b;
        if dd < 1e-5f32 {
            (0.0, if b > c { d / b } else { e / c })
        } else {
            ((b * e - c * d) / dd, (a * e - b * d) / dd)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineSeg3 {
    pub ray: Ray3,
    pub length: f32,
}

impl LineSeg3 {
    pub fn new(ray: Ray3, length: f32) -> Self {
        Self { ray, length }
    }

    pub fn end(&self) -> Vec3 {
        self.ray.origin + self.ray.dir * self.length
    }

    pub fn from_endpoints(origin: Vec3, end: Vec3) -> LineSeg3 {
        let diff = end - origin;
        let length = diff.length();
        let dir = if length > 1e-9f32 {
            diff / length
        } else {
            Vec3::ZERO
        };
        LineSeg3::new(Ray3::new(origin, dir), length)
    }
}
