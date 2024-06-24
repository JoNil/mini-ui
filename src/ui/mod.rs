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

use crate::math::{vec2, vec4, Vec2};

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

pub struct OuiResources {}

pub struct OuiContext {
    resources: OuiResources,
    state: HashMap<usize, OuiState>,
    _marker: PhantomData<*const ()>,
}

impl OuiContext {
    pub fn new() -> OuiContext {
        OuiContext {
            resources: OuiResources {},
            state: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

struct OuiState {
    bounding_boxes: Vec<(Id, BoundingBox)>,
    mouse_pressed: bool,
    mouse_pressed_id: Id,
    mouse_released_time: Instant,
}

impl Default for OuiState {
    fn default() -> OuiState {
        OuiState {
            bounding_boxes: Vec::new(),
            mouse_pressed: false,
            mouse_pressed_id: Id::none(),
            mouse_released_time: Instant::now() - Duration::from_millis(1000),
        }
    }
}

#[must_use = "You should call .show()"]
pub struct Oui<'ctx> {
    ctx: &'ctx mut OuiContext,
    style: Style,
}

impl<'ctx> Oui<'ctx> {
    #[inline]
    pub fn new(ctx: &'ctx mut OuiContext) -> Oui {
        Oui {
            ctx,
            style: Style::default(),
        }
    }

    #[inline]
    pub fn style(mut self, style: Style) -> Oui<'ctx> {
        self.style = style;
        self
    }

    pub fn show(self, func: impl FnOnce(&mut Ui)) {
        let ctx_key = &func as *const _ as usize;
        let state = self.ctx.state.entry(ctx_key).or_default();

        let mut draw = DrawApi::new();
        let mouse_pos = Vec2::ZERO;

        let responses = {
            let mut found_first = false;
            let pressed = false;
            let released = false;

            let double_clicked = !state.mouse_pressed
                && pressed
                && state.mouse_released_time.elapsed() < Duration::from_millis(500);

            if pressed {
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
                        },
                    )
                })
                .collect::<Vec<(Id, Response)>>()
        };

        let style = self.style.align(self.style.align.unwrap_or(Align::Left));

        let mut ui = Ui {
            resources: &mut self.ctx.resources,
            draw: &mut draw,
            responses: &responses,
            style,
            current_line: Vec::new(),
            lines: Vec::new(),
            parent_id: Id::from_vec2(Vec2::ZERO),
        };

        frame::show(&mut ui, false, style, None, None, func);

        let element = &ui.current_line[0];

        let bounding_box = spacing::bounding_box(
            element.content_box,
            element.style.margin,
            element.style.padding,
        );

        let cursor = Vec2::ZERO;
        let element_cursor = cursor
            + vec2(
                element.style.padding.left + element.style.margin.left,
                -element.style.padding.top - element.style.margin.top,
            );

        for pass in 0..2 {
            draw.set_pass(pass);

            if element.style.debug && pass == 1 {
                debug::draw_content_boxes(&draw, cursor, element, element.content_box.y);
            }

            if let Some(render) = element.render.as_ref() {
                render(&mut draw, element_cursor, element.content_box.y);
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
