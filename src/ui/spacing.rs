use crate::math::{vec2, Vec2};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Spacing {
    pub const ZERO: Spacing = Spacing {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: 0.0,
    };

    #[must_use]
    #[inline]
    pub fn symmetrical(value: f32) -> Spacing {
        Spacing {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    #[must_use]
    #[inline]
    pub fn x(value: f32) -> Spacing {
        Spacing {
            top: 0.0,
            bottom: 0.0,
            left: value,
            right: value,
        }
    }

    #[must_use]
    #[inline]
    pub fn y(value: f32) -> Spacing {
        Spacing {
            top: value,
            bottom: value,
            left: 0.0,
            right: 0.0,
        }
    }

    #[must_use]
    #[inline]
    pub fn with_top(&self, top: f32) -> Spacing {
        Spacing {
            top,
            bottom: self.top,
            left: self.left,
            right: self.right,
        }
    }

    #[must_use]
    #[inline]
    pub fn with_bottom(&self, bottom: f32) -> Spacing {
        Spacing {
            top: self.top,
            bottom,
            left: self.left,
            right: self.right,
        }
    }

    #[must_use]
    #[inline]
    pub fn with_left(&self, left: f32) -> Spacing {
        Spacing {
            top: self.top,
            bottom: self.bottom,
            left,
            right: self.right,
        }
    }

    #[must_use]
    #[inline]
    pub fn with_right(&self, right: f32) -> Spacing {
        Spacing {
            top: self.top,
            bottom: self.bottom,
            left: self.left,
            right,
        }
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.left + self.right
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.top + self.bottom
    }

    #[inline]
    pub fn size(&self) -> Vec2 {
        vec2(self.width(), self.height())
    }
}

pub fn bounding_box(content_box: Vec2, margin: Spacing, padding: Spacing) -> Spacing {
    content_box + margin + padding
}

impl Add for Spacing {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Spacing) -> Spacing {
        Spacing {
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
            right: self.right + rhs.right,
        }
    }
}

impl Sub for Spacing {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Spacing) -> Spacing {
        Spacing {
            top: self.top - rhs.top,
            bottom: self.bottom - rhs.bottom,
            left: self.left - rhs.left,
            right: self.right - rhs.right,
        }
    }
}

impl Mul<Spacing> for f32 {
    type Output = Spacing;

    #[inline]
    fn mul(self, spacing: Spacing) -> Spacing {
        Spacing {
            top: spacing.top * self,
            bottom: spacing.bottom * self,
            left: spacing.left * self,
            right: spacing.right * self,
        }
    }
}

impl Mul<f32> for Spacing {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            top: self.top * scalar,
            bottom: self.bottom * scalar,
            left: self.left * scalar,
            right: self.right * scalar,
        }
    }
}

impl MulAssign<f32> for Spacing {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.top *= scalar;
        self.bottom *= scalar;
        self.left *= scalar;
        self.right *= scalar;
    }
}

impl Div<f32> for Spacing {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            top: self.top / scalar,
            bottom: self.bottom / scalar,
            left: self.left / scalar,
            right: self.right / scalar,
        }
    }
}

impl DivAssign<f32> for Spacing {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.top /= scalar;
        self.bottom /= scalar;
        self.left /= scalar;
        self.right /= scalar;
    }
}

impl Add<Spacing> for Vec2 {
    type Output = Spacing;

    #[inline]
    fn add(self, spacing: Spacing) -> Spacing {
        Spacing {
            top: spacing.top + self.y / 2.0,
            bottom: spacing.bottom + self.y / 2.0,
            left: spacing.left + self.x / 2.0,
            right: spacing.right + self.x / 2.0,
        }
    }
}

impl Add<Vec2> for Spacing {
    type Output = Self;

    #[inline]
    fn add(self, vec: Vec2) -> Self::Output {
        Self {
            top: self.top + vec.y / 2.0,
            bottom: self.bottom + vec.y / 2.0,
            left: self.left + vec.x / 2.0,
            right: self.right + vec.x / 2.0,
        }
    }
}

impl Mul<Spacing> for Vec2 {
    type Output = Spacing;

    #[inline]
    fn mul(self, spacing: Spacing) -> Spacing {
        Spacing {
            top: spacing.top * self.y,
            bottom: spacing.bottom * self.y,
            left: spacing.left * self.x,
            right: spacing.right * self.x,
        }
    }
}

impl Mul<Vec2> for Spacing {
    type Output = Self;

    #[inline]
    fn mul(self, vec: Vec2) -> Self::Output {
        Self {
            top: self.top * vec.y,
            bottom: self.bottom * vec.y,
            left: self.left * vec.x,
            right: self.right * vec.x,
        }
    }
}

impl MulAssign<Vec2> for Spacing {
    #[inline]
    fn mul_assign(&mut self, vec: Vec2) {
        self.top *= vec.y;
        self.bottom *= vec.y;
        self.left *= vec.x;
        self.right *= vec.x;
    }
}

impl Div<Vec2> for Spacing {
    type Output = Self;

    #[inline]
    fn div(self, vec: Vec2) -> Self::Output {
        Self {
            top: self.top / vec.y,
            bottom: self.bottom / vec.y,
            left: self.left / vec.x,
            right: self.right / vec.x,
        }
    }
}

impl DivAssign<Vec2> for Spacing {
    #[inline]
    fn div_assign(&mut self, vec: Vec2) {
        self.top /= vec.y;
        self.bottom /= vec.y;
        self.left /= vec.x;
        self.right /= vec.x;
    }
}
