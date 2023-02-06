// From https://github.com/michaelfairley/ezing

use std::f32::consts::{FRAC_PI_2, PI};

#[inline]
fn powf(x: f32, exp: f32) -> f32 {
    (x as f64).powf(exp as f64) as f32
}

#[inline]
pub fn linear(t: f32) -> f32 {
    t
}

#[inline]
pub fn quad_in(t: f32) -> f32 {
    t * t
}

#[inline]
pub fn quad_out(t: f32) -> f32 {
    -t * (t - (2.0))
}

#[inline]
pub fn quad_inout(t: f32) -> f32 {
    if t < (0.5) {
        (2.0) * t * t
    } else {
        ((-2.0) * t * t) + ((4.0) * t) - (1.0)
    }
}

#[inline]
pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

#[inline]
pub fn cubic_out(t: f32) -> f32 {
    let f = t - (1.0);
    f * f * f + (1.0)
}

#[inline]
pub fn cubic_inout(t: f32) -> f32 {
    if t < (0.5) {
        (4.0) * t * t * t
    } else {
        let f = ((2.0) * t) - (2.0);
        (0.5) * f * f * f + (1.0)
    }
}

#[inline]
pub fn quart_in(t: f32) -> f32 {
    t * t * t * t
}

#[inline]
pub fn quart_out(t: f32) -> f32 {
    let f = t - (1.0);
    f * f * f * ((1.0) - t) + (1.0)
}

#[inline]
pub fn quart_inout(t: f32) -> f32 {
    if t < (0.5) {
        (8.0) * t * t * t * t
    } else {
        let f = t - (1.0);
        (-8.0) * f * f * f * f + (1.0)
    }
}

#[inline]
pub fn quint_in(t: f32) -> f32 {
    t * t * t * t * t
}

#[inline]
pub fn quint_out(t: f32) -> f32 {
    let f = t - (1.0);
    f * f * f * f * f + (1.0)
}

#[inline]
pub fn quint_inout(t: f32) -> f32 {
    if t < (0.5) {
        (16.0) * t * t * t * t * t
    } else {
        let f = ((2.0) * t) - (2.0);
        (0.5) * f * f * f * f * f + (1.0)
    }
}

// Sine

#[inline]
pub fn sine_in(t: f32) -> f32 {
    ((t - (1.0)) * (FRAC_PI_2)).sin() + (1.0)
}

#[inline]
pub fn sine_out(t: f32) -> f32 {
    (t * (FRAC_PI_2)).sin()
}

#[inline]
pub fn sine_inout(t: f32) -> f32 {
    (0.5) * ((1.0) - (t * (PI)).cos())
}

// Circular

#[inline]
pub fn circ_in(t: f32) -> f32 {
    (1.0) - ((1.0) - t * t).sqrt()
}

#[inline]
pub fn circ_out(t: f32) -> f32 {
    (((2.0) - t) * t).sqrt()
}

#[inline]
pub fn circ_inout(t: f32) -> f32 {
    if t < (0.5) {
        (0.5) * ((1.0) - ((1.0) - (4.0) * t * t).sqrt())
    } else {
        (0.5) * ((-((2.0) * t - (3.0)) * ((2.0) * t - (1.0))).sqrt() + (1.0))
    }
}

// Exponential

