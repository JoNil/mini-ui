use std::ptr;
use crate::cairo::{ffi, utils::status_to_result, Error, RectangleInt, RegionOverlap};

#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "cairo_region_t")]
pub struct Region(ptr::NonNull<ffi::cairo_region_t>);

impl Clone for Region {
    #[inline]
    fn clone(&self) -> Region {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl Drop for Region {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_region_destroy(self.0.as_ptr());
        }
    }
}

impl PartialEq for Region {
    #[doc(alias = "cairo_region_equal")]
    #[inline]
    fn eq(&self, other: &Region) -> bool {
        unsafe { ffi::cairo_region_equal(self.0.as_ptr(), other.0.as_ptr()).as_bool() }
    }
}

impl Eq for Region {}

impl Region {
    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_region_t) -> Region {
        debug_assert!(!ptr.is_null());
        ffi::cairo_region_reference(ptr);
        Region(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_region_t) -> crate::cairo::Borrowed<Region> {
        debug_assert!(!ptr.is_null());
        crate::cairo::Borrowed::new(Region(ptr::NonNull::new_unchecked(ptr)))
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_region_t) -> Region {
        debug_assert!(!ptr.is_null());
        Region(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_region_t {
        self.0.as_ptr()
    }

    #[doc(alias = "cairo_region_create")]
    pub fn create() -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_create()) }
    }

    #[doc(alias = "cairo_region_create_rectangle")]
    pub fn create_rectangle(rectangle: &RectangleInt) -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_create_rectangle(rectangle.to_raw_none())) }
    }

    #[doc(alias = "cairo_region_create_rectangles")]
    pub fn create_rectangles(rectangles: &[RectangleInt]) -> Region {
        unsafe {
            Self::from_raw_full(ffi::cairo_region_create_rectangles(
                rectangles.as_ptr() as *mut ffi::cairo_rectangle_int_t,
                rectangles.len() as i32,
            ))
        }
    }

    #[doc(alias = "cairo_region_copy")]
    #[must_use]
    pub fn copy(&self) -> Region {
        unsafe { Self::from_raw_full(ffi::cairo_region_copy(self.0.as_ptr())) }
    }

    #[doc(alias = "get_extents")]
    #[doc(alias = "cairo_region_get_extents")]
    pub fn extents(&self, rectangle: &RectangleInt) {
        unsafe { ffi::cairo_region_get_extents(self.0.as_ptr(), rectangle.to_raw_none()) }
    }

    #[doc(alias = "cairo_region_num_rectangles")]
    pub fn num_rectangles(&self) -> i32 {
        unsafe { ffi::cairo_region_num_rectangles(self.0.as_ptr()) }
    }

    #[doc(alias = "get_rectangle")]
    #[doc(alias = "cairo_region_get_rectangle")]
    pub fn rectangle(&self, nth: i32) -> RectangleInt {
        unsafe {
            let rectangle: RectangleInt = ::std::mem::zeroed();
            ffi::cairo_region_get_rectangle(self.0.as_ptr(), nth, rectangle.to_raw_none());
            rectangle
        }
    }

    #[doc(alias = "cairo_region_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::cairo_region_is_empty(self.0.as_ptr()).as_bool() }
    }

    #[doc(alias = "cairo_region_contains_point")]
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::cairo_region_contains_point(self.0.as_ptr(), x, y).as_bool() }
    }

    #[doc(alias = "cairo_region_contains_rectangle")]
    pub fn contains_rectangle(&self, rectangle: &RectangleInt) -> RegionOverlap {
        unsafe {
            RegionOverlap::from(ffi::cairo_region_contains_rectangle(
                self.0.as_ptr(),
                rectangle.to_raw_none(),
            ))
        }
    }

    #[doc(alias = "cairo_region_translate")]
    pub fn translate(&self, dx: i32, dy: i32) {
        unsafe { ffi::cairo_region_translate(self.0.as_ptr(), dx, dy) }
    }

    #[doc(alias = "cairo_region_intersect")]
    pub fn intersect(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_intersect(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_intersect_rectangle")]
    pub fn intersect_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_intersect_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_subtract")]
    pub fn subtract(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_subtract(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_subtract_rectangle")]
    pub fn subtract_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_subtract_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_union")]
    pub fn union(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_union(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_union_rectangle")]
    pub fn union_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status =
                ffi::cairo_region_union_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_xor")]
    pub fn xor(&self, other: &Region) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_xor(self.0.as_ptr(), other.0.as_ptr());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_xor_rectangle")]
    pub fn xor_rectangle(&self, rectangle: &RectangleInt) -> Result<(), Error> {
        unsafe {
            let status = ffi::cairo_region_xor_rectangle(self.0.as_ptr(), rectangle.to_raw_none());
            status_to_result(status)
        }
    }

    #[doc(alias = "cairo_region_status")]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_region_status(self.0.as_ptr()) };
        status_to_result(status)
    }
}
