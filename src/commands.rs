#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String, // Todo: Move to arena
}

#[derive(Debug)]
pub enum Command {
    Rectangle(Rectangle),
    Text(Text),
}
