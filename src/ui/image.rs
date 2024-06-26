#[derive(Clone, Copy, Debug, Default)]
pub struct Image {
    pub(crate) id: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Image {
    pub fn load_png(data: &[u8]) -> Image {
        Image {
            id: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn load(width: i32, height: i32, data: &[u8]) -> Image {
        Image {
            id: 0,
            width,
            height,
        }
    }
}
