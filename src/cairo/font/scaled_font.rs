use crate::cairo::{
    ffi, utils::status_to_result, Error, FontExtents, FontFace, FontOptions, FontType, Glyph,
    Matrix, TextCluster, TextExtents,
};
use std::{ffi::CString, mem::MaybeUninit, ptr};

#[derive(Debug)]
#[doc(alias = "cairo_scaled_font_t")]
pub struct ScaledFont(ptr::NonNull<ffi::cairo_scaled_font_t>);

impl ScaledFont {
    #[doc(alias = "cairo_scaled_font_create")]
    pub fn new(
        font_face: &FontFace,
        font_matrix: &Matrix,
        ctm: &Matrix,
        options: &FontOptions,
    ) -> Result<ScaledFont, Error> {
        let scaled_font: ScaledFont = unsafe {
            ScaledFont::from_raw_full(ffi::cairo_scaled_font_create(
                font_face.to_raw_none(),
                font_matrix.ptr(),
                ctm.ptr(),
                options.to_raw_none(),
            ))
        };
        let status = unsafe { ffi::cairo_scaled_font_status(scaled_font.to_raw_none()) };
        status_to_result(status)?;

        Ok(scaled_font)
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_scaled_font_t {
        self.0.as_ptr()
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        debug_assert!(!ptr.is_null());
        ScaledFont(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_scaled_font_t) -> ScaledFont {
        debug_assert!(!ptr.is_null());
        ffi::cairo_scaled_font_reference(ptr);
        ScaledFont(ptr::NonNull::new_unchecked(ptr))
    }

    #[doc(alias = "cairo_scaled_font_get_type")]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> FontType {
        unsafe { FontType::from(ffi::cairo_scaled_font_get_type(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_scaled_font_get_reference_count")]
    #[doc(alias = "get_reference_count")]
    pub fn reference_count(&self) -> usize {
        unsafe { ffi::cairo_scaled_font_get_reference_count(self.to_raw_none()) as usize }
    }

    #[doc(alias = "cairo_scaled_font_extents")]
    pub fn extents(&self) -> FontExtents {
        let mut extents = MaybeUninit::<FontExtents>::uninit();

        unsafe {
            ffi::cairo_scaled_font_extents(self.to_raw_none(), extents.as_mut_ptr() as *mut _);
            extents.assume_init()
        }
    }

    #[doc(alias = "cairo_scaled_font_text_extents")]
    pub fn text_extents(&self, text: &str) -> TextExtents {
        let mut extents = MaybeUninit::<TextExtents>::uninit();

        let text = CString::new(text).unwrap();
        unsafe {
            ffi::cairo_scaled_font_text_extents(
                self.to_raw_none(),
                text.as_ptr(),
                extents.as_mut_ptr() as *mut _,
            );
            extents.assume_init()
        }
    }

    #[doc(alias = "cairo_scaled_font_glyph_extents")]
    pub fn glyph_extents(&self, glyphs: &[Glyph]) -> TextExtents {
        let mut extents = MaybeUninit::<TextExtents>::uninit();

        unsafe {
            ffi::cairo_scaled_font_glyph_extents(
                self.to_raw_none(),
                glyphs.as_ptr() as *const _,
                glyphs.len() as _,
                extents.as_mut_ptr() as *mut _,
            );
            extents.assume_init()
        }
    }

    #[doc(alias = "cairo_scaled_font_text_to_glyphs")]
    pub fn text_to_glyphs(
        &self,
        x: f64,
        y: f64,
        text: &str,
    ) -> Result<(Vec<Glyph>, Vec<TextCluster>), Error> {
        // This large unsafe block is due to the FFI function returning two specially allocated
        // (cairo_{glyph,text_cluster}_allocate) pointers that need to be copied into Vec<T>
        // types before they're of any use to Rust code.

        unsafe {
            let mut glyphs_ptr: *mut Glyph = ptr::null_mut();
            let mut glyph_count = 0i32;
            let mut clusters_ptr: *mut TextCluster = ptr::null_mut();
            let mut cluster_count = 0i32;
            let mut cluster_flags = 0i32;
            let text_length = text.len() as i32;
            let text = CString::new(text).unwrap();

            let status = ffi::cairo_scaled_font_text_to_glyphs(
                self.to_raw_none(),
                x,
                y,
                text.as_ptr(),
                text_length,
                &mut glyphs_ptr as *mut *mut Glyph as *mut _,
                &mut glyph_count,
                &mut clusters_ptr as *mut *mut TextCluster as *mut _,
                &mut cluster_count,
                &mut cluster_flags,
            );
            status_to_result(status)?;

            let glyph_count = glyph_count as usize;
            let glyphs: Vec<Glyph> = {
                let mut glyphs: Vec<Glyph> = Vec::with_capacity(glyph_count);

                ptr::copy(glyphs_ptr, glyphs.as_mut_ptr(), glyph_count);

                glyphs.set_len(glyph_count);

                glyphs
            };

            let cluster_count = cluster_count as usize;
            let clusters: Vec<TextCluster> = {
                let mut clusters = Vec::with_capacity(cluster_count);

                ptr::copy(clusters_ptr, clusters.as_mut_ptr(), cluster_count);

                clusters.set_len(cluster_count);

                clusters
            };

            ffi::cairo_glyph_free(glyphs_ptr as _);
            ffi::cairo_text_cluster_free(clusters_ptr as _);

            Ok((glyphs, clusters))
        }
    }

    #[doc(alias = "cairo_scaled_font_get_font_face")]
    #[doc(alias = "get_font_face")]
    pub fn font_face(&self) -> FontFace {
        unsafe { FontFace::from_raw_none(ffi::cairo_scaled_font_get_font_face(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_scaled_font_get_font_options")]
    #[doc(alias = "get_font_options")]
    pub fn font_options(&self) -> Result<FontOptions, Error> {
        let options = FontOptions::new()?;

        unsafe {
            ffi::cairo_scaled_font_get_font_options(self.to_raw_none(), options.to_raw_none())
        }

        Ok(options)
    }

    #[doc(alias = "cairo_scaled_font_get_font_matrix")]
    #[doc(alias = "get_font_matrix")]
    pub fn font_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe { ffi::cairo_scaled_font_get_font_matrix(self.to_raw_none(), matrix.mut_ptr()) }

        matrix
    }

    #[doc(alias = "cairo_scaled_font_get_ctm")]
    #[doc(alias = "get_ctm")]
    pub fn ctm(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe { ffi::cairo_scaled_font_get_ctm(self.to_raw_none(), matrix.mut_ptr()) }

        matrix
    }

    #[doc(alias = "cairo_scaled_font_get_scale_matrix")]
    #[doc(alias = "get_scale_matrix")]
    pub fn scale_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();

        unsafe { ffi::cairo_scaled_font_get_scale_matrix(self.to_raw_none(), matrix.mut_ptr()) }

        matrix
    }

    #[doc(alias = "cairo_scaled_font_status")]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_scaled_font_status(self.to_raw_none()) };
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
            ffi::cairo_scaled_font_set_user_data(
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
            let ptr = ffi::cairo_scaled_font_get_user_data(self.to_raw_none(), &key.ffi);
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
            ffi::cairo_scaled_font_set_user_data(
                self.to_raw_none(),
                &key.ffi,
                std::ptr::null_mut(),
                None,
            )
        };
        crate::cairo::utils::status_to_result(status)
    }
}

impl Drop for ScaledFont {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_scaled_font_destroy(self.to_raw_none());
        }
    }
}

impl Clone for ScaledFont {
    #[inline]
    fn clone(&self) -> ScaledFont {
        unsafe { ScaledFont::from_raw_none(self.to_raw_none()) }
    }
}
