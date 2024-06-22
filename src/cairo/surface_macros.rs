// e.g. declare_surface(ImageSurface, SurfaceType::Image)
macro_rules! declare_surface {
    ($surf_name:ident, $surf_type:expr) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $surf_name(Surface);

        impl TryFrom<Surface> for $surf_name {
            type Error = Surface;

            #[inline]
            fn try_from(surface: Surface) -> Result<$surf_name, Surface> {
                if surface.type_() == $surf_type {
                    Ok($surf_name(surface))
                } else {
                    Err(surface)
                }
            }
        }

        impl $surf_name {
            #[inline]
            pub unsafe fn from_raw_full(
                ptr: *mut crate::cairo::ffi::cairo_surface_t,
            ) -> Result<$surf_name, crate::cairo::error::Error> {
                let surface = Surface::from_raw_full(ptr)?;
                Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
            }

            #[inline]
            pub unsafe fn from_raw_none(
                ptr: *mut crate::cairo::ffi::cairo_surface_t,
            ) -> Result<$surf_name, crate::cairo::error::Error> {
                let surface = Surface::from_raw_none(ptr);
                Self::try_from(surface).map_err(|_| crate::cairo::error::Error::SurfaceTypeMismatch)
            }
        }

        impl Deref for $surf_name {
            type Target = Surface;

            #[inline]
            fn deref(&self) -> &Surface {
                &self.0
            }
        }

        impl AsRef<Surface> for $surf_name {
            #[inline]
            fn as_ref(&self) -> &Surface {
                &self.0
            }
        }

        impl Clone for $surf_name {
            #[inline]
            fn clone(&self) -> $surf_name {
                $surf_name(self.0.clone())
            }
        }
    };
}
