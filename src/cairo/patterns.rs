use std::{ops::Deref, ptr};

use std::ffi::{c_double, c_int, c_uint};

use crate::cairo::{
    ffi, utils::status_to_result, Error, Extend, Filter, Matrix, MeshCorner, Path, PatternType,
    Surface,
};

// See https://cairographics.org/manual/bindings-patterns.html for more info
#[derive(Debug)]
pub struct Pattern {
    pointer: *mut ffi::cairo_pattern_t,
}

impl Pattern {
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
            ffi::cairo_pattern_set_user_data(
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
            let ptr = ffi::cairo_pattern_get_user_data(self.to_raw_none(), &key.ffi);
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
            ffi::cairo_pattern_set_user_data(
                self.to_raw_none(),
                &key.ffi,
                std::ptr::null_mut(),
                None,
            )
        };
        crate::cairo::utils::status_to_result(status)
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_pattern_t {
        self.pointer
    }

    #[inline]
    pub unsafe fn from_raw_none(pointer: *mut ffi::cairo_pattern_t) -> Pattern {
        ffi::cairo_pattern_reference(pointer);
        Self::from_raw_full(pointer)
    }

    #[inline]
    pub unsafe fn from_raw_full(pointer: *mut ffi::cairo_pattern_t) -> Pattern {
        Self { pointer }
    }

    #[doc(alias = "cairo_pattern_get_type")]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> PatternType {
        unsafe { ffi::cairo_pattern_get_type(self.pointer).into() }
    }

    #[doc(alias = "cairo_pattern_get_reference_count")]
    #[doc(alias = "get_reference_count")]
    pub fn reference_count(&self) -> isize {
        unsafe { ffi::cairo_pattern_get_reference_count(self.pointer) as isize }
    }

    #[doc(alias = "cairo_pattern_set_extend")]
    pub fn set_extend(&self, extend: Extend) {
        unsafe { ffi::cairo_pattern_set_extend(self.pointer, extend.into()) }
    }

    #[doc(alias = "cairo_pattern_get_extend")]
    #[doc(alias = "get_extend")]
    pub fn extend(&self) -> Extend {
        unsafe { Extend::from(ffi::cairo_pattern_get_extend(self.pointer)) }
    }

    #[doc(alias = "cairo_pattern_set_filter")]
    pub fn set_filter(&self, filter: Filter) {
        unsafe { ffi::cairo_pattern_set_filter(self.pointer, filter.into()) }
    }

    #[doc(alias = "cairo_pattern_get_filter")]
    #[doc(alias = "get_filter")]
    pub fn filter(&self) -> Filter {
        unsafe { Filter::from(ffi::cairo_pattern_get_filter(self.pointer)) }
    }

    #[doc(alias = "cairo_pattern_set_matrix")]
    pub fn set_matrix(&self, matrix: Matrix) {
        unsafe { ffi::cairo_pattern_set_matrix(self.pointer, matrix.ptr()) }
    }

    #[doc(alias = "cairo_pattern_get_matrix")]
    #[doc(alias = "get_matrix")]
    pub fn matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_pattern_get_matrix(self.pointer, matrix.mut_ptr());
        }
        matrix
    }

    #[doc(alias = "cairo_pattern_status")]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_pattern_status(self.pointer) };
        status_to_result(status)
    }
}

impl Clone for Pattern {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            pointer: unsafe { ffi::cairo_pattern_reference(self.pointer) },
        }
    }
}

impl Drop for Pattern {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::cairo_pattern_destroy(self.pointer) }
    }
}

impl AsRef<Pattern> for Pattern {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        self
    }
}

#[derive(Debug, Clone)]
pub struct SolidPattern(Pattern);

impl Deref for SolidPattern {
    type Target = Pattern;

    #[inline]
    fn deref(&self) -> &Pattern {
        &self.0
    }
}

