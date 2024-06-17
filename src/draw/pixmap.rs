use crate::{
    draw::{
        color::PremultipliedColorU8,
        geom::{IntSizeExt, ScreenIntRect},
        Color, IntRect,
    },
    path::IntSize,
};
use std::{convert::TryFrom, num::NonZeroUsize};

/// Number of bytes per pixel.
pub const BYTES_PER_PIXEL: usize = 4;

/// A container that owns premultiplied RGBA pixels.
///
/// The data is not aligned, therefore width == stride.
#[derive(Clone, PartialEq)]
pub struct Pixmap {
    data: Vec<u8>,
    size: IntSize,
}

impl Pixmap {
    /// Allocates a new pixmap.
    ///
    /// A pixmap is filled with transparent black by default, aka (0, 0, 0, 0).
    ///
    /// Zero size in an error.
    ///
    /// Pixmap's width is limited by i32::MAX/4.
    pub fn new(width: u32, height: u32) -> Option<Self> {
        let size = IntSize::from_wh(width, height)?;
        let data_len = data_len_for_size(size)?;

        // We cannot check that allocation was successful yet.
        // We have to wait for https://github.com/rust-lang/rust/issues/48043

        Some(Pixmap {
            data: vec![0; data_len],
            size,
        })
    }

    /// Creates a new pixmap by taking ownership over an image buffer
    /// (premultiplied RGBA pixels).
    ///
    /// The size needs to match the data provided.
    ///
    /// Pixmap's width is limited by i32::MAX/4.
    pub fn from_vec(data: Vec<u8>, size: IntSize) -> Option<Self> {
        let data_len = data_len_for_size(size)?;
        if data.len() != data_len {
            return None;
        }

        Some(Pixmap { data, size })
    }

    /// Returns a container that references Pixmap's data.
    pub fn as_ref(&self) -> PixmapRef {
        PixmapRef {
            data: &self.data,
            size: self.size,
        }
    }

    /// Returns a container that references Pixmap's data.
    pub fn as_mut(&mut self) -> PixmapMut {
        PixmapMut {
            data: &mut self.data,
            size: self.size,
        }
    }

    /// Returns pixmap's width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.size.width()
    }

    /// Returns pixmap's height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.size.height()
    }

    /// Returns pixmap's size.
    #[allow(dead_code)]
    pub(crate) fn size(&self) -> IntSize {
        self.size
    }

    /// Fills the entire pixmap with a specified color.
    pub fn fill(&mut self, color: Color) {
        let c = color.premultiply().to_color_u8();
        for p in self.as_mut().pixels_mut() {
            *p = c;
        }
    }

    /// Returns the internal data.
    ///
    /// Byteorder: RGBA
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Returns the mutable internal data.
    ///
    /// Byteorder: RGBA
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Returns a pixel color.
    ///
    /// Returns `None` when position is out of bounds.
    pub fn pixel(&self, x: u32, y: u32) -> Option<PremultipliedColorU8> {
        let idx = self.width().checked_mul(y)?.checked_add(x)?;
        self.pixels().get(idx as usize).cloned()
    }

    /// Returns a mutable slice of pixels.
    pub fn pixels_mut(&mut self) -> &mut [PremultipliedColorU8] {
        unsafe { std::mem::transmute(self.data_mut()) }
    }

    /// Returns a slice of pixels.
    pub fn pixels(&self) -> &[PremultipliedColorU8] {
        unsafe { std::mem::transmute(self.data()) }
    }

    /// Consumes the internal data.
    ///
    /// Byteorder: RGBA
    pub fn take(self) -> Vec<u8> {
        self.data
    }

    /// Returns a copy of the pixmap that intersects the `rect`.
    ///
    /// Returns `None` when `Pixmap`'s rect doesn't contain `rect`.
    pub fn clone_rect(&self, rect: IntRect) -> Option<Pixmap> {
        self.as_ref().clone_rect(rect)
    }
}

impl core::fmt::Debug for Pixmap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pixmap")
            .field("data", &"...")
            .field("width", &self.size.width())
            .field("height", &self.size.height())
            .finish()
    }
}

/// A container that references premultiplied RGBA pixels.
///
/// Can be created from `Pixmap` or from a user provided data.
///
/// The data is not aligned, therefore width == stride.
#[derive(Clone, Copy, PartialEq)]
pub struct PixmapRef<'a> {
    data: &'a [u8],
    size: IntSize,
}

