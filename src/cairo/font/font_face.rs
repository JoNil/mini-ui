use crate::cairo::{ffi, utils::status_to_result, Error, FontSlant, FontType, FontWeight};
use std::{
    ffi::{CStr, CString},
    ptr,
};

#[derive(Debug)]
#[doc(alias = "cairo_font_face_t")]
pub struct FontFace(ptr::NonNull<ffi::cairo_font_face_t>);

impl FontFace {
    #[doc(alias = "cairo_toy_font_face_create")]
    pub fn toy_create(
        family: &str,
        slant: FontSlant,
        weight: FontWeight,
    ) -> Result<FontFace, Error> {
        let font_face: FontFace = unsafe {
            let family = CString::new(family).unwrap();
            FontFace::from_raw_full(ffi::cairo_toy_font_face_create(
                family.as_ptr(),
                slant.into(),
                weight.into(),
            ))
        };
        let status = unsafe { ffi::cairo_font_face_status(font_face.to_raw_none()) };
        status_to_result(status)?;

        Ok(font_face)
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        debug_assert!(!ptr.is_null());
        FontFace(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        debug_assert!(!ptr.is_null());
        FontFace(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.0.as_ptr()
    }

    #[doc(alias = "cairo_toy_font_face_get_family")]
    pub fn toy_get_family(&self) -> Option<String> {
        unsafe { to_optional_string(ffi::cairo_toy_font_face_get_family(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_toy_font_face_get_slant")]
    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe { FontSlant::from(ffi::cairo_toy_font_face_get_slant(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_toy_font_face_get_weight")]
    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe { FontWeight::from(ffi::cairo_toy_font_face_get_weight(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_font_face_get_type")]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> FontType {
        unsafe { FontType::from(ffi::cairo_font_face_get_type(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_font_face_get_reference_count")]
    #[doc(alias = "get_reference_count")]
    pub fn reference_count(&self) -> usize {
        unsafe { ffi::cairo_font_face_get_reference_count(self.to_raw_none()) as usize }
    }

    #[doc(alias = "cairo_font_face_status")]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_font_face_status(self.to_raw_none()) };
        status_to_result(status)
    }

    user_data_methods! {
        ffi::cairo_font_face_get_user_data,
        ffi::cairo_font_face_set_user_data,
    }
}

impl Drop for FontFace {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_face_destroy(self.to_raw_none());
        }
    }
}

impl Clone for FontFace {
    #[inline]
    fn clone(&self) -> FontFace {
        unsafe { FontFace::from_raw_none(self.to_raw_none()) }
    }
}

pub(crate) unsafe fn to_optional_string(str: *const std::ffi::c_char) -> Option<String> {
    if str.is_null() {
        None
    } else {
        Some(String::from_utf8_lossy(CStr::from_ptr(str).to_bytes()).into_owned())
    }
}
