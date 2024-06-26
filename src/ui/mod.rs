use crate::{
    cairo::Context,
    math::{vec2, vec4, Vec2},
    window::{MouseMode, Window},
};
use bounding_box::BoundingBox;
use draw_api::DrawApi;
use id::Id;
use std::{
    collections::HashMap,
    marker::PhantomData,
    mem,
    time::{Duration, Instant},
};

mod bounding_box;
mod color;
mod debug;
mod draw_api;
mod font;
mod frame;
mod id;
mod image;
mod response;
mod spacing;
mod style;
mod textedit;
mod ui;

pub use font::Font;
pub use frame::FrameStyle;
pub use image::Image;
pub use response::Response;
pub use spacing::Spacing;
pub use style::Style;
pub use textedit::TextEdit;
pub use ui::Ui;

#[derive(Clone, Copy, Debug)]
pub enum Align {
    Right,
    Left,
    Center,
}

#[derive(Clone, Copy, Debug)]
pub enum VertAlign {
    Top,
    Bottom,
    Center,
}

pub struct OuiContext {
    state: HashMap<usize, OuiState>,
    _marker: PhantomData<*const ()>,
}

impl OuiContext {
    pub fn new() -> OuiContext {
        OuiContext {
            state: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

struct OuiState {
    bounding_boxes: Vec<(Id, BoundingBox)>,
    mouse_pressed: bool,
    mouse_pressed_pos: Vec2,
    mouse_pressed_id: Id,
    mouse_released_time: Instant,
}

impl Default for OuiState {
    fn default() -> OuiState {
        OuiState {
            bounding_boxes: Vec::new(),
            mouse_pressed: false,
            mouse_pressed_pos: Vec2::ZERO,
            mouse_pressed_id: Id::none(),
            mouse_released_time: Instant::now() - Duration::from_millis(1000),
        }
    }
}

#[must_use = "You should call .show()"]
pub struct Oui<'ctx> {
    ctx: &'ctx mut OuiContext,
    style: Style,
    fill: bool,
}

impl<'ctx> Oui<'ctx> {
    #[inline]
    pub fn new(ctx: &'ctx mut OuiContext) -> Oui {
        Oui {
            ctx,
            style: Style::default(),
            fill: true,
        }
    }

    #[inline]
    pub fn style(mut self, style: Style) -> Oui<'ctx> {
        self.style = style;
        self
    }

    #[inline]
    pub fn fill(mut self, fill: bool) -> Oui<'ctx> {
        self.fill = fill;
        self
    }

    pub fn show(
        self,
        window: &Window,
        surface_context: &Context,
        screen_size: Vec2,
        func: impl FnOnce(&mut Ui),
    ) {
        let ctx_key = &func as *const _ as usize;
        let state = self.ctx.state.entry(ctx_key).or_default();

        let mut draw = DrawApi::new(surface_context);

        let mouse_pos = Vec2::from(window.get_mouse_pos(MouseMode::Pass).unwrap_or_default());

        let responses = {
            let mut found_first = false;
            let pressed = false; // TODO
            let released = false; // TODO

            let double_clicked = !state.mouse_pressed
                && pressed
                && (state.mouse_pressed_pos - mouse_pos).length() < 10.0
                && state.mouse_released_time.elapsed() < Duration::from_millis(500);

            if pressed {
                state.mouse_pressed_pos = mouse_pos;
                state.mouse_pressed = true;
            }

            if released {
                state.mouse_pressed = false;
                state.mouse_released_time = Instant::now();
            }

            state
                .bounding_boxes
                .iter()
                .rev()
                .map(|(id, bb)| {
                    let hovered = bb.intersect(mouse_pos);

                    let relative_mouse_pos = mouse_pos - bb.top_left;

                    let pressed = !found_first && hovered && pressed;
                    let released =
                        !found_first && hovered && released && state.mouse_pressed_id == *id;
                    let double_clicked = !found_first && hovered && double_clicked;
                    let held = !found_first
                        && hovered
                        && state.mouse_pressed
                        && state.mouse_pressed_id == *id;

                    if pressed {
                        state.mouse_pressed_id = *id;
                    }

                    if pressed || released || double_clicked || held {
                        found_first = true;
                    }

                    (
                        *id,
                        Response {
                            hovered,
                            pressed,
                            released,
                            double_clicked,
                            held,
                            relative_mouse_pos,
                        },
                    )
                })
                .collect::<Vec<(Id, Response)>>()
        };

        let style = self.style.align(self.style.align.unwrap_or(Align::Left));

        let mut ui = Ui {
            draw: &mut draw,
            responses: &responses,
            style,
            current_line: Vec::new(),
            lines: Vec::new(),
            parent_id: Id::from_vec2(vec2(0.0, 0.0)),
        };

        frame::show(&mut ui, false, style, None, None, true, func);

        let element = &ui.current_line[0];

        if self.fill {
            let content_box = element.content_box.get();
            let mut extra_x = 0.0;

            if content_box.x < screen_size.x {
                extra_x = screen_size.x - content_box.x;
                element
                    .content_box
                    .set(vec2(screen_size.x, element.content_box.get().y));
            }

            if content_box.y < screen_size.y {
                element
                    .content_box
                    .set(vec2(element.content_box.get().x, screen_size.y));
            }

            if let Some(update_with_max_width) = &element.update_with_max_width {
                update_with_max_width(vec2(extra_x, 0.0));
            }
        }

        let cursor = Vec2::ZERO;
        let element_cursor = cursor
            + vec2(
                element.style.padding.left + element.style.margin.left,
                -element.style.padding.top - element.style.margin.top,
            );

        for pass in 0..2 {
            draw.set_pass(pass);

            if element.style.debug && pass == 1 {
                debug::draw_content_boxes(&draw, cursor, element, element.content_box.get().y);
            }

            if let Some(render) = element.render.as_ref() {
                render(&mut draw, element_cursor, element.content_box.get());
            }
        }

        let bounding_boxes = mem::take(&mut draw.boxes);

        if style.debug {
            let mut found_first = false;

            for (_, bb) in bounding_boxes.iter().rev() {
                if !found_first && bb.intersect(mouse_pos) {
                    draw.rectangle(bb.top_left, bb.size, vec4(0.0, 0.0, 1.0, 0.2));
                    found_first = true;
                }

                draw.rectangle_border(bb.top_left, bb.size, 1.0, vec4(1.0, 0.0, 0.0, 1.0));
            }

            draw.circle(
                mouse_pos - vec2(2.0, -2.0),
                1.0,
                2.0,
                vec4(0.0, 1.0, 0.0, 1.0),
                8,
            );
        }

        state.bounding_boxes = bounding_boxes;
    }
}
