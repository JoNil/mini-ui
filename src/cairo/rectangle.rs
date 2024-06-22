use crate::cairo::ffi;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
#[doc(alias = "cairo_rectangle_t")]
pub struct Rectangle(ffi::cairo_rectangle_t);

impl Rectangle {
    #[inline]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self(ffi::cairo_rectangle_t {
            x,
            y,
            width,
            height,
        })
    }
    #[inline]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.0.y
    }
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }
    #[inline]
    pub fn width(&self) -> f64 {
        self.0.width
    }
    #[inline]
    pub fn set_width(&mut self, width: f64) {
        self.0.width = width;
    }
    #[inline]
    pub fn height(&self) -> f64 {
        self.0.height
    }
    #[inline]
    pub fn set_height(&mut self, height: f64) {
        self.0.height = height;
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl Rectangle {
    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_rectangle_t {
        &self.0 as *const ffi::cairo_rectangle_t as *mut ffi::cairo_rectangle_t
    }
}
