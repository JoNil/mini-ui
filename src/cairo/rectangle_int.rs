use crate::cairo::ffi;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
#[doc(alias = "cairo_rectangle_int_t")]
pub struct RectangleInt(ffi::cairo_rectangle_int_t);

impl RectangleInt {
    #[inline]
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self(ffi::cairo_rectangle_int_t {
            x,
            y,
            width,
            height,
        })
    }
    #[inline]
    pub fn x(&self) -> i32 {
        self.0.x
    }
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.0.x = x;
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.0.y
    }
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.0.y = y;
    }
    #[inline]
    pub fn width(&self) -> i32 {
        self.0.width
    }
    #[inline]
    pub fn set_width(&mut self, width: i32) {
        self.0.width = width;
    }
    #[inline]
    pub fn height(&self) -> i32 {
        self.0.height
    }
    #[inline]
    pub fn set_height(&mut self, height: i32) {
        self.0.height = height;
    }
}

impl fmt::Debug for RectangleInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RectangleInt")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl RectangleInt {
    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_rectangle_int_t {
        &self.0 as *const ffi::cairo_rectangle_int_t as *mut ffi::cairo_rectangle_int_t
    }
}
