use core::slice;
use mini_ui::{
    cairo::{self, Context, Format, ImageSurface},
    window::{Key, Window, WindowOptions},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut surface = ImageSurface::create(Format::ARgb32, WIDTH as _, HEIGHT as _)
        .expect("Can't create surface");

    {
        let context = Context::new(&surface).unwrap();

        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint();

        context.select_font_face(
            "Helvetica",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Normal,
        );
        context.set_font_size(40.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.move_to(50.0, 100.0);
        context.show_text("Hello, Cairo!");
    }

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let data = surface.data().unwrap();
        window
            .update_with_buffer(
                unsafe { slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 4) },
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }
}
