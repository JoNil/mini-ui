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

    /// Attach user data to `self` for the given `key`.
    pub fn set_user_data<T: 'static>(
        &self,
        key: &'static crate::cairo::UserDataKey<T>,
        value: std::rc::Rc<T>,
    ) -> Result<(), crate::cairo::Error> {
        unsafe extern "C" fn destructor<T>(ptr: *mut std::ffi::c_void) {
            let ptr: *const T = ptr as _;
            drop(std::rc::Rc::from_raw(ptr))
        }
        // Safety:
        //
        // The destructor’s cast and `from_raw` are symmetric
        // with the `into_raw` and cast below.
        // They both transfer ownership of one strong reference:
        // neither of them touches the reference count.
        let ptr: *const T = std::rc::Rc::into_raw(value);
        let ptr = ptr as *mut T as *mut std::ffi::c_void;
        let status = crate::cairo::utils::status_to_result(unsafe {
            ffi::cairo_font_face_set_user_data(
                self.to_raw_none(),
                &key.ffi,
                ptr,
                Some(destructor::<T>),
            )
        });

        if status.is_err() {
            // Safety:
            //
            // On errors the user data is leaked by cairo and needs to be freed here.
            unsafe {
                destructor::<T>(ptr);
            }
        }

        status
    }

    /// Return the user data previously attached to `self` with the given `key`, if any.
    pub fn user_data<T: 'static>(
        &self,
        key: &'static crate::cairo::UserDataKey<T>,
    ) -> Option<std::rc::Rc<T>> {
        let ptr = self.user_data_ptr(key)?.as_ptr();

        // Safety:
        //
        // `Rc::from_raw` would normally take ownership of a strong reference for this pointer.
        // But `self` still has a copy of that pointer and `get_user_data` can be called again
        // with the same key.
        // We use `ManuallyDrop` to avoid running the destructor of that first `Rc`,
        // and return a cloned one (which increments the reference count).
        unsafe {
            let rc = std::mem::ManuallyDrop::new(std::rc::Rc::from_raw(ptr));
            Some(std::rc::Rc::clone(&rc))
        }
    }

    /// Return the user data previously attached to `self` with the given `key`, if any,
    /// without incrementing the reference count.
    ///
    /// The pointer is valid when it is returned from this method,
    /// until the cairo object that `self` represents is destroyed
    /// or `remove_user_data` or `set_user_data` is called with the same key.
    pub fn user_data_ptr<T: 'static>(
        &self,
        key: &'static crate::cairo::UserDataKey<T>,
    ) -> Option<std::ptr::NonNull<T>> {
        // Safety:
        //
        // If `ffi_get_user_data` returns a non-null pointer,
        // there was a previous call to `ffi_set_user_data` with a key with the same address.
        // Either:
        //
        // * This was a call to a Rust `Self::set_user_data` method.
        //   Because that method takes a `&'static` reference,
        //   the key used then must live at that address until the end of the process.
        //   Because `UserDataKey<T>` has a non-zero size regardless of `T`,
        //   no other `UserDataKey<U>` value can have the same address.
        //   Therefore, the `T` type was the same then at it is now and `cast` is type-safe.
        //
        // * Or, it is technically possible that the `set` call was to the C function directly,
        //   with a `cairo_user_data_key_t` in heap-allocated memory that was then freed,
        //   then `Box::new(UserDataKey::new()).leak()` was used to create a `&'static`
        //   that happens to have the same address because the allocator for `Box`
        //   reused that memory region.
        //   Since this involves a C (or FFI) call *and* is so far out of “typical” use
        //   of the user data functionality, we consider this a misuse of an unsafe API.
        unsafe {
            let ptr = ffi::cairo_font_face_get_user_data(self.to_raw_none(), &key.ffi);
            Some(std::ptr::NonNull::new(ptr)?.cast())
        }
    }

    /// Unattached from `self` the user data associated with `key`, if any.
    /// If there is no other `Rc` strong reference, the data is destroyed.
    pub fn remove_user_data<T: 'static>(
        &self,
        key: &'static crate::cairo::UserDataKey<T>,
    ) -> Result<(), crate::cairo::Error> {
        let status = unsafe {
            ffi::cairo_font_face_set_user_data(
                self.to_raw_none(),
                &key.ffi,
                std::ptr::null_mut(),
                None,
            )
        };
        crate::cairo::utils::status_to_result(status)
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
