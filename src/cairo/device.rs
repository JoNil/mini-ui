use crate::cairo::{ffi, utils::status_to_result, DeviceType, Error};
use std::ptr;

#[derive(Debug)]
#[must_use = "if unused the Device will immediately be released"]
pub struct DeviceAcquireGuard<'a>(&'a Device);

impl<'a> Drop for DeviceAcquireGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        self.0.release();
    }
}

#[derive(Debug)]
#[doc(alias = "cairo_device_t")]
#[repr(transparent)]
pub struct Device(ptr::NonNull<ffi::cairo_device_t>);

impl Device {
    #[inline]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_device_t) -> Device {
        debug_assert!(!ptr.is_null());
        ffi::cairo_device_reference(ptr);
        Device(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_device_t) -> crate::cairo::Borrowed<Device> {
        debug_assert!(!ptr.is_null());
        crate::cairo::Borrowed::new(Device(ptr::NonNull::new_unchecked(ptr)))
    }

    #[inline]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_device_t) -> Device {
        debug_assert!(!ptr.is_null());
        Device(ptr::NonNull::new_unchecked(ptr))
    }

    #[inline]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_device_t {
        self.0.as_ptr()
    }

    #[doc(alias = "cairo_device_finish")]
    pub fn finish(&self) {
        unsafe { ffi::cairo_device_finish(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_flush")]
    pub fn flush(&self) {
        unsafe { ffi::cairo_device_flush(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_get_type")]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> DeviceType {
        unsafe { DeviceType::from(ffi::cairo_device_get_type(self.to_raw_none())) }
    }

    #[doc(alias = "cairo_device_acquire")]
    pub fn acquire(&self) -> Result<DeviceAcquireGuard, Error> {
        unsafe {
            let status = ffi::cairo_device_acquire(self.to_raw_none());
            status_to_result(status)?;
        }
        Ok(DeviceAcquireGuard(self))
    }

    #[doc(alias = "cairo_device_release")]
    fn release(&self) {
        unsafe { ffi::cairo_device_release(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_elapsed")]
    pub fn observer_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_elapsed(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_fill_elapsed")]
    pub fn observer_fill_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_fill_elapsed(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_glyphs_elapsed")]
    pub fn observer_glyphs_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_glyphs_elapsed(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_mask_elapsed")]
    pub fn observer_mask_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_mask_elapsed(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_paint_elapsed")]
    pub fn observer_paint_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_paint_elapsed(self.to_raw_none()) }
    }

    #[doc(alias = "cairo_device_observer_stroke_elapsed")]
    pub fn observer_stroke_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_stroke_elapsed(self.to_raw_none()) }
    }

    #[cfg(target_os = "linux")]
    #[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
    #[doc(alias = "cairo_xlib_device_debug_cap_xrender_version")]
    #[doc(alias = "cairo_xcb_device_debug_cap_xrender_version")]
    pub fn debug_cap_xrender_version(&self, _major_version: i32, _minor_version: i32) {
        match self.type_() {
            DeviceType::Xlib => unsafe {
                ffi::cairo_xlib_device_debug_cap_xrender_version(
                    self.to_raw_none(),
                    _major_version,
                    _minor_version,
                )
            },
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
    #[doc(alias = "cairo_xlib_device_debug_get_precision")]
    #[doc(alias = "cairo_xcb_device_debug_get_precision")]
    pub fn debug_get_precision(&self) -> i32 {
        match self.type_() {
            DeviceType::Xlib => unsafe {
                ffi::cairo_xlib_device_debug_get_precision(self.to_raw_none())
            },
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
    #[doc(alias = "cairo_xlib_device_debug_set_precision")]
    #[doc(alias = "cairo_xcb_device_debug_set_precision")]
    pub fn debug_set_precision(&self, _precision: i32) {
        match self.type_() {
            DeviceType::Xlib => unsafe {
                ffi::cairo_xlib_device_debug_set_precision(self.to_raw_none(), _precision)
            },
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[doc(alias = "cairo_device_status")]
    #[inline]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_device_status(self.to_raw_none()) };
        status_to_result(status)
    }

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
            ffi::cairo_device_set_user_data(
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
            let ptr = ffi::cairo_device_get_user_data(self.to_raw_none(), &key.ffi);
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
            ffi::cairo_device_set_user_data(
                self.to_raw_none(),
                &key.ffi,
                std::ptr::null_mut(),
                None,
            )
        };
        crate::cairo::utils::status_to_result(status)
    }
}

impl Clone for Device {
    #[inline]
    fn clone(&self) -> Device {
        unsafe { Self::from_raw_none(ffi::cairo_device_reference(self.0.as_ptr())) }
    }
}

impl Drop for Device {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_device_destroy(self.0.as_ptr());
        }
    }
}
