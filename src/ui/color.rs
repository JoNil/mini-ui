use crate::math::{vec4, Vec4};
use std::mem;

/// Convert rgb floats ([0-1],[0-1],[0-1]) to hsv floats ([0-1],[0-1],[0-1]), from Foley & van Dam p592
/// Optimized http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv
#[inline]
pub fn rgb_to_hsv(rgb: Vec4) -> Vec4 {
    let mut r = rgb.x;
    let mut g = rgb.y;
    let mut b = rgb.z;

    let mut k = 0.0;
    if g < b {
        mem::swap(&mut g, &mut b);
        k = -1.0;
    }
    if r < g {
        mem::swap(&mut r, &mut g);
        k = -2.0 / 6.0 - k;
    }

    let chroma = r - if g < b { g } else { b };
    let h = (k + (g - b) / (6.0 * chroma + 1e-20)).abs();
    let s = chroma / (r + 1e-20);
    let v = r;

    vec4(h, s, v, rgb.w)
}

/// Convert hsv floats ([0-1],[0-1],[0-1]) to rgb floats ([0-1],[0-1],[0-1]), from Foley & van Dam p593
/// also http://en.wikipedia.org/wiki/HSL_and_HSV
#[inline]
pub fn hsv_to_rgb(hsv: Vec4) -> Vec4 {
    let mut h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    if s == 0.0 {
        // gray
        return vec4(v, v, v, hsv.w);
    }

    h = h.rem_euclid(1.0) / (60.0 / 360.0);
    let i = h as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    vec4(r, g, b, hsv.w)
}

#[inline]
pub fn hover_color(color: Vec4) -> Vec4 {
    let mut hsv = rgb_to_hsv(color);

    if hsv.z > 0.5 {
        hsv.z = (hsv.z - 0.05).rem_euclid(1.0);
    } else {
        hsv.z = (hsv.z + 0.05).rem_euclid(1.0);
    }

    hsv_to_rgb(hsv)
}

#[inline]
pub fn held_color(color: Vec4) -> Vec4 {
    let mut hsv = rgb_to_hsv(color);

    if hsv.z > 0.5 {
        hsv.z = (hsv.z - 0.1).rem_euclid(1.0);
    } else {
        hsv.z = (hsv.z + 0.1).rem_euclid(1.0);
    }

    hsv_to_rgb(hsv)
}
