use crate::{ffi, Error, Format, Surface, SurfaceType};
use ffi::CGContextRef;
use std::{fmt, ops::Deref};

#[derive(Debug)]
#[repr(transparent)]
pub struct QuartzSurface(Surface);

impl TryFrom<Surface> for QuartzSurface {
    type Error = Surface;

    #[inline]
    fn try_from(surface: Surface) -> Result<QuartzSurface, Surface> {
        if surface.type_() == SurfaceType::Quartz {
            Ok(QuartzSurface(surface))
        } else {
            Err(surface)
        }
    }
}

impl QuartzSurface {
    #[inline]
    pub unsafe fn from_raw_full(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<QuartzSurface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_full(ptr)?;
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }

    #[inline]
    pub unsafe fn from_raw_none(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<QuartzSurface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_none(ptr);
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }
}

impl Deref for QuartzSurface {
    type Target = Surface;

    #[inline]
    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl AsRef<Surface> for QuartzSurface {
    #[inline]
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for QuartzSurface {
    #[inline]
    fn clone(&self) -> QuartzSurface {
        QuartzSurface(self.0.clone())
    }
}

impl QuartzSurface {
    #[doc(alias = "cairo_quartz_surface_create")]
    pub fn create(format: Format, width: u32, height: u32) -> Result<QuartzSurface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create(
                format.into(),
                width,
                height,
            ))
        }
    }

    #[doc(alias = "cairo_quartz_surface_create_for_cg_context")]
    pub fn create_for_cg_context(
        cg_context: CGContextRef,
        width: u32,
        height: u32,
    ) -> Result<QuartzSurface, Error> {
        unsafe {
            Self::from_raw_full(ffi::cairo_quartz_surface_create_for_cg_context(
                cg_context, width, height,
            ))
        }
    }

    #[doc(alias = "cairo_quartz_surface_get_cg_context")]
    #[doc(alias = "get_cg_context")]
    pub fn cg_context(&self) -> CGContextRef {
        unsafe { ffi::cairo_quartz_surface_get_cg_context(self.to_raw_none()) }
    }
}
