use glam::*;
use std::ops::{self, Range};

// -------------------------------
// 1D

#[derive(Debug, Default, PartialEq, Hash, Eq, Clone, Copy)]
pub struct IRange1 {
    pub min: i32,
    pub max: i32,
}

impl IRange1 {
    pub const ZERO: Self = Self {
        min: 0,
        max: 0,
    };

    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn sized(min: i32, size: i32) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: i32, size: i32) -> Self {
        Self::sized(center - size / 2, size)
    }

    pub fn from_range(x: Range<i32>) -> Self {
        Self {
            min: x.start,
            max: x.end,
        }
    }

    pub fn as_range(&self) -> Range<i32> {
        self.min..self.max
    }

    pub fn centered_pos(&self, size: i32) -> i32 {
        (self.size() - size) / 2 + self.min
    }

    pub fn contains(&self, p: i32) -> bool {
        p >= self.min && p < self.max
    }

    pub fn is_empty(&self) -> bool {
        self.min >= self.max
    }

    pub fn is_negative(&self) -> bool {
        self.min > self.max
    }

    pub fn size(&self) -> i32 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = i32> {
        self.as_range().into_iter()
    }

    pub fn expand(&self, margin: i32) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }
}

impl ops::BitAnd<Self> for IRange1 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<Self> for IRange1 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<i32> for IRange1 {
    type Output = Self;
    fn add(self, rhs: i32) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<i32> for IRange1 {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<i32> for IRange1 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<i32> for IRange1 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Range1 {
    pub min: f32,
    pub max: f32,
}

impl Range1 {
    pub const ZERO: Self = Self {
        min: 0.0,
        max: 0.0,
    };

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn sized(min: f32, size: f32) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: f32, size: f32) -> Self {
        Self::sized(center - size * 0.5, size)
    }

    pub fn from_range(x: Range<f32>) -> Self {
        Self {
            min: x.start,
            max: x.end,
        }
    }

    pub fn as_range(&self) -> Range<f32> {
        self.min..self.max
    }

    pub fn centered_pos(&self, size: f32) -> f32 {
        (self.size() - size) * 0.5 + self.min
    }

    pub fn lerp(&self, t: f32) -> f32 {
        self.min + self.size() * t
    }

    pub fn inv_lerp(&self, t: f32) -> f32 {
        (t - self.min) / self.size()
    }
    
    pub fn contains(&self, p: f32) -> bool {
        p >= self.min && p < self.max
    }

    pub fn is_empty(&self) -> bool {
        self.min >= self.max
    }

    pub fn is_negative(&self) -> bool {
        self.min > self.max
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn expand(&self, margin: f32) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }
}

impl ops::BitAnd<Self> for Range1 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<Self> for Range1 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<f32> for Range1 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<f32> for Range1 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<f32> for Range1 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<f32> for Range1 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

// -------------------------------
// 2D

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IRange2 {
    pub min: IVec2,
    pub max: IVec2,
}

pub struct IRangeIter2 {
    range: IRange2,
    pos: IVec2,
}

impl Iterator for IRangeIter2 {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x < self.range.max.x {
            self.pos.x += 1;
        } else {
            self.pos.x = self.range.min.x;
            if self.pos.y < self.range.max.y {
                self.pos.y += 1;
            } else {
                return None;
            }
        }
        Some(self.pos)
    }
}

impl IRange2 {
    pub const ZERO: Self = Self {
        min: IVec2::ZERO,
        max: IVec2::ZERO,
    };

    pub fn new(min: IVec2, max: IVec2) -> Self {
        Self { min, max }
    }

    pub fn from_xy(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self::new(IVec2::new(x0, y0), IVec2::new(x1, y1))
    }

    pub fn from_vertices<I: Iterator<Item = IVec2>>(mut iter: I) -> Option<Self> {
        if let Some(v) = iter.next() {
            let mut r = Self::new(v, v);
            while let Some(v) = iter.next() {
                r = r.including(v);
            }
            Some(IRange2::new(r.min, r.max + IVec2::ONE))
        } else {
            None
        }
    }

    pub fn x0(&self) -> i32 {
        self.min.x
    }

    pub fn y0(&self) -> i32 {
        self.min.y
    }

    pub fn x1(&self) -> i32 {
        self.max.x
    }

    pub fn y1(&self) -> i32 {
        self.max.y
    }

