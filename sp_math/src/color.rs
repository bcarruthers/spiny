use glam::Vec4;

/// sRGB to linear conversion:
/// https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_texture_sRGB_decode.txt
fn srgba_to_linear(x: f32) -> f32 {
    if x > 0.04045 {
        ((x + 0.055) / 1.055).powf(2.4)
    } else {
        x / 12.92
    }
}

/// Linear to sRGB conversion:
/// https://en.wikipedia.org/wiki/SRGB
fn linear_to_srgba(x: f32) -> f32 {
    if x > 0.0031308 {
        let a = 0.055;
        (1.0 + a) * x.powf(-2.4) - a
    } else {
        12.92 * x
    }
}

fn lerp_hue(h0: f32, h1: f32, t: f32) -> f32 {
    let (h0, h1, t) =
    if (h1 - h0).abs() < 180.0 {
        (h0, h1, t)
    } else if h1 > h0 {
        (h1, h0 + 360.0, 1.0 - t)
    } else {
        (h0, h1 + 360.0, t)
    };
    let h = crate::interp::lerp(h0, h1, t);
    if h >= 360.0 {
        h - 360.0
    } else {
        h
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct IRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl IRgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const fn lum(x: u8) -> Self {
        Self::rgb(x, x, x)
    }

    /// R is in high bits, A is in low bits
    pub const fn from_u32(x: u32) -> Self {
        Self::new(
            ((x >> 24) & 0xff) as u8,
            ((x >> 16) & 0xff) as u8,
            ((x >> 8) & 0xff) as u8,
            ((x >> 0) & 0xff) as u8,
        )
    }

    pub const WHITE: Self = Self::from_u32(0xffffffff);
    pub const BLACK: Self = Self::from_u32(0x000000ff);
    pub const ZERO: Self = Self::from_u32(0x00000000);
    pub const RED: Self = Self::from_u32(0xff0000ff);
    pub const YELLOW: Self = Self::from_u32(0xffff00ff);
    pub const GREEN: Self = Self::from_u32(0x00ff00ff);
    pub const CYAN: Self = Self::from_u32(0x00ffffff);
    pub const BLUE: Self = Self::from_u32(0x0000ffff);
    pub const MAGENTA: Self = Self::from_u32(0xff00ffff);

    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_rgba(&self) -> Rgba {
        Rgba::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn lum(x: f32) -> Self {
        Self::rgb(x, x, x)
    }

    pub fn from_vec4(v: Vec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }

    /// R is in high bits, A is in low bits
    pub fn from_u32(x: u32) -> Self {
        IRgba::from_u32(x).to_rgba()
    }

    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    pub fn to_irgba(&self) -> IRgba {
        IRgba::new(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    pub fn to_hsva(&self) -> Hsva {
        let m = self.r.min(self.g).min(self.b);
        let v = self.r.max(self.g).max(self.b);
        let s = if v > 1e-7f32 { 1.0 - m / v } else { 0.0 };
        let l = (m + v) / 2.0;
        let vm = v - m;
        if l < 1e-7 || vm < 1e-7 {
            Hsva::new(0.0, s, v, self.a)
        } else {
            let r2 = (v - self.r) / vm;
            let g2 = (v - self.g) / vm;
            let b2 = (v - self.b) / vm;
            let hx = if self.r == v {
                if self.g == m {
                    5.0 + b2
                } else {
                    1.0 - g2
                }
            } else if self.g == v {
                if self.b == m {
                    1.0 + r2
                } else {
                    3.0 - b2
                }
            } else if self.r == m {
                3.0 + g2
            } else {
                5.0 - r2
            };
            let h = if hx >= 6.0 { hx - 6.0 } else { hx };
            Hsva::new(h * 60.0, s, v, self.a)
        }
    }

    pub fn mul_alpha(&self, alpha: f32) -> Self {
        Self {
            a: self.a * alpha,
            ..*self
        }
    }

    pub fn mul_rgb(&self, t: f32) -> Self {
        Self {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
            ..*self
        }
    }

    pub fn lerp(&self, rhs: Self, t: f32) -> Self {
        Self::from_vec4(self.to_vec4().lerp(rhs.to_vec4(), t))
    }

    pub fn srgba_to_linear(&self) -> Self {
        Self {
            r: srgba_to_linear(self.r),
            g: srgba_to_linear(self.g),
            b: srgba_to_linear(self.b),
            a: self.a,
        }
    }

    pub fn linear_to_srgba(&self) -> Self {
        Self {
            r: linear_to_srgba(self.r),
            g: linear_to_srgba(self.g),
            b: linear_to_srgba(self.b),
            a: self.a,
        }
    }
}

impl std::ops::Mul<Rgba> for Rgba {
    type Output = Self;
    fn mul(self, rhs: Rgba) -> Self {
        Self::from_vec4(self.to_vec4() * rhs.to_vec4())
    }
}

impl std::ops::Mul<f32> for Rgba {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::from_vec4(self.to_vec4() * rhs)
    }
}

/// Hue is in degrees: [0, 360]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Hsva {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

impl Hsva {
    pub const fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { h, s, v, a }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::new(h, s, v, 1.0)
    }

    pub fn from_vec4(v: Vec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.h, self.s, self.v, self.a)
    }

    pub fn to_rgba(&self) -> Rgba {
        let h = self.h / 360.0;
        let s = self.s;
        let v = self.v;
        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        match (i as u32) % 6 {
            0 => Rgba::new(v, t, p, self.a),
            1 => Rgba::new(q, v, p, self.a),
            2 => Rgba::new(p, v, t, self.a),
            3 => Rgba::new(p, q, v, self.a),
            4 => Rgba::new(t, p, v, self.a),
            5 => Rgba::new(v, p, q, self.a),
            _ => Rgba::ZERO,
        }
    }

    pub fn add_hue(&self, h: f32) -> Self {
        Self { h: self.h + h, ..*self }
    }

    pub fn mul_alpha(&self, alpha: f32) -> Self {
        Self {
            a: self.a * alpha,
            ..*self
        }
    }

    pub fn lerp(&self, rhs: Self, t: f32) -> Self {
        let h = lerp_hue(self.h, rhs.h, t);
        Self {
            h,
            s: crate::interp::lerp(self.s, rhs.s, t),
            v: crate::interp::lerp(self.v, rhs.v, t),
            a: crate::interp::lerp(self.a, rhs.a, t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip(c: Rgba) {
        assert_eq!(c.to_hsva().to_rgba(), c);
    }

    #[test]
    fn lerp_hsva() {
        assert_eq!(Rgba::GREEN.to_hsva().lerp(Rgba::RED.to_hsva(), 0.5).h, 60.0);
        assert_eq!(Rgba::RED.to_hsva().lerp(Rgba::GREEN.to_hsva(), 0.5).h, 60.0);
        assert_eq!(Rgba::MAGENTA.to_hsva().lerp(Rgba::RED.to_hsva(), 0.5).h, 330.0);
        assert_eq!(Rgba::RED.to_hsva().lerp(Rgba::MAGENTA.to_hsva(), 0.5).h, 330.0);
    }

    #[test]
    fn convert_hsva() {
        test_roundtrip(Rgba::BLACK);
        test_roundtrip(Rgba::CYAN);
        test_roundtrip(Rgba::WHITE);
        assert_eq!(Rgba::RED.to_hsva(), Hsva::new(0.0, 1.0, 1.0, 1.0));
        assert_eq!(Rgba::GREEN.to_hsva(), Hsva::new(120.0, 1.0, 1.0, 1.0));
        assert_eq!(Rgba::BLUE.to_hsva(), Hsva::new(240.0, 1.0, 1.0, 1.0));
    }
}
