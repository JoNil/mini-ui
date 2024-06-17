#![allow(clippy::approx_constant)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wrong_self_convention)]

mod alpha_runs;
mod blend_mode;
mod blitter;
mod color;
mod edge;
mod edge_builder;
mod edge_clipper;
mod fixed_point;
mod geom;
mod line_clipper;
mod mask;
mod math;
mod painter;
mod path64;
mod path_geometry;
mod pipeline;
mod pixmap;
mod scan;
mod shaders;
mod wide;

pub use crate::path::{IntRect, IntSize, NonZeroRect, Point, Rect, Size, Transform};
pub use crate::path::{LineCap, LineJoin, Stroke, StrokeDash};
pub use crate::path::{Path, PathBuilder, PathSegment, PathSegmentsIter, PathStroker};
pub use blend_mode::BlendMode;
pub use color::{Color, ColorSpace, ColorU8, PremultipliedColor, PremultipliedColorU8};
pub use color::{ALPHA_OPAQUE, ALPHA_TRANSPARENT, ALPHA_U8_OPAQUE, ALPHA_U8_TRANSPARENT};
pub use mask::{Mask, MaskType};
pub use painter::{FillRule, Paint};
pub use pixmap::{Pixmap, PixmapMut, PixmapRef, BYTES_PER_PIXEL};
pub use shaders::{FilterQuality, GradientStop, PixmapPaint, SpreadMode};
pub use shaders::{LinearGradient, Pattern, RadialGradient, Shader};

/// An integer length that is guarantee to be > 0
type LengthU32 = core::num::NonZeroU32;
