#![allow(clippy::missing_safety_doc)]

pub use crate::cairo::{
    borrowed::Borrowed,
    constants::*,
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
    utils::{debug_reset_static_data, version_string, Version},
};

#[macro_use]
mod surface_macros;
#[macro_use]
mod user_data;
mod borrowed;
mod constants;
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
mod utils;

#[cfg(target_os = "macos")]
mod quartz_surface;
#[cfg(target_os = "macos")]
pub use quartz_surface::QuartzSurface;

#[cfg(target_os = "windows")]
mod win32_surface;

#[cfg(target_os = "windows")]
#[cfg_attr(docsrs, doc(cfg(target_os = "windows")))]
pub use win32_surface::Win32Surface;