impl<'a> PixmapRef<'a> {
    /// Creates a new `PixmapRef` from bytes.
    ///
    /// The size must be at least `size.width() * size.height() * BYTES_PER_PIXEL`.
    /// Zero size in an error. Width is limited by i32::MAX/4.
    ///
    /// The `data` is assumed to have premultiplied RGBA pixels (byteorder: RGBA).
    pub fn from_bytes(data: &'a [u8], width: u32, height: u32) -> Option<Self> {
        let size = IntSize::from_wh(width, height)?;
        let data_len = data_len_for_size(size)?;
        if data.len() < data_len {
            return None;
        }

        Some(PixmapRef { data, size })
    }

    /// Creates a new `Pixmap` from the current data.
    ///
    /// Clones the underlying data.
    pub fn to_owned(&self) -> Pixmap {
        Pixmap {
            data: self.data.to_vec(),
            size: self.size,
        }
    }

    /// Returns pixmap's width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.size.width()
    }

    /// Returns pixmap's height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.size.height()
    }

    /// Returns pixmap's size.
    pub(crate) fn size(&self) -> IntSize {
        self.size
    }

    /// Returns pixmap's rect.
    pub(crate) fn rect(&self) -> ScreenIntRect {
        self.size.to_screen_int_rect(0, 0)
    }

    /// Returns the internal data.
    ///
    /// Byteorder: RGBA
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns a pixel color.
    ///
    /// Returns `None` when position is out of bounds.
    pub fn pixel(&self, x: u32, y: u32) -> Option<PremultipliedColorU8> {
        let idx = self.width().checked_mul(y)?.checked_add(x)?;
        self.pixels().get(idx as usize).cloned()
    }

    /// Returns a slice of pixels.
    pub fn pixels(&self) -> &'a [PremultipliedColorU8] {
        unsafe { std::mem::transmute(self.data()) }
    }

    // TODO: add rows() iterator

    /// Returns a copy of the pixmap that intersects the `rect`.
    ///
    /// Returns `None` when `Pixmap`'s rect doesn't contain `rect`.
    pub fn clone_rect(&self, rect: IntRect) -> Option<Pixmap> {
        // TODO: to ScreenIntRect?

        let rect = self.rect().to_int_rect().intersect(&rect)?;
        let mut new = Pixmap::new(rect.width(), rect.height())?;
        {
            let old_pixels = self.pixels();
            let mut new_mut = new.as_mut();
            let new_pixels = new_mut.pixels_mut();

            // TODO: optimize
            for y in 0..rect.height() {
                for x in 0..rect.width() {
                    let old_idx = (y + rect.y() as u32) * self.width() + (x + rect.x() as u32);
                    let new_idx = y * rect.width() + x;
                    new_pixels[new_idx as usize] = old_pixels[old_idx as usize];
                }
            }
        }

        Some(new)
    }

    /// Encodes pixmap into a PNG data.
    #[cfg(feature = "png-format")]
    pub fn encode_png(&self) -> Result<Vec<u8>, png::EncodingError> {
        // Skia uses skcms here, which is somewhat similar to RasterPipeline.

        // Sadly, we have to copy the pixmap here, because of demultiplication.
        // Not sure how to avoid this.
        // TODO: remove allocation
        let mut tmp_pixmap = self.to_owned();

        // Demultiply alpha.
        //
        // RasterPipeline is 15% faster here, but produces slightly different results
        // due to rounding. So we stick with this method for now.
        for pixel in tmp_pixmap.pixels_mut() {
            let c = pixel.demultiply();
            *pixel =
                PremultipliedColorU8::from_rgba_unchecked(c.red(), c.green(), c.blue(), c.alpha());
        }

        let mut data = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut data, self.width(), self.height());
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header()?;
            writer.write_image_data(&tmp_pixmap.data)?;
        }

        Ok(data)
    }

    /// Saves pixmap as a PNG file.
    #[cfg(feature = "png-format")]
    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), png::EncodingError> {
        let data = self.encode_png()?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

impl core::fmt::Debug for PixmapRef<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PixmapRef")
            .field("data", &"...")
            .field("width", &self.size.width())
            .field("height", &self.size.height())
            .finish()
    }
}

/// A container that references mutable premultiplied RGBA pixels.
///
/// Can be created from `Pixmap` or from a user provided data.
///
/// The data is not aligned, therefore width == stride.
#[derive(PartialEq)]
pub struct PixmapMut<'a> {
    data: &'a mut [u8],
    size: IntSize,
}