    pub fn x_range(&self) -> IRange1 {
        IRange1::new(self.min.x, self.max.x)
    }

    pub fn y_range(&self) -> IRange1 {
        IRange1::new(self.min.y, self.max.y)
    }

    pub fn x0y0(&self) -> IVec2 {
        IVec2::new(self.min.x, self.min.y)
    }

    pub fn x1y0(&self) -> IVec2 {
        IVec2::new(self.max.x, self.min.y)
    }

    pub fn x0y1(&self) -> IVec2 {
        IVec2::new(self.min.x, self.max.y)
    }

    pub fn x1y1(&self) -> IVec2 {
        IVec2::new(self.max.x, self.max.y)
    }

    pub fn x(&self) -> Range<i32> {
        self.min.x..self.max.x
    }

    pub fn y(&self) -> Range<i32> {
        self.min.y..self.max.y
    }

    pub fn sized(min: IVec2, size: IVec2) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: IVec2, size: IVec2) -> Self {
        Self::sized(center - size / 2, size)
    }

    pub fn from_ranges(x: IRange1, y: IRange1) -> Self {
        Self {
            min: IVec2::new(x.min, y.min),
            max: IVec2::new(x.max, y.max),
        }
    }

    pub fn centered_pos(&self, size: IVec2) -> IVec2 {
        (self.size() - size) / 2 + self.min
    }

    pub fn as_range2(&self) -> Range2 {
        Range2::new(self.min.as_vec2(), self.max.as_vec2())
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }

    pub fn is_negative(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    pub fn contains(&self, p: IVec2) -> bool {
        p.x >= self.min.x && p.x < self.max.x && p.y >= self.min.y && p.y < self.max.y
    }

    pub fn size(&self) -> IVec2 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn iter(&self) -> IRangeIter2 {
        IRangeIter2 {
            range: Self::new(self.min, self.max - IVec2::ONE),
            pos: IVec2::new(self.min.x - 1, self.min.y),
        }
    }

    pub fn including(&self, p: IVec2) -> Self {
        Self::new(self.min.min(p), self.max.max(p))
    }

    pub fn expand(&self, margin: IVec2) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }
}

