use std::hash;
use std::ptr;

use crate::cairo::{
    ffi, utils::status_to_result, Antialias, Error, HintMetrics, HintStyle, SubpixelOrder,
};

#[derive(Debug)]
#[doc(alias = "cairo_font_options_t")]
pub struct FontOptions(ptr::NonNull<ffi::cairo_font_options_t>);

unsafe impl Send for FontOptions {}
unsafe impl Sync for FontOptions {}

impl FontOptions {
    #[doc(alias = "cairo_font_options_create")]
    pub fn new() -> Result<FontOptions, Error> {
        let font_options: FontOptions =
            unsafe { FontOptions::from_raw_full(ffi::cairo_font_options_create()) };

        let status = unsafe { ffi::cairo_font_options_status(font_options.to_raw_none()) };
        status_to_result(status)?;

        Ok(font_options)
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_options_t) -> Self {
        debug_assert!(!ptr.is_null());
        Self(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_options_t {
        self.0.as_ptr()
    }

    #[doc(alias = "cairo_font_options_merge")]
    pub fn merge(&mut self, other: &FontOptions) {
        unsafe { ffi::cairo_font_options_merge(self.to_raw_none(), other.to_raw_none()) }
    }

    #[doc(alias = "cairo_font_options_set_antialias")]
    pub fn set_antialias(&mut self, antialias: Antialias) {
        unsafe { ffi::cairo_font_options_set_antialias(self.to_raw_none(), antialias.into()) }
    }

    #[doc(alias = "cairo_font_options_get_antialias")]
    #[doc(alias = "get_antialias")]
    pub fn antialias(&self) -> Antialias {
        unsafe { Antialias::from(ffi::cairo_font_options_get_antialias(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_font_options_set_subpixel_order")]
    pub fn set_subpixel_order(&mut self, order: SubpixelOrder) {
        unsafe { ffi::cairo_font_options_set_subpixel_order(self.to_raw_none(), order.into()) }
    }

    #[doc(alias = "cairo_font_options_get_subpixel_order")]
    #[doc(alias = "get_subpixel_order")]
    pub fn subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            SubpixelOrder::from(ffi::cairo_font_options_get_subpixel_order(
                self.to_raw_none(),
            ))
        }
    }

    #[doc(alias = "cairo_font_options_set_hint_style")]
    pub fn set_hint_style(&mut self, hint_style: HintStyle) {
        unsafe { ffi::cairo_font_options_set_hint_style(self.to_raw_none(), hint_style.into()) }
    }

    #[doc(alias = "cairo_font_options_get_hint_style")]
    #[doc(alias = "get_hint_style")]
    pub fn hint_style(&self) -> HintStyle {
        unsafe { HintStyle::from(ffi::cairo_font_options_get_hint_style(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_font_options_set_hint_metrics")]
    pub fn set_hint_metrics(&mut self, hint_metrics: HintMetrics) {
        unsafe { ffi::cairo_font_options_set_hint_metrics(self.to_raw_none(), hint_metrics.into()) }
    }

    #[doc(alias = "cairo_font_options_get_hint_metrics")]
    #[doc(alias = "get_hint_metrics")]
    pub fn hint_metrics(&self) -> HintMetrics {
        unsafe { HintMetrics::from(ffi::cairo_font_options_get_hint_metrics(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_font_options_status")]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_font_options_status(self.to_raw_none()) };
        status_to_result(status)
    }
}

impl PartialEq for FontOptions {
    #[doc(alias = "cairo_font_options_equal")]
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe { ffi::cairo_font_options_equal(self.to_raw_none(), other.to_raw_none()).as_bool() }
    }
}

impl Eq for FontOptions {}

impl hash::Hash for FontOptions {
    #[doc(alias = "cairo_font_options_hash")]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        unsafe { hash::Hash::hash(&ffi::cairo_font_options_hash(self.to_raw_none()), state) }
    }
}

impl Drop for FontOptions {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_options_destroy(self.to_raw_none());
        }
    }
}

impl Clone for FontOptions {
    #[inline]
    fn clone(&self) -> FontOptions {
        unsafe { FontOptions::from_raw_full(ffi::cairo_font_options_copy(self.to_raw_none())) }
    }
}
