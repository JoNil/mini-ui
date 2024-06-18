use mini_ui::window::{Key, Window, WindowOptions};
use parley::{
    layout::{Alignment, GlyphRun, Layout},
    style::{FontStack, FontWeight, StyleProperty},
    FontContext, LayoutContext,
};
use skrifa::{
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{DrawSettings, HintingInstance, HintingMode, OutlinePen},
    raw::FontRef,
    GlyphId, MetadataProvider, OutlineGlyph, OutlineGlyphCollection,
};
use std::slice;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, PixmapMut, Transform};

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

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_buffer = draw();
        let data = new_buffer.as_ptr();
        window
            .update_with_buffer(
                unsafe { slice::from_raw_parts(data as *const u32, new_buffer.len() / 4) },
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }
}

fn draw() -> Vec<u8> {
    let text = String::from(
        "Some text here. Let's make it a bit longer so that line wrapping kicks in. And also some اللغة العربية arabic text.",
    );

    let display_scale = 1.0;
    let max_advance = Some(200.0 * display_scale);

    let foreground_color = [0, 0, 0];
    let background_color = [255, 255, 255];

    let padding = 20;

    let mut font_cx = FontContext::default();
    let mut layout_cx = LayoutContext::new();

    let mut builder = layout_cx.ranged_builder(&mut font_cx, &text, display_scale);

    let brush_style = StyleProperty::Brush(foreground_color);
    builder.push_default(&brush_style);

    let font_stack = FontStack::Source("system-ui");
    let font_stack_style = StyleProperty::FontStack(font_stack);
    builder.push_default(&font_stack_style);
    builder.push_default(&StyleProperty::LineHeight(1.3));
    builder.push_default(&StyleProperty::FontSize(16.0));

    let bold = FontWeight::new(1600.0);
    let bold_style = StyleProperty::FontWeight(bold);
    builder.push(&bold_style, 0..4);

    let mut layout: Layout<[u8; 3]> = builder.build();
    layout.break_all_lines(max_advance, Alignment::Start);

    let mut img = Pixmap::new(WIDTH as _, HEIGHT as _).unwrap();
    img.fill(Color::from_rgba8(
        background_color[0],
        background_color[1],
        background_color[2],
        255,
    ));

    let mut pen = TinySkiaPen::new(img.as_mut());

    for line in layout.lines() {
        for glyph_run in line.glyph_runs() {
            render_glyph_run(&glyph_run, &mut pen, padding);
        }
    }

    img.take()
}

fn render_glyph_run(glyph_run: &GlyphRun<[u8; 3]>, pen: &mut TinySkiaPen<'_>, padding: u32) {
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;
    let run = glyph_run.run();
    let font = run.font();
    let font_size = run.font_size();

    let normalized_coords = run
        .normalized_coords()
        .iter()
        .map(|coord| NormalizedCoord::from_bits(*coord))
        .collect::<Vec<_>>();

    let font_collection_ref = font.data.as_ref();
    let font_ref = FontRef::from_index(font_collection_ref, font.index).unwrap();
    let outlines = font_ref.outline_glyphs();

    for glyph in glyph_run.glyphs() {
        let glyph_x = run_x + glyph.x + padding as f32;
        let glyph_y = run_y - glyph.y + padding as f32;
        run_x += glyph.advance;

        let glyph_id = GlyphId::from(glyph.id);
        let glyph_outline = outlines.get(glyph_id).unwrap();

        pen.set_origin(glyph_x, glyph_y);
        pen.set_color(color);
        pen.draw_glyph(&glyph_outline, font_size, &normalized_coords, &outlines);
    }
}

struct TinySkiaPen<'a> {
    pixmap: PixmapMut<'a>,
    x: f32,
    y: f32,
    paint: Paint<'static>,
    open_path: PathBuilder,
}

impl TinySkiaPen<'_> {
    fn new(pixmap: PixmapMut) -> TinySkiaPen {
        TinySkiaPen {
            pixmap,
            x: 0.0,
            y: 0.0,
            paint: Paint::default(),
            open_path: PathBuilder::new(),
        }
    }

    fn set_origin(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn set_color(&mut self, color: [u8; 3]) {
        self.paint
            .set_color(Color::from_rgba8(color[0], color[1], color[2], 255));
    }

    fn draw_glyph(
        &mut self,
        glyph: &OutlineGlyph<'_>,
        size: f32,
        normalized_coords: &[NormalizedCoord],
        outlines: &OutlineGlyphCollection,
    ) {
        let location_ref = LocationRef::new(normalized_coords);

        let hints = HintingInstance::new(
            outlines,
            Size::new(size),
            location_ref,
            HintingMode::Smooth {
                lcd_subpixel: None,
                preserve_linear_metrics: true,
            },
        )
        .unwrap();
        let settings = DrawSettings::hinted(&hints, false);
        glyph.draw(settings, self).unwrap();

        let builder = core::mem::replace(&mut self.open_path, PathBuilder::new());
        if let Some(path) = builder.finish() {
            self.pixmap.fill_path(
                &path,
                &self.paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }
}

impl OutlinePen for TinySkiaPen<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.open_path.move_to(self.x + x, self.y - y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.open_path.line_to(self.x + x, self.y - y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.open_path
            .quad_to(self.x + cx0, self.y - cy0, self.x + x, self.y - y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.open_path.cubic_to(
            self.x + cx0,
            self.y - cy0,
            self.x + cx1,
            self.y - cy1,
            self.x + x,
            self.y - y,
        );
    }

    fn close(&mut self) {
        self.open_path.close();
    }
}