impl ops::BitAnd<Self> for IRange2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<Self> for IRange2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<IVec2> for IRange2 {
    type Output = Self;
    fn add(self, rhs: IVec2) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<IVec2> for IRange2 {
    type Output = Self;
    fn sub(self, rhs: IVec2) -> Self {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<IVec2> for IRange2 {
    type Output = Self;
    fn mul(self, rhs: IVec2) -> Self {
        Self {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<IVec2> for IRange2 {
    type Output = Self;
    fn div(self, rhs: IVec2) -> Self {
        Self {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Range2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Range2 {
    pub const ZERO: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::ZERO,
    };

    pub const ZERO_TO_ONE: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::ONE,
    };

    pub fn new(min: Vec2, max: Vec2) -> Range2 {
        Range2 { min, max }
    }

    pub fn from_x0y0x1y1(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new(Vec2::new(x0, y0), Vec2::new(x1, y1))
    }

    pub fn from_vertices<I: Iterator<Item = Vec2>>(mut iter: I) -> Option<Self> {
        if let Some(v) = iter.next() {
            let mut r = Self::new(v, v);
            while let Some(v) = iter.next() {
                r = r.including(v);
            }
            Some(r)
        } else {
            None
        }
    }

    pub fn x0(&self) -> f32 {
        self.min.x
    }

    pub fn y0(&self) -> f32 {
        self.min.y
    }

    pub fn x1(&self) -> f32 {
        self.max.x
    }

    pub fn y1(&self) -> f32 {
        self.max.y
    }

    pub fn x_range(&self) -> Range1 {
        Range1::new(self.min.x, self.max.x)
    }

    pub fn y_range(&self) -> Range1 {
        Range1::new(self.min.y, self.max.y)
    }

    pub fn x0y0(&self) -> Vec2 {
        Vec2::new(self.min.x, self.min.y)
    }

    pub fn x1y0(&self) -> Vec2 {
        Vec2::new(self.max.x, self.min.y)
    }

    pub fn x0y1(&self) -> Vec2 {
        Vec2::new(self.min.x, self.max.y)
    }

    pub fn x1y1(&self) -> Vec2 {
        Vec2::new(self.max.x, self.max.y)
    }

    pub fn x(&self) -> Range<f32> {
        self.min.x..self.max.x
    }

    pub fn y(&self) -> Range<f32> {
        self.min.y..self.max.y
    }

    pub fn center(&self) -> Vec2 {
        self.x0y0() * 0.5 + self.x1y1() * 0.5
    }

    pub fn sized(min: Vec2, size: Vec2) -> Range2 {
        Range2 {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: Vec2, size: Vec2) -> Self {
        Self::sized(center - size / 2.0, size)
    }

    pub fn lerp(&self, t: Vec2) -> Vec2 {
        self.min + self.size() * t
    }

    pub fn inv_lerp(&self, p: Vec2) -> Vec2 {
        (p - self.min) / self.size()
    }

    pub fn clamp(&self, p: Vec2) -> Vec2 {
        p.clamp(self.min, self.max)
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }

    pub fn is_negative(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x < self.max.x && p.y >= self.min.y && p.y < self.max.y
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn including(&self, p: Vec2) -> Self {
        Self::new(self.min.min(p), self.max.max(p))
    }

    pub fn expand(&self, margin: Vec2) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }
}

impl ops::BitAnd<Range2> for Range2 {
    type Output = Range2;
    fn bitand(self, rhs: Range2) -> Range2 {
        Range2 {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<Range2> for Range2 {
    type Output = Range2;
    fn bitor(self, rhs: Range2) -> Range2 {
        Range2 {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<Vec2> for Range2 {
    type Output = Range2;
    fn add(self, rhs: Vec2) -> Range2 {
        Range2 {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<Vec2> for Range2 {
    type Output = Range2;
    fn sub(self, rhs: Vec2) -> Range2 {
        Range2 {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<Vec2> for Range2 {
    type Output = Range2;
    fn mul(self, rhs: Vec2) -> Range2 {
        Range2 {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<Vec2> for Range2 {
    type Output = Range2;
    fn div(self, rhs: Vec2) -> Range2 {
        Range2 {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

// -------------------------------
// 3D

#[derive(Debug, Default, PartialEq, Hash, Eq, Clone, Copy)]
pub struct IRange3 {
    pub min: IVec3,
    pub max: IVec3,
}

pub struct IRangeIter3 {
    range: IRange3,
    pos: IVec3,
}

impl Iterator for IRangeIter3 {
    type Item = IVec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x < self.range.max.x {
            self.pos.x += 1;
        } else {
            self.pos.x = self.range.min.x;
            if self.pos.y < self.range.max.y {
                self.pos.y += 1;
            } else {
                self.pos.y = self.range.min.y;
                if self.pos.z < self.range.max.z {
                    self.pos.z += 1;
                } else {
                    return None;
                }
            }
        }
        Some(self.pos)
    }
}

impl IRange3 {
    pub const ZERO: Self = Self {
        min: IVec3::ZERO,
        max: IVec3::ZERO,
    };

    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn sized(min: IVec3, size: IVec3) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: IVec3, size: IVec3) -> Self {
        Self::sized(center - size / 2, size)
    }

    pub fn from_xyz(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) -> Self {
        Self::new(IVec3::new(x0, y0, z0), IVec3::new(x1, y1, z1))
    }

    pub fn from_ranges(x: IRange1, y: IRange1, z: IRange1) -> Self {
        Self {
            min: IVec3::new(x.min, y.min, z.min),
            max: IVec3::new(x.max, y.max, z.max),
        }
    }

    pub fn from_vertices<I: Iterator<Item = IVec3>>(mut iter: I) -> Option<Self> {
        if let Some(v) = iter.next() {
            let mut r = Self::new(v, v);
            while let Some(v) = iter.next() {
                r = r.including(v);
            }
            Some(r)
        } else {
            None
        }
    }

    pub fn x0(&self) -> i32 {
        self.min.x
    }

    pub fn y0(&self) -> i32 {
        self.min.y
    }

    pub fn z0(&self) -> i32 {
        self.min.z
    }

    pub fn x1(&self) -> i32 {
        self.max.x
    }

    pub fn y1(&self) -> i32 {
        self.max.y
    }

    pub fn z1(&self) -> i32 {
        self.max.z
    }

    pub fn x_range(&self) -> IRange1 {
        IRange1::new(self.min.x, self.max.x)
    }

    pub fn y_range(&self) -> IRange1 {
        IRange1::new(self.min.y, self.max.y)
    }

    pub fn z_range(&self) -> IRange1 {
        IRange1::new(self.min.z, self.max.z)
    }

    pub fn x0y0z0(&self) -> IVec3 {
        IVec3::new(self.min.x, self.min.y, self.min.z)
    }

    pub fn x1y0z0(&self) -> IVec3 {
        IVec3::new(self.max.x, self.min.y, self.min.z)
    }

    pub fn x0y1z0(&self) -> IVec3 {
        IVec3::new(self.min.x, self.max.y, self.min.z)
    }

    pub fn x1y1z0(&self) -> IVec3 {
        IVec3::new(self.max.x, self.max.y, self.min.z)
    }

    pub fn x0y0z1(&self) -> IVec3 {
        IVec3::new(self.min.x, self.min.y, self.max.z)
    }

    pub fn x1y0z1(&self) -> IVec3 {
        IVec3::new(self.max.x, self.min.y, self.max.z)
    }

    pub fn x0y1z1(&self) -> IVec3 {
        IVec3::new(self.min.x, self.max.y, self.max.z)
    }

    pub fn x1y1z1(&self) -> IVec3 {
        IVec3::new(self.max.x, self.max.y, self.max.z)
    }

    pub fn x(&self) -> Range<i32> {
        self.min.x..self.max.x
    }

    pub fn y(&self) -> Range<i32> {
        self.min.y..self.max.y
    }

    pub fn z(&self) -> Range<i32> {
        self.min.z..self.max.z
    }

    pub fn xy(&self) -> IRange2 {
        IRange2::new(self.min.xy(), self.max.xy())
    }

    pub fn vertices(&self) -> [IVec3; 8] {
        [
            self.x0y0z0(),
            self.x1y0z0(),
            self.x0y1z0(),
            self.x1y1z0(),
            self.x0y0z1(),
            self.x1y0z1(),
            self.x0y1z1(),
            self.x1y1z1(),
        ]
    }

    pub fn as_range3(&self) -> Range3 {
        Range3::new(self.min.as_vec3(), self.max.as_vec3())
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    pub fn is_negative(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    pub fn contains(&self, p: IVec3) -> bool {
        p.x >= self.min.x
            && p.x < self.max.x
            && p.y >= self.min.y
            && p.y < self.max.y
            && p.z >= self.min.z
            && p.z < self.max.z
    }

    pub fn size(&self) -> IVec3 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn including(&self, p: IVec3) -> Self {
        IRange3::new(self.min.min(p), self.max.max(p))
    }

    pub fn expand(&self, margin: IVec3) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }

    pub fn iter(&self) -> IRangeIter3 {
        let r = if self.is_empty() { Self::ZERO } else { *self };
        IRangeIter3 {
            range: Self::new(r.min, r.max - IVec3::ONE),
            pos: IVec3::new(r.min.x - 1, r.min.y, r.min.z),
        }
    }
}

impl ops::BitAnd<IRange3> for IRange3 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<IRange3> for IRange3 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<IVec3> for IRange3 {
    type Output = IRange3;
    fn add(self, rhs: IVec3) -> Self {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<IVec3> for IRange3 {
    type Output = Self;
    fn sub(self, rhs: IVec3) -> Self {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<IVec3> for IRange3 {
    type Output = IRange3;
    fn mul(self, rhs: IVec3) -> IRange3 {
        IRange3 {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<IVec3> for IRange3 {
    type Output = IRange3;
    fn div(self, rhs: IVec3) -> IRange3 {
        IRange3 {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Range3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Range3 {
    pub const ZERO: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };

    pub const ZERO_TO_ONE: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ONE,
    };

    pub fn new(min: Vec3, max: Vec3) -> Range3 {
        Range3 { min, max }
    }

    pub fn sized(min: Vec3, size: Vec3) -> Range3 {
        Range3 {
            min,
            max: min + size,
        }
    }

    pub fn centered(center: Vec3, size: Vec3) -> Self {
        Self::sized(center - size / 2.0, size)
    }

    pub fn from_xyz(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Self::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1))
    }

    pub fn from_vertices<I: Iterator<Item = Vec3>>(mut iter: I) -> Option<Self> {
        if let Some(v) = iter.next() {
            let mut r = Self::new(v, v);
            while let Some(v) = iter.next() {
                r = r.including(v);
            }
            Some(r)
        } else {
            None
        }
    }

    pub fn x0(&self) -> f32 {
        self.min.x
    }

    pub fn y0(&self) -> f32 {
        self.min.y
    }

    pub fn z0(&self) -> f32 {
        self.min.z
    }

    pub fn x1(&self) -> f32 {
        self.max.x
    }

    pub fn y1(&self) -> f32 {
        self.max.y
    }

    pub fn z1(&self) -> f32 {
        self.max.z
    }

    pub fn x0y0z0(&self) -> Vec3 {
        Vec3::new(self.min.x, self.min.y, self.min.z)
    }

    pub fn x1y0z0(&self) -> Vec3 {
        Vec3::new(self.max.x, self.min.y, self.min.z)
    }

    pub fn x0y1z0(&self) -> Vec3 {
        Vec3::new(self.min.x, self.max.y, self.min.z)
    }

    pub fn x1y1z0(&self) -> Vec3 {
        Vec3::new(self.max.x, self.max.y, self.min.z)
    }

    pub fn x0y0z1(&self) -> Vec3 {
        Vec3::new(self.min.x, self.min.y, self.max.z)
    }

    pub fn x1y0z1(&self) -> Vec3 {
        Vec3::new(self.max.x, self.min.y, self.max.z)
    }

    pub fn x0y1z1(&self) -> Vec3 {
        Vec3::new(self.min.x, self.max.y, self.max.z)
    }

    pub fn x1y1z1(&self) -> Vec3 {
        Vec3::new(self.max.x, self.max.y, self.max.z)
    }

    pub fn xy(&self) -> Range2 {
        Range2::new(self.min.xy(), self.max.xy())
    }

    pub fn lerp(&self, t: Vec3) -> Vec3 {
        self.min + self.size() * t
    }

    pub fn inv_lerp(&self, p: Vec3) -> Vec3 {
        (p - self.min) / self.size()
    }
    
    pub fn clamp(&self, p: Vec3) -> Vec3 {
        p.clamp(self.min, self.max)
    }

    pub fn contains(&self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.x < self.max.x
            && p.y >= self.min.y
            && p.y < self.max.y
            && p.z >= self.min.z
            && p.z < self.max.z
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn ordered(&self) -> Range3 {
        Range3 {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn including(&self, p: Vec3) -> Self {
        Range3::new(self.min.min(p), self.max.max(p))
    }
}

impl ops::BitAnd<Range3> for Range3 {
    type Output = Range3;
    fn bitand(self, rhs: Range3) -> Range3 {
        Range3 {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }
}

impl ops::BitOr<Range3> for Range3 {
    type Output = Range3;
    fn bitor(self, rhs: Range3) -> Range3 {
        Range3 {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}

impl ops::Add<Vec3> for Range3 {
    type Output = Range3;
    fn add(self, rhs: Vec3) -> Range3 {
        Range3 {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl ops::Sub<Vec3> for Range3 {
    type Output = Range3;
    fn sub(self, rhs: Vec3) -> Range3 {
        Range3 {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl ops::Mul<Vec3> for Range3 {
    type Output = Range3;
    fn mul(self, rhs: Vec3) -> Range3 {
        Range3 {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl ops::Div<Vec3> for Range3 {
    type Output = Range3;
    fn div(self, rhs: Vec3) -> Range3 {
        Range3 {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_range2() {
        let r = IRange2::sized(IVec2::ZERO, IVec2::ONE * 2);
        let results = r.iter().collect::<Vec<_>>();
        //println!("{:#?}", results);
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn iter_range2_negative() {
        let r = IRange2::new(IVec2::new(-2, 0), IVec2::new(-32, 1));
        let results = r.iter().collect::<Vec<_>>();
        println!("{:#?}", results);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn iter_range3() {
        let r = IRange3::sized(IVec3::ZERO, IVec3::ONE * 2);
        let results = r.iter().collect::<Vec<_>>();
        //println!("{:#?}", results);
        assert_eq!(results.len(), 8);
    }

    #[test]
    fn iter_range3_negative() {
        let r = IRange3::new(IVec3::new(-2, -2, 0), IVec3::new(0, -32, 1));
        let results = r.iter().collect::<Vec<_>>();
        println!("{:#?}", results);
        assert_eq!(results.len(), 0);
    }
}
