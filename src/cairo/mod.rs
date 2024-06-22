#![allow(clippy::missing_safety_doc)]

#[cfg(feature = "freetype")]
#[cfg_attr(docsrs, doc(cfg(feature = "freetype")))]
pub use freetype;

#[cfg(feature = "pdf")]
#[cfg_attr(docsrs, doc(cfg(feature = "pdf")))]
pub use pdf::PdfSurface;
#[cfg(feature = "ps")]
#[cfg_attr(docsrs, doc(cfg(feature = "ps")))]
pub use ps::PsSurface;
#[cfg(any(feature = "pdf", feature = "svg", feature = "ps"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "pdf", feature = "svg", feature = "ps")))
)]
pub use stream::StreamWithError;
#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub use svg::SvgSurface;
#[cfg(feature = "xcb")]
#[cfg_attr(docsrs, doc(cfg(feature = "xcb")))]
pub use xcb::{
    XCBConnection, XCBDrawable, XCBPixmap, XCBRenderPictFormInfo, XCBScreen, XCBSurface,
    XCBVisualType,
};

pub use crate::cairo::{
    context::{Context, RectangleList},
    device::Device,
    enums::*,
    error::{BorrowError, Error, IoError, Result},
    font::{
        Antialias, FontExtents, FontFace, FontOptions, FontSlant, FontType, FontWeight, Glyph,
        HintMetrics, HintStyle, ScaledFont, SubpixelOrder, TextCluster, TextExtents, UserFontFace,
    },
    image_surface::{ImageSurface, ImageSurfaceData, ImageSurfaceDataOwned},
    matrices::Matrix,
    paths::{Path, PathSegment, PathSegments},
    patterns::{
        Gradient, LinearGradient, Mesh, Pattern, RadialGradient, SolidPattern, SurfacePattern,
    },
    recording_surface::RecordingSurface,
    rectangle::Rectangle,
    rectangle_int::RectangleInt,
    region::Region,
    surface::{MappedImageSurface, Surface},
    user_data::UserDataKey,
};

#[macro_use]
mod surface_macros;
#[macro_use]
mod user_data;
mod constants;
pub use crate::cairo::constants::*;
mod utils;
pub use crate::cairo::utils::{debug_reset_static_data, version_string, Version};
mod context;
mod device;
mod enums;
mod error;
mod ffi;
mod font;
mod image_surface;
mod matrices;
mod paths;
mod patterns;
mod recording_surface;
mod rectangle;
mod rectangle_int;
mod region;
mod surface;
#[cfg(feature = "png")]
mod surface_png;
#[cfg(feature = "xcb")]
mod xcb;

#[cfg(any(feature = "pdf", feature = "svg", feature = "ps"))]
#[macro_use]
mod stream;
#[cfg(feature = "pdf")]
mod pdf;
#[cfg(feature = "ps")]
mod ps;
#[cfg(feature = "svg")]
mod svg;

#[cfg(target_os = "macos")]
mod quartz_surface;
#[cfg(target_os = "macos")]
pub use quartz_surface::QuartzSurface;

#[cfg(all(windows, feature = "win32-surface"))]
mod win32_surface;

#[cfg(all(windows, feature = "win32-surface"))]
#[cfg_attr(docsrs, doc(cfg(all(windows, feature = "win32-surface"))))]
pub use win32_surface::Win32Surface;

mod borrowed {
    use std::mem;

    /// Wrapper around values representing borrowed C memory.
    ///
    /// This is returned by `from_glib_borrow()` and ensures that the wrapped value
    /// is never dropped when going out of scope.
    ///
    /// Borrowed values must never be passed by value or mutable reference to safe Rust code and must
    /// not leave the C scope in which they are valid.
    #[derive(Debug)]
    pub struct Borrowed<T>(mem::ManuallyDrop<T>);

    impl<T> Borrowed<T> {
        /// Creates a new borrowed value.
        #[inline]
        pub fn new(val: T) -> Self {
            Self(mem::ManuallyDrop::new(val))
        }

        /// Extracts the contained value.
        ///
        /// The returned value must never be dropped and instead has to be passed to `mem::forget()` or
        /// be directly wrapped in `mem::ManuallyDrop` or another `Borrowed` wrapper.
        #[inline]
        pub unsafe fn into_inner(self) -> T {
            mem::ManuallyDrop::into_inner(self.0)
        }
    }

    impl<T> AsRef<T> for Borrowed<T> {
        #[inline]
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T> std::ops::Deref for Borrowed<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            &self.0
        }
    }
}

pub use borrowed::Borrowed;
