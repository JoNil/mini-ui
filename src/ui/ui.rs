use crate::{
    math::{vec2, vec4, Vec2, Vec4},
    ui::{
        color::{held_color, hover_color},
        draw_api::DrawApi,
        frame,
        id::Id,
        spacing, Font, Image, OuiResources, Response, Style,
    },
};
use std::{borrow::Cow, mem};

use super::Align;

type UiDraw<'a> = dyn Fn(&mut DrawApi, Vec2, f32) + 'a;

pub(crate) struct UiArea<'a> {
    pub(crate) content_box: Vec2,
    pub(crate) id: Option<Id>,
    pub(crate) border_extra: Vec2,
    pub(crate) flex_y: bool,
    pub(crate) style: Style,
    pub(crate) render: Option<Box<UiDraw<'a>>>,
}

pub struct Ui<'a, 'ctx, 'show> {
    pub(crate) resources: &'ctx OuiResources,
    pub(crate) draw: &'show mut DrawApi,
    pub(crate) responses: &'show Vec<(Id, Response)>,
    pub(crate) style: Style,
    pub(crate) current_line: Vec<UiArea<'a>>,
    pub(crate) lines: Vec<Vec<UiArea<'a>>>,
    pub(crate) parent_id: Id,
}

impl<'a, 'ctx, 'show> Ui<'a, 'ctx, 'show> {
    pub fn style(&self) -> Style {
        self.style
    }

    #[inline]
    pub fn with_style(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        let old = self.style;
        self.style = style;
        func(self);
        self.style = old;
    }

    #[inline]
    pub fn colored_text_with_font(
        &mut self,
        text: impl Into<Cow<'a, str>>,
        color: Vec4,
        font: Font,
    ) {
        let style = self.style;
        let text_height = style.text_height;
        let extra_top = style.text_height * 0.1;
        let text = text.into();

        let content_box = self
            .draw
            .calc_text_size(text.as_ref(), text_height, f32::INFINITY, font)
            + vec2(0.0, extra_top);

        self.canvas(content_box, move |draw, cursor| {
            draw.text(
                text.as_ref(),
                cursor + vec2(0.0, -extra_top),
                content_box + vec2(0.001, 0.001),
                text_height,
                Align::Center,
                color,
                font,
            );
        });
    }

    #[inline]
    pub fn colored_text(&mut self, text: impl Into<Cow<'a, str>>, color: Vec4) {
        self.colored_text_with_font(text, color, self.style.font.unwrap_or_default())
    }

    #[inline]
    pub fn active_text(&mut self, text: impl Into<Cow<'a, str>>, active: bool) {
        let color = if active {
            self.style.text_color
        } else {
            self.style.inactive_color
        };
        self.colored_text(text, color)
    }

    #[inline]
    pub fn text(&mut self, text: impl Into<Cow<'a, str>>) {
        self.colored_text(text, self.style.text_color);
    }

    #[inline]
    pub fn image(&mut self, image: Image) {
        let style = self.style;
        let height = style.text_height;
        let width = height * image.width as f32 / image.height as f32;

        let content_box = vec2(width, height);

        self.canvas(content_box, move |draw, cursor| {
            draw.image(cursor, content_box, image);
        });
    }

    #[inline]
    pub fn rectangle(&mut self, size: Vec2, color: Vec4) {
        self.canvas(size, move |api, cursor| {
            api.rectangle(cursor, size, color);
        });
    }

    pub fn rounded_rectangle(&mut self, size: Vec2, rounding: f32, color: Vec4) {
        self.canvas(size, move |api, cursor| {
            api.rectangle_rounded(cursor, size, rounding, color);
        });
    }

    #[inline]
    pub fn separator(&mut self) {
        let style = self.style;
        let text_height = style.text_height;
        let font = style.font.unwrap_or_default();

        let content_box = self
            .draw
            .calc_text_size("  ", text_height, f32::INFINITY, font);

        self.flex_canvas(content_box, move |api, cursor, line_height| {
            let middle = cursor.x + content_box.x / 2.0;
            let start = cursor.y;
            let end = cursor.y - line_height;

            api.line(
                vec2(middle, start),
                vec2(middle, end),
                content_box.x / 10.0,
                style.border_color,
            );
        });
    }

    #[inline]
    pub fn next_line(&mut self) {
        self.lines.push(mem::take(&mut self.current_line));
    }

    #[inline]
    pub fn space(&mut self) {
        let style = self.style;
        let text_height = style.text_height;
        let font = style.font.unwrap_or_default();

        let content_box = self
            .draw
            .calc_text_size(" ", text_height, f32::INFINITY, font);

        self.current_line.push(UiArea {
            content_box,
            id: None,
            border_extra: Vec2::ZERO,
            flex_y: false,
            style,
            render: None,
        });
    }

    #[inline]
    pub fn empty_area(&mut self, size: Vec2) {
        let style = self.style;

        self.current_line.push(UiArea {
            content_box: size,
            id: None,
            border_extra: Vec2::ZERO,
            flex_y: false,
            style,
            render: None,
        });
    }

