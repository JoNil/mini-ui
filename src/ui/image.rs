use std::{alloc, slice};

use crate::{
    cairo::{Format, ImageSurface},
    png::stbi_load_from_memory,
};

#[derive(Clone, Debug)]
pub struct Image {
    pub(crate) image: ImageSurface,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Image {
    pub fn load_png(data: &[u8]) -> Image {
        let mut width = 0;
        let mut height = 0;
        let mut comp = 0;

        let data = unsafe {
            stbi_load_from_memory(
                data.as_ptr(),
                data.len() as _,
                &mut width as *mut _,
                &mut height as *mut _,
                &mut comp as *mut _,
                4,
            )
        };

        let image = Image::load(width, height, unsafe {
            slice::from_raw_parts(data, (4 * width * height) as _)
        });

        let layout = alloc::Layout::from_size_align(1, 1).expect("Bad layout");
        unsafe { alloc::dealloc(data, layout) };

        image
    }

    pub fn load(width: i32, height: i32, data: &[u8]) -> Image {
        let mut image = ImageSurface::create(Format::ARgb32, width, height).unwrap();

        {
            let mut image_date = image.data().unwrap();
            image_date.copy_from_slice(data);
        }

        Image {
            image,
            width,
            height,
        }
    }
}