impl<'a> PixmapMut<'a> {
    /// Creates a new `PixmapMut` from bytes.
    ///
    /// The size must be at least `size.width() * size.height() * BYTES_PER_PIXEL`.
    /// Zero size in an error. Width is limited by i32::MAX/4.
    ///
    /// The `data` is assumed to have premultiplied RGBA pixels (byteorder: RGBA).
    pub fn from_bytes(data: &'a mut [u8], width: u32, height: u32) -> Option<Self> {
        let size = IntSize::from_wh(width, height)?;
        let data_len = data_len_for_size(size)?;
        if data.len() < data_len {
            return None;
        }

        Some(PixmapMut { data, size })
    }

    /// Creates a new `Pixmap` from the current data.
    ///
    /// Clones the underlying data.
    pub fn to_owned(&self) -> Pixmap {
        Pixmap {
            data: self.data.to_vec(),
            size: self.size,
        }
    }

    /// Returns a container that references Pixmap's data.
    pub fn as_ref(&self) -> PixmapRef {
        PixmapRef {
            data: self.data,
            size: self.size,
        }
    }

    /// Returns pixmap's width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.size.width()
    }

    /// Returns pixmap's height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.size.height()
    }

    /// Returns pixmap's size.
    pub(crate) fn size(&self) -> IntSize {
        self.size
    }

    /// Fills the entire pixmap with a specified color.
    pub fn fill(&mut self, color: Color) {
        let c = color.premultiply().to_color_u8();
        for p in self.pixels_mut() {
            *p = c;
        }
    }

    /// Returns the mutable internal data.
    ///
    /// Byteorder: RGBA
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    /// Returns a mutable slice of pixels.
    pub fn pixels_mut(&mut self) -> &mut [PremultipliedColorU8] {
        unsafe { std::mem::transmute(self.data_mut()) }
    }

    /// Creates `SubPixmapMut` that contains the whole `PixmapMut`.
    pub(crate) fn as_subpixmap(&mut self) -> SubPixmapMut {
        SubPixmapMut {
            size: self.size(),
            real_width: self.width() as usize,
            data: self.data,
        }
    }

    /// Returns a mutable reference to the pixmap region that intersects the `rect`.
    ///
    /// Returns `None` when `Pixmap`'s rect doesn't contain `rect`.
    pub(crate) fn subpixmap(&mut self, rect: IntRect) -> Option<SubPixmapMut> {
        let rect = self.size.to_int_rect(0, 0).intersect(&rect)?;
        let row_bytes = self.width() as usize * BYTES_PER_PIXEL;
        let offset = rect.top() as usize * row_bytes + rect.left() as usize * BYTES_PER_PIXEL;

        Some(SubPixmapMut {
            size: rect.size(),
            real_width: self.width() as usize,
            data: &mut self.data[offset..],
        })
    }
}

impl core::fmt::Debug for PixmapMut<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PixmapMut")
            .field("data", &"...")
            .field("width", &self.size.width())
            .field("height", &self.size.height())
            .finish()
    }
}

/// A `PixmapMut` subregion.
///
/// Unlike `PixmapMut`, contains `real_width` which references the parent `PixmapMut` width.
/// This way we can operate on a `PixmapMut` subregion without reallocations.
/// Primarily required because of `DrawTiler`.
///
/// We cannot implement it in `PixmapMut` directly, because it will brake `fill`, `data_mut`
/// `pixels_mut` and other similar methods.
/// This is because `SubPixmapMut.data` references more "data" than it actually allowed to access.
/// On the other hand, `PixmapMut.data` can access all it's data and it's stored linearly.
pub struct SubPixmapMut<'a> {
    pub data: &'a mut [u8],
    pub size: IntSize,
    pub real_width: usize,
}

impl<'a> SubPixmapMut<'a> {
    /// Returns a mutable slice of pixels.
    pub fn pixels_mut(&mut self) -> &mut [PremultipliedColorU8] {
        unsafe { std::mem::transmute(self.data) }
    }
}

/// Returns minimum bytes per row as usize.
///
/// Pixmap's maximum value for row bytes must fit in 31 bits.
fn min_row_bytes(size: IntSize) -> Option<NonZeroUsize> {
    let w = i32::try_from(size.width()).ok()?;
    let w = w.checked_mul(BYTES_PER_PIXEL as i32)?;
    NonZeroUsize::new(w as usize)
}

/// Returns storage size required by pixel array.
fn compute_data_len(size: IntSize, row_bytes: usize) -> Option<usize> {
    let h = size.height().checked_sub(1)?;
    let h = (h as usize).checked_mul(row_bytes)?;

    let w = (size.width() as usize).checked_mul(BYTES_PER_PIXEL)?;

    h.checked_add(w)
}

fn data_len_for_size(size: IntSize) -> Option<usize> {
    let row_bytes = min_row_bytes(size)?;
    compute_data_len(size, row_bytes.get())
}
