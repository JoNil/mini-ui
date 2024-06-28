use crate::cairo::{Format, ImageSurface};

#[derive(Clone, Debug)]
pub struct Image {
    pub(crate) image: ImageSurface,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Image {
    pub fn load_png(data: &[u8]) -> Image {
        let mut image = ImageSurface::create(Format::ARgb32, 1, 1).unwrap();

        Image {
            image,
            width: 1,
            height: 1,
        }
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
