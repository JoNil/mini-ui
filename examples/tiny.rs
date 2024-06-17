use mini_ui::window::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
