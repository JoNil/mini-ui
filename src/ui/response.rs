use crate::math::Vec2;

#[derive(Copy, Clone, Default)]
pub struct Response {
    pub hovered: bool,
    pub pressed: bool,
    pub released: bool,
    pub double_clicked: bool,
    pub held: bool,
    pub relative_mouse_pos: Vec2,
}
