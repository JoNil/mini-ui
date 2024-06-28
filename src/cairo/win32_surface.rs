#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::{
    cairo::{ffi, Error, Format, Surface, SurfaceType},
    win32::HDC,
};
use std::ops::Deref;

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Surface(Surface);

impl TryFrom<Surface> for Win32Surface {
    type Error = Surface;

    #[inline]
    fn try_from(surface: Surface) -> Result<Win32Surface, Surface> {
        if surface.type_() == SurfaceType::Win32 {
            Ok(Win32Surface(surface))
        } else {
            Err(surface)
        }
    }
}

impl Win32Surface {
    #[inline]
    pub unsafe fn from_raw_full(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<Win32Surface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_full(ptr)?;
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }

    #[inline]
    pub unsafe fn from_raw_none(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<Win32Surface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_none(ptr);
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }
}

impl Deref for Win32Surface {
    type Target = Surface;

    #[inline]
    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl AsRef<Surface> for Win32Surface {
    #[inline]
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for Win32Surface {
    #[inline]
    fn clone(&self) -> Win32Surface {
        Win32Surface(self.0.clone())
    }
}

impl Win32Surface {
    #[doc(alias = "cairo_win32_surface_create")]
    pub fn create(hdc: HDC) -> Result<Win32Surface, Error> {
        unsafe { Self::from_raw_full(ffi::cairo_win32_surface_create(hdc)) }
    }

    #[doc(alias = "cairo_win32_surface_create_with_format")]
    pub fn create_with_format(hdc: HDC, format: Format) -> Result<Win32Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_format(
                hdc,
                format.into(),
            ))
        }
    }

    #[doc(alias = "cairo_win32_surface_create_with_dib")]
    pub fn create_with_dib(format: Format, width: i32, height: i32) -> Result<Win32Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_dib(
                format.into(),
                width,
                height,
            ))
        }
    }

    #[doc(alias = "cairo_win32_surface_create_with_ddb")]
    pub fn create_with_ddb(
        hdc: HDC,
        format: Format,
        width: i32,
        height: i32,
    ) -> Result<Win32Surface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_win32_surface_create_with_ddb(
                hdc,
                format.into(),
                width,
                height,
            ))
        }
    }

    #[doc(alias = "cairo_win32_printing_surface_create")]
    pub fn printing_surface_create(hdc: HDC) -> Result<Win32Surface, Error> {
        unsafe { Self::from_raw_full(ffi::cairo_win32_printing_surface_create(hdc)) }
    }
}
