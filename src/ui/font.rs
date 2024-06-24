#[derive(Clone, Copy, Debug, Default)]
pub struct Font {
    pub(crate) id: i32,
}

impl Font {
    pub fn load() -> Font {
        Font { id: 0 }
    }
}