    #[inline]
    pub fn frame(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, false, style, None, None, func);
    }

    #[inline]
    pub fn area(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, true, style, None, None, func);
    }

    #[inline]
    pub fn sized_frame(&mut self, size: Vec2, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, false, style, Some(size), None, func);
    }

    #[inline]
    pub fn sized_area(&mut self, size: Vec2, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, true, style, Some(size), None, func);
    }

    #[inline]
    pub fn canvas(&mut self, content_box: Vec2, draw: impl Fn(&mut DrawApi, Vec2) + 'a) {
        self.push_ui_area(
            self.style,
            content_box,
            None,
            Vec2::ZERO,
            false,
            move |draw_api, cursor, _| draw(draw_api, cursor),
        );
    }

    #[inline]
    pub fn interactable_canvas(
        &mut self,
        content_box: Vec2,
        draw: impl Fn(&mut DrawApi, Vec2) + 'a,
    ) -> Response {
        let response = self.response();

        self.push_ui_area(
            self.style,
            content_box,
            Some(self.current_id()),
            Vec2::ZERO,
            false,
            move |draw_api, cursor, _| draw(draw_api, cursor),
        );

        response
    }

    #[inline]
    pub fn flex_canvas(&mut self, content_box: Vec2, draw: impl Fn(&mut DrawApi, Vec2, f32) + 'a) {
        self.push_ui_area(self.style, content_box, None, Vec2::ZERO, true, draw);
    }

    #[inline]
    fn button_inner(&mut self, size: Option<Vec2>, func: impl FnOnce(&mut Ui)) -> Response {
        let response = self.response();
        let style = self.style;

        let style = if response.held {
            style.frame_color(held_color(style.frame_color))
        } else if response.hovered {
            style.frame_color(hover_color(style.frame_color))
        } else {
            style
        };

        frame::show(self, false, style, size, Some(self.current_id()), func);

        response
    }

    #[inline]
    pub fn button(&mut self, text: impl Into<Cow<'a, str>>) -> Response {
        let text = text.into().into_owned();
        self.button_inner(None, move |ui| ui.text(text))
    }

    #[inline]
    pub fn sized_button(&mut self, size: Vec2, text: impl Into<Cow<'a, str>>) -> Response {
        let text = text.into().into_owned();
        self.button_inner(Some(size), move |ui| ui.text(text))
    }

    #[inline]
    pub fn image_button(&mut self, image: Image) -> Response {
        let response = self.response();
        let style = self.style;
        let height = style.text_height;
        let width = height * image.width as f32 / image.height as f32;

        let content_box = vec2(width, height);

        self.interactable_canvas(content_box, move |draw, cursor| {
            let tint = if response.held {
                vec4(1.0, 1.0, 1.0, 0.6)
            } else if response.hovered {
                vec4(1.0, 1.0, 1.0, 0.8)
            } else {
                Vec4::ONE
            };

            draw.set_tint(tint);
            draw.image(cursor, content_box, image);
            draw.set_tint(Vec4::ONE);
        })
    }

    #[inline]
    pub(crate) fn current_id(&self) -> Id {
        self.parent_id.with_child(Id::from_vec2(vec2(
            self.current_line.len() as _,
            self.lines.len() as _,
        )))
    }

    #[inline]
    pub fn response(&self) -> Response {
        self.responses
            .iter()
            .find_map(|(id, resp)| (id == &self.current_id()).then_some(*resp))
            .unwrap_or_default()
    }

    #[inline]
    pub(crate) fn push_ui_area(
        &mut self,
        style: Style,
        content_box: Vec2,
        id: Option<Id>,
        border_extra: Vec2,
        flex_y: bool,
        render: impl Fn(&mut DrawApi, Vec2, f32) + 'a,
    ) {
        self.current_line.push(UiArea {
            content_box,
            id,
            border_extra,
            flex_y,
            style,
            render: Some(Box::new(render)),
        });
    }

    pub(crate) fn layout(mut self) -> (Vec<Vec<UiArea<'a>>>, Vec<Vec2>, Vec2) {
        self.lines.push(self.current_line);

        if self.lines.len() > 1 && self.lines.last().unwrap().is_empty() {
            self.lines.pop();
        }

        let mut line_sizes = Vec::new();
        let mut total_size = Vec2::ZERO;

        for line in &self.lines {
            let mut line_size = Vec2::ZERO;

            for op in line.iter() {
                let bounding_box =
                    spacing::bounding_box(op.content_box, op.style.margin, op.style.padding);

                line_size.x += bounding_box.width();
                line_size.y = line_size.y.max(bounding_box.height());
            }

            line_sizes.push(line_size);

            total_size.x = total_size.x.max(line_size.x);
            total_size.y += line_size.y;
        }

        (self.lines, line_sizes, total_size)
    }
}
