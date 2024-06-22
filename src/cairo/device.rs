use std::ptr;
use crate::cairo::{ffi, utils::status_to_result, DeviceType, Error};

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
            DeviceType::Xlib => {
                unsafe {
                    ffi::cairo_xlib_device_debug_cap_xrender_version(
                        self.to_raw_none(),
                        _major_version,
                        _minor_version,
                    )
                }
            }
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
    #[doc(alias = "cairo_xlib_device_debug_get_precision")]
    #[doc(alias = "cairo_xcb_device_debug_get_precision")]
    pub fn debug_get_precision(&self) -> i32 {
        match self.type_() {
            DeviceType::Xlib => {
                unsafe {
                    ffi::cairo_xlib_device_debug_get_precision(self.to_raw_none())
                }
                
            }
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
    #[doc(alias = "cairo_xlib_device_debug_set_precision")]
    #[doc(alias = "cairo_xcb_device_debug_set_precision")]
    pub fn debug_set_precision(&self, _precision: i32) {
        match self.type_() {
            DeviceType::Xlib => {
                unsafe {
                    ffi::cairo_xlib_device_debug_set_precision(self.to_raw_none(), _precision)
                }
            }
            d => panic!("invalid device type: {:#?}", d),
        }
    }

    #[doc(alias = "cairo_device_status")]
    #[inline]
    pub fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_device_status(self.to_raw_none()) };
        status_to_result(status)
    }

    user_data_methods! {
        ffi::cairo_device_get_user_data,
        ffi::cairo_device_set_user_data,
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
