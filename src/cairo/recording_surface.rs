use std::ops::Deref;

use crate::cairo::{ffi, Content, Error, Rectangle, Surface, SurfaceType};

#[derive(Debug)]
#[repr(transparent)]
pub struct RecordingSurface(Surface);

impl TryFrom<Surface> for RecordingSurface {
    type Error = Surface;

    #[inline]
    fn try_from(surface: Surface) -> Result<RecordingSurface, Surface> {
        if surface.type_() == SurfaceType::Recording {
            Ok(RecordingSurface(surface))
        } else {
            Err(surface)
        }
    }
}

impl RecordingSurface {
    #[inline]
    pub unsafe fn from_raw_full(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<RecordingSurface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_full(ptr)?;
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }

    #[inline]
    pub unsafe fn from_raw_none(
        ptr: *mut crate::cairo::ffi::cairo_surface_t,
    ) -> Result<RecordingSurface, crate::cairo::error::Error> {
        let surface = Surface::from_raw_none(ptr);
        Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
    }
}

impl Deref for RecordingSurface {
    type Target = Surface;

    #[inline]
    fn deref(&self) -> &Surface {
        &self.0
    }
}

impl AsRef<Surface> for RecordingSurface {
    #[inline]
    fn as_ref(&self) -> &Surface {
        &self.0
    }
}

impl Clone for RecordingSurface {
    #[inline]
    fn clone(&self) -> RecordingSurface {
        RecordingSurface(self.0.clone())
    }
}

impl RecordingSurface {
    #[doc(alias = "cairo_recording_surface_create")]
    pub fn create(content: Content, extends: Option<Rectangle>) -> Result<RecordingSurface, Error> {
        unsafe {
            let extends_ptr = match extends {
                Some(ref c) => c.to_raw_none(),
                None => ::std::ptr::null(),
            };

            Self::from_raw_full(ffi::cairo_recording_surface_create(
                content.into(),
                extends_ptr,
            ))
        }
    }

    #[doc(alias = "cairo_recording_surface_get_extents")]
    #[doc(alias = "get_extents")]
    pub fn extents(&self) -> Option<Rectangle> {
        unsafe {
            let rectangle: Rectangle = ::std::mem::zeroed();
            if ffi::cairo_recording_surface_get_extents(self.to_raw_none(), rectangle.to_raw_none())
                .as_bool()
            {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    #[doc(alias = "cairo_recording_surface_ink_extents")]
    pub fn ink_extents(&self) -> (f64, f64, f64, f64) {
        let mut x0 = 0.;
        let mut y0 = 0.;
        let mut width = 0.;
        let mut height = 0.;

        unsafe {
            ffi::cairo_recording_surface_ink_extents(
                self.to_raw_none(),
                &mut x0,
                &mut y0,
                &mut width,
                &mut height,
            );
        }
        (x0, y0, width, height)
    }
}
