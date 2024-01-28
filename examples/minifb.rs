use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut ctx = mini_ui::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let commands = ctx.run(|ui| {
            if ui.button() {
                println!("Hi");
            }

            ui.label("hello");
        });

        mini_ui::render(&commands);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