impl AsRef<Pattern> for SolidPattern {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}
impl TryFrom<Pattern> for SolidPattern {
    type Error = Pattern;

    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::Solid {
            Ok(SolidPattern(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl SolidPattern {
    #[doc(alias = "cairo_pattern_create_rgb")]
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        unsafe {
            Self(Pattern::from_raw_full(ffi::cairo_pattern_create_rgb(
                red, green, blue,
            )))
        }
    }

    #[doc(alias = "cairo_pattern_create_rgba")]
    pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        unsafe {
            Self(Pattern::from_raw_full(ffi::cairo_pattern_create_rgba(
                red, green, blue, alpha,
            )))
        }
    }

    #[doc(alias = "cairo_pattern_get_rgba")]
    #[doc(alias = "get_rgba")]
    pub fn rgba(&self) -> Result<(f64, f64, f64, f64), Error> {
        unsafe {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut alpha = 0.0;

            let status = ffi::cairo_pattern_get_rgba(
                self.pointer,
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            );
            status_to_result(status)?;

            Ok((red, green, blue, alpha))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gradient(Pattern);

impl Deref for Gradient {
    type Target = Pattern;

    #[inline]
    fn deref(&self) -> &Pattern {
        &self.0
    }
}

impl AsRef<Pattern> for Gradient {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}

impl TryFrom<Pattern> for Gradient {
    type Error = Pattern;
    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::LinearGradient
            || pattern.type_() == PatternType::RadialGradient
        {
            Ok(Gradient(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl Gradient {
    #[doc(alias = "cairo_pattern_add_color_stop_rgb")]
    pub fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
        unsafe { ffi::cairo_pattern_add_color_stop_rgb(self.pointer, offset, red, green, blue) }
    }

    #[doc(alias = "cairo_pattern_add_color_stop_rgba")]
    pub fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgba(self.pointer, offset, red, green, blue, alpha)
        }
    }

    #[doc(alias = "cairo_pattern_get_color_stop_count")]
    #[doc(alias = "get_color_stop_count")]
    pub fn color_stop_count(&self) -> Result<isize, Error> {
        unsafe {
            let mut count = 0;
            let status = ffi::cairo_pattern_get_color_stop_count(self.pointer, &mut count);

            status_to_result(status)?;
            Ok(count as isize)
        }
    }

    #[doc(alias = "cairo_pattern_get_color_stop_rgba")]
    #[doc(alias = "get_color_stop_rgba")]
    pub fn color_stop_rgba(&self, index: isize) -> Result<(f64, f64, f64, f64, f64), Error> {
        unsafe {
            let mut offset = 0.0;
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut alpha = 0.0;

            let status = ffi::cairo_pattern_get_color_stop_rgba(
                self.pointer,
                index as c_int,
                &mut offset,
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            );
            status_to_result(status)?;
            Ok((offset, red, green, blue, alpha))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient(Gradient);

impl Deref for LinearGradient {
    type Target = Gradient;

    #[inline]
    fn deref(&self) -> &Gradient {
        &self.0
    }
}

impl AsRef<Gradient> for LinearGradient {
    #[inline]
    fn as_ref(&self) -> &Gradient {
        &self.0
    }
}

impl AsRef<Pattern> for LinearGradient {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}

impl TryFrom<Pattern> for LinearGradient {
    type Error = Pattern;
    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::LinearGradient {
            let pattern = Gradient(pattern);
            Ok(LinearGradient(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl TryFrom<Gradient> for LinearGradient {
    type Error = Gradient;
    fn try_from(pattern: Gradient) -> Result<Self, Gradient> {
        if pattern.type_() == PatternType::LinearGradient {
            Ok(LinearGradient(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl LinearGradient {
    #[doc(alias = "cairo_pattern_create_linear")]
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        unsafe {
            Self(Gradient(Pattern::from_raw_full(
                ffi::cairo_pattern_create_linear(x0, y0, x1, y1),
            )))
        }
    }

    #[doc(alias = "cairo_pattern_get_linear_points")]
    #[doc(alias = "get_linear_points")]
    pub fn linear_points(&self) -> Result<(f64, f64, f64, f64), Error> {
        unsafe {
            let mut x0 = 0.0;
            let mut y0 = 0.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;

            let status = ffi::cairo_pattern_get_linear_points(
                self.pointer,
                &mut x0,
                &mut y0,
                &mut x1,
                &mut y1,
            );
            status_to_result(status)?;
            Ok((x0, y0, x1, y1))
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadialGradient(Gradient);

impl Deref for RadialGradient {
    type Target = Gradient;

    #[inline]
    fn deref(&self) -> &Gradient {
        &self.0
    }
}

impl AsRef<Gradient> for RadialGradient {
    #[inline]
    fn as_ref(&self) -> &Gradient {
        &self.0
    }
}

impl AsRef<Pattern> for RadialGradient {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}

impl TryFrom<Pattern> for RadialGradient {
    type Error = Pattern;
    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::RadialGradient {
            let pattern = Gradient(pattern);
            Ok(RadialGradient(pattern))
        } else {
            Err(pattern)
        }
    }
}
impl TryFrom<Gradient> for RadialGradient {
    type Error = Gradient;
    fn try_from(pattern: Gradient) -> Result<Self, Gradient> {
        if pattern.type_() == PatternType::RadialGradient {
            Ok(RadialGradient(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl RadialGradient {
    #[doc(alias = "cairo_pattern_create_radial")]
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Self {
        unsafe {
            Self(Gradient(Pattern::from_raw_full(
                ffi::cairo_pattern_create_radial(x0, y0, r0, x1, y1, r1),
            )))
        }
    }

    #[doc(alias = "cairo_pattern_get_radial_circles")]
    #[doc(alias = "get_radial_circles")]
    pub fn radial_circles(&self) -> Result<(f64, f64, f64, f64, f64, f64), Error> {
        unsafe {
            let mut x0 = 0.0;
            let mut y0 = 0.0;
            let mut r0 = 0.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut r1 = 0.0;

            let status = ffi::cairo_pattern_get_radial_circles(
                self.pointer,
                &mut x0,
                &mut y0,
                &mut r0,
                &mut x1,
                &mut y1,
                &mut r1,
            );
            status_to_result(status)?;
            Ok((x0, y0, r0, x1, y1, r1))
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurfacePattern(Pattern);

impl Deref for SurfacePattern {
    type Target = Pattern;

    #[inline]
    fn deref(&self) -> &Pattern {
        &self.0
    }
}

impl AsRef<Pattern> for SurfacePattern {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}

impl TryFrom<Pattern> for SurfacePattern {
    type Error = Pattern;
    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::Surface {
            Ok(SurfacePattern(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl SurfacePattern {
    #[doc(alias = "cairo_pattern_create_for_surface")]
    pub fn create(surface: impl AsRef<Surface>) -> Self {
        unsafe {
            Self(Pattern::from_raw_full(
                ffi::cairo_pattern_create_for_surface(surface.as_ref().to_raw_none()),
            ))
        }
    }

    #[doc(alias = "cairo_pattern_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Result<Surface, Error> {
        unsafe {
            let mut surface_ptr: *mut ffi::cairo_surface_t = ptr::null_mut();
            let status = ffi::cairo_pattern_get_surface(self.pointer, &mut surface_ptr);
            status_to_result(status)?;
            Ok(Surface::from_raw_none(surface_ptr))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh(Pattern);

impl Deref for Mesh {
    type Target = Pattern;

    #[inline]
    fn deref(&self) -> &Pattern {
        &self.0
    }
}

impl AsRef<Pattern> for Mesh {
    #[inline]
    fn as_ref(&self) -> &Pattern {
        &self.0
    }
}

impl TryFrom<Pattern> for Mesh {
    type Error = Pattern;
    fn try_from(pattern: Pattern) -> Result<Self, Pattern> {
        if pattern.type_() == PatternType::Mesh {
            Ok(Mesh(pattern))
        } else {
            Err(pattern)
        }
    }
}

impl Mesh {
    #[doc(alias = "cairo_pattern_create_mesh")]
    pub fn new() -> Self {
        unsafe { Self(Pattern::from_raw_full(ffi::cairo_pattern_create_mesh())) }
    }

    #[doc(alias = "cairo_mesh_pattern_begin_patch")]
    pub fn begin_patch(&self) {
        unsafe { ffi::cairo_mesh_pattern_begin_patch(self.pointer) }
    }

    #[doc(alias = "cairo_mesh_pattern_end_patch")]
    pub fn end_patch(&self) {
        unsafe { ffi::cairo_mesh_pattern_end_patch(self.pointer) }
    }

    #[doc(alias = "cairo_mesh_pattern_move_to")]
    pub fn move_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_move_to(self.pointer, x, y) }
    }

    #[doc(alias = "cairo_mesh_pattern_line_to")]
    pub fn line_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_line_to(self.pointer, x, y) }
    }

    #[doc(alias = "cairo_mesh_pattern_curve_to")]
    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe { ffi::cairo_mesh_pattern_curve_to(self.pointer, x1, y1, x2, y2, x3, y3) }
    }

    #[doc(alias = "cairo_mesh_pattern_set_control_point")]
    pub fn set_control_point(&self, corner: MeshCorner, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_set_control_point(self.pointer, corner.into(), x, y) }
    }

    #[doc(alias = "cairo_mesh_pattern_get_control_point")]
    #[doc(alias = "get_control_point")]
    pub fn control_point(&self, patch_num: usize, corner: MeshCorner) -> Result<(f64, f64), Error> {
        let mut x: c_double = 0.0;
        let mut y: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_control_point(
                self.pointer,
                patch_num as c_uint,
                corner.into(),
                &mut x,
                &mut y,
            )
        };
        status_to_result(status)?;
        Ok((x, y))
    }

    #[doc(alias = "cairo_mesh_pattern_set_corner_color_rgb")]
    pub fn set_corner_color_rgb(&self, corner: MeshCorner, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgb(
                self.pointer,
                corner.into(),
                red,
                green,
                blue,
            )
        }
    }

    #[doc(alias = "cairo_mesh_pattern_set_corner_color_rgba")]
    pub fn set_corner_color_rgba(
        &self,
        corner: MeshCorner,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    ) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgba(
                self.pointer,
                corner.into(),
                red,
                green,
                blue,
                alpha,
            )
        }
    }

    #[doc(alias = "cairo_mesh_pattern_get_corner_color_rgba")]
    #[doc(alias = "get_corner_color_rgba")]
    pub fn corner_color_rgba(
        &self,
        patch_num: usize,
        corner: MeshCorner,
    ) -> Result<(f64, f64, f64, f64), Error> {
        let mut red: c_double = 0.0;
        let mut green: c_double = 0.0;
        let mut blue: c_double = 0.0;
        let mut alpha: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_corner_color_rgba(
                self.pointer,
                patch_num as c_uint,
                corner.into(),
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            )
        };
        status_to_result(status)?;
        Ok((red, green, blue, alpha))
    }

    #[doc(alias = "cairo_mesh_pattern_get_patch_count")]
    #[doc(alias = "get_patch_count")]
    pub fn patch_count(&self) -> Result<usize, Error> {
        let mut count: c_uint = 0;
        unsafe {
            let status = ffi::cairo_mesh_pattern_get_patch_count(self.pointer, &mut count);
            status_to_result(status)?;
        }
        Ok(count as usize)
    }

    #[doc(alias = "cairo_mesh_pattern_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self, patch_num: usize) -> Result<Path, Error> {
        let path: Path = unsafe {
            Path::from_raw_full(ffi::cairo_mesh_pattern_get_path(
                self.pointer,
                patch_num as c_uint,
            ))
        };
        let status = unsafe {
            let ptr: *mut ffi::cairo_path_t = path.as_ptr();
            (*ptr).status
        };
        status_to_result(status)?;
        Ok(path)
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn try_from() {
    let linear = LinearGradient::new(0., 0., 1., 1.);
    let gradient = Gradient::clone(&linear);
    let pattern = Pattern::clone(&linear);
    assert!(Gradient::try_from(pattern.clone()).is_ok());
    assert!(LinearGradient::try_from(gradient).is_ok());
    assert!(LinearGradient::try_from(pattern).is_ok());
}
