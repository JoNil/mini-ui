#[derive(Copy, Clone, Default)]
pub struct Response {
    pub hovered: bool,
    pub pressed: bool,
    pub released: bool,
    pub double_clicked: bool,
    pub held: bool,
}