#[inline]
pub fn expo_in(t: f32) -> f32 {
    if t == (0.0) {
        0.0
    } else {
        powf(2.0, (10.0) * (t - (1.0)))
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_out(t: f32) -> f32 {
    if t == (1.0) {
        1.0
    } else {
        (1.0) - powf(2.0, (-10.0) * t)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
#[inline]
pub fn expo_inout(t: f32) -> f32 {
    if t == (0.0) {
        0.0
    } else if t == (1.0) {
        1.0
    } else if t < (0.5) {
        (0.5) * powf(2.0, (20.0) * t - (10.0))
    } else {
        (-0.5) * powf(2.0, (-20.0) * t + (10.0)) + (1.0)
    }
}

// Elastic

#[inline]
pub fn elastic_in(t: f32) -> f32 {
    ((13.0) * (FRAC_PI_2) * t).sin() * powf(2.0, (10.0) * (t - (1.0)))
}

#[inline]
pub fn elastic_out(t: f32) -> f32 {
    ((-13.0) * (FRAC_PI_2) * (t + (1.0))).sin() * powf(2.0, (-10.0) * t) + (1.0)
}

#[inline]
pub fn elastic_inout(t: f32) -> f32 {
    if t < (0.5) {
        (0.5) * ((13.0) * (FRAC_PI_2) * (2.0) * t).sin() * powf(2.0, (10.0) * ((2.0) * t - (1.0)))
    } else {
        (0.5)
            * (((-13.0) * (FRAC_PI_2) * (2.0) * t).sin() * powf(2.0, (-10.0) * ((2.0) * t - (1.0)))
                + (2.0))
    }
}

// Back

#[inline]
pub fn back_in(t: f32) -> f32 {
    t * t * t - t * (t * (PI)).sin()
}

#[inline]
pub fn back_out(t: f32) -> f32 {
    let f = (1.0) - t;
    (1.0) - f * f * f + f * (f * (PI)).sin()
}

#[inline]
pub fn back_inout(t: f32) -> f32 {
    if t < (0.5) {
        let f = (2.0) * t;
        (0.5) * (f * f * f - f * (f * (PI)).sin())
    } else {
        let f = (2.0) - (2.0) * t;
        (0.5) * ((1.0) - (f * f * f - f * (f * (PI)).sin())) + (0.5)
    }
}

// Bounce

#[inline]
pub fn bounce_in(t: f32) -> f32 {
    (1.0) - bounce_out((1.0) - t)
}

#[inline]
pub fn bounce_out(t: f32) -> f32 {
    if t < (4.0 / 11.0) {
        (121.0 / 16.0) * t * t
    } else if t < (8.0 / 11.0) {
        (363.0 / 40.0) * t * t - (99.0 / 10.0) * t + (17.0 / 5.0)
    } else if t < (9.0 / 10.0) {
        (4356.0 / 361.0) * t * t - (35442.0 / 1805.0) * t + (16061.0 / 1805.0)
    } else {
        (54.0 / 5.0) * t * t - (513.0 / 25.0) * t + (268.0 / 25.0)
    }
}

#[inline]
pub fn bounce_inout(t: f32) -> f32 {
    if t < (0.5) {
        (0.5) * bounce_in(t * (2.0))
    } else {
        (0.5) * bounce_out(t * (2.0) - (1.0)) + (0.5)
    }
}

#[derive(Clone, Copy)]
pub enum Ease {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    CircIn,
    CircOut,
    CircInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

impl Default for Ease {
    fn default() -> Self {
        Self::Linear
    }
}

impl Ease {
    #[rustfmt::skip]
    pub fn map(&self, x: f32) -> f32 {
        match *self {
            Ease::Linear        => linear(x),
            Ease::QuadIn        => quad_in(x),
            Ease::QuadOut       => quad_out(x),
            Ease::QuadInOut     => quad_inout(x),
            Ease::CubicIn       => cubic_in(x),
            Ease::CubicOut      => cubic_out(x),
            Ease::CubicInOut    => cubic_inout(x),
            Ease::QuartIn       => quart_in(x),
            Ease::QuartOut      => quart_out(x),
            Ease::QuartInOut    => quart_inout(x),
            Ease::QuintIn       => quint_in(x),
            Ease::QuintOut      => quint_out(x),
            Ease::QuintInOut    => quint_inout(x),
            Ease::SineIn        => sine_in(x),
            Ease::SineOut       => sine_out(x),
            Ease::SineInOut     => sine_inout(x),
            Ease::CircIn        => circ_in(x),
            Ease::CircOut       => circ_out(x),
            Ease::CircInOut     => circ_inout(x),
            Ease::ExpoIn        => expo_in(x),
            Ease::ExpoOut       => expo_out(x),
            Ease::ExpoInOut     => expo_inout(x),
            Ease::ElasticIn     => elastic_in(x),
            Ease::ElasticOut    => elastic_out(x),
            Ease::ElasticInOut  => elastic_inout(x),
            Ease::BackIn        => back_in(x),
            Ease::BackOut       => back_out(x),
            Ease::BackInOut     => back_inout(x),
            Ease::BounceIn      => bounce_in(x),
            Ease::BounceOut     => bounce_out(x),
            Ease::BounceInOut   => bounce_inout(x),
        }
    }
}
