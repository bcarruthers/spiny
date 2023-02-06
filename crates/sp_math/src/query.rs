use glam::Vec3;

#[derive(Clone)]
pub struct Cone {
    pub origin: Vec3,
    pub dir: Vec3,
    pub radians: f32,
    pub inv_sin_angle: f32,
    pub sin_angle: f32,
    pub cos_angle_sqr: f32,
}

impl Cone {
    pub fn new(origin: Vec3, dir: Vec3, radians: f32) -> Self {
        // https://www.geometrictools.com/GTE/Mathematics/Cone.h
        let cos_angle = radians.cos();
        let sin_angle = radians.sin();
        let cos_angle_sqr = cos_angle * cos_angle;
        let inv_sin_angle = 1.0f32 / sin_angle;
        Self {
            origin,
            dir,
            radians,
            inv_sin_angle,
            sin_angle,
            cos_angle_sqr,
        }
    }

    pub fn intersects_sphere(&self, sphere_center: Vec3, radius: f32) -> bool {
        // https://www.geometrictools.com/GTE/Mathematics/IntrSphere3Cone3.h
        let u = self.origin - (radius * self.inv_sin_angle) * self.dir;
        let cm_u = sphere_center - u;
        let ad_cm_u = self.dir.dot(cm_u);
        if ad_cm_u > 0.0f32 {
            let sqr_length_cm_u = cm_u.dot(cm_u);
            if ad_cm_u * ad_cm_u >= sqr_length_cm_u * self.cos_angle_sqr {
                let cm_v = sphere_center - self.origin;
                let ad_cm_v = self.dir.dot(cm_v);
                if ad_cm_v < -radius {
                    false
                } else {
                    let r_sin_angle = radius * self.sin_angle;
                    if ad_cm_v >= -r_sin_angle {
                        true
                    } else {
                        let sqr_length_cm_v = cm_v.dot(cm_v);
                        sqr_length_cm_v <= radius * radius
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
