use crate::math::{vec2, vec4, Vec2, Vec4};
use crate::ui::{
    color::{held_color, hover_color},
    draw_api::DrawApi,
    frame,
    id::Id,
    spacing, Font, Image, Response, Spacing, Style,
};
use std::{borrow::Cow, cell::Cell, f32::INFINITY, mem, rc::Rc};

type Draw<'a> = dyn Fn(&mut DrawApi, Vec2, Vec2) + 'a;
type UpdateWithMaxWidth = dyn Fn(Vec2);

pub(crate) struct Element<'a> {
    pub(crate) content_box: Rc<Cell<Vec2>>,
    pub(crate) id: Option<Id>,
    pub(crate) border_extra: Vec2,
    pub(crate) flex_x: bool,
    pub(crate) flex_y: bool,
    pub(crate) style: Style,
    pub(crate) render: Option<Box<Draw<'a>>>,
    pub(crate) update_with_max_width: Option<Box<UpdateWithMaxWidth>>,
}

pub struct Ui<'a, 'draw, 'show> {
    pub(crate) draw: &'show mut DrawApi<'draw>,
    pub(crate) responses: &'show Vec<(Id, Response)>,
    pub(crate) style: Style,
    pub(crate) current_line: Vec<Element<'a>>,
    pub(crate) lines: Vec<Vec<Element<'a>>>,
    pub(crate) parent_id: Id,
}

impl<'a, 'draw, 'show> Ui<'a, 'draw, 'show> {
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
        let text = text.into();

        let content_box = self
            .draw
            .calc_text_size(text.as_ref(), text_height, INFINITY, font);

        self.canvas(content_box, move |draw, cursor, content_box| {
            draw.text(
                text.as_ref(),
                cursor,
                content_box,
                text_height,
                super::Align::Center,
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

        self.canvas(content_box, move |draw, cursor, content_box| {
            draw.image(cursor, content_box, image.clone());
        });
    }

    #[inline]
    pub fn rectangle(&mut self, size: Vec2, color: Vec4) {
        self.canvas(size, move |api, cursor, size| {
            api.rectangle(cursor, size, color);
        });
    }

    pub fn rounded_rectangle(&mut self, size: Vec2, rounding: f32, color: Vec4) {
        self.canvas(size, move |api, cursor, size| {
            api.rectangle_rounded(cursor, size, rounding, color);
        });
    }

    #[inline]
    pub fn separator(&mut self) {
        let style = self.style;
        let text_height = style.text_height;
        let font = style.font.unwrap_or_default();

        let content_box = self.draw.calc_text_size("  ", text_height, INFINITY, font);

        self.flex_canvas(content_box, move |api, cursor, content_box| {
            let middle = cursor.x + content_box.x / 2.0;
            let start = cursor.y;
            let end = cursor.y - content_box.y;

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

        let content_box = self.draw.calc_text_size(" ", text_height, INFINITY, font);

        self.current_line.push(Element {
            content_box: Rc::new(Cell::new(content_box)),
            id: None,
            border_extra: Vec2::ZERO,
            flex_x: false,
            flex_y: false,
            style: style.margin(Spacing::ZERO).padding(Spacing::ZERO),
            render: None,
            update_with_max_width: None,
        });
    }

    #[inline]
    pub fn empty_area(&mut self, size: Vec2) {
        let style = self.style;

        self.current_line.push(Element {
            content_box: Rc::new(Cell::new(size)),
            id: None,
            border_extra: Vec2::ZERO,
            flex_x: false,
            flex_y: false,
            style: style.margin(Spacing::ZERO).padding(Spacing::ZERO),
            render: None,
            update_with_max_width: None,
        });
    }

    #[inline]
    pub fn frame(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, false, style, None, None, true, func);
    }

    #[inline]
    pub fn area(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, true, style, None, None, true, func);
    }

    #[inline]
    pub fn no_expand_frame(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, false, style, None, None, false, func);
    }

    #[inline]
    pub fn no_expand_area(&mut self, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, true, style, None, None, false, func);
    }

    #[inline]
    pub fn sized_frame(&mut self, size: Vec2, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, false, style, Some(size), None, false, func);
    }

    #[inline]
    pub fn sized_area(&mut self, size: Vec2, style: Style, func: impl FnOnce(&mut Ui)) {
        frame::show(self, true, style, Some(size), None, false, func);
    }

    #[inline]
    pub fn canvas(&mut self, content_box: Vec2, draw: impl Fn(&mut DrawApi, Vec2, Vec2) + 'a) {
        self.push_ui_element(
            self.style,
            content_box,
            None,
            Vec2::ZERO,
            false,
            false,
            draw,
        );
    }

    #[inline]
    pub fn interactable_canvas(
        &mut self,
        content_box: Vec2,
        draw: impl Fn(&mut DrawApi, Vec2, Vec2) + 'a,
    ) -> Response {
        let response = self.response();

        self.push_ui_element(
            self.style,
            content_box,
            Some(self.current_id()),
            Vec2::ZERO,
            false,
            false,
            draw,
        );

        response
    }

    #[inline]
    pub fn flex_canvas(&mut self, content_box: Vec2, draw: impl Fn(&mut DrawApi, Vec2, Vec2) + 'a) {
        self.push_ui_element(self.style, content_box, None, Vec2::ZERO, false, true, draw);
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

        frame::show(
            self,
            false,
            style,
            size,
            Some(self.current_id()),
            true,
            func,
        );

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

        self.interactable_canvas(content_box, move |draw, cursor, content_box| {
            let tint = if response.held {
                vec4(1.0, 1.0, 1.0, 0.6)
            } else if response.hovered {
                vec4(1.0, 1.0, 1.0, 0.8)
            } else {
                Vec4::ONE
            };

            draw.set_tint(tint);
            draw.image(cursor, content_box, image.clone());
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
            .find_map(|(id, r)| {
                if *id == self.current_id() {
                    Some(r)
                } else {
                    None
                }
            })
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn push_ui_element(
        &mut self,
        style: Style,
        content_box: Vec2,
        id: Option<Id>,
        border_extra: Vec2,
        flex_x: bool,
        flex_y: bool,
        render: impl Fn(&mut DrawApi, Vec2, Vec2) + 'a,
    ) {
        self.current_line.push(Element {
            content_box: Rc::new(Cell::new(content_box)),
            id,
            border_extra,
            flex_x,
            flex_y,
            style,
            render: Some(Box::new(render)),
            update_with_max_width: None,
        });
    }

    #[inline]
    pub fn horizontal_spring(&mut self) {
        self.current_line.push(Element {
            content_box: Rc::new(Cell::new(Vec2::ZERO)),
            id: None,
            border_extra: Vec2::ZERO,
            flex_x: true,
            flex_y: false,
            style: self.style.margin(Spacing::ZERO).padding(Spacing::ZERO),
            render: None,
            update_with_max_width: None,
        });
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn layout(
        mut self,
    ) -> (
        Vec<Vec<Element<'a>>>,
        Vec<Rc<Cell<Vec2>>>,
        Vec2,
        Vec<Box<UpdateWithMaxWidth>>,
    ) {
        self.lines.push(self.current_line);

        if self.lines.len() > 1 && self.lines.last().unwrap().is_empty() {
            self.lines.pop();
        }

        let mut line_sizes = Vec::new();
        let mut total_size = Vec2::ZERO;

        for line in &self.lines {
            let mut line_size = Vec2::ZERO;

            for element in line.iter() {
                let bounding_box = spacing::bounding_box(
                    element.content_box.get(),
                    element.style.margin,
                    element.style.padding,
                );

                line_size.x += bounding_box.width();
                line_size.y = line_size.y.max(bounding_box.height());
            }

            line_sizes.push(Rc::new(Cell::new(line_size)));

            total_size.x = total_size.x.max(line_size.x);
            total_size.y += line_size.y;
        }

        let mut flex_children = Vec::<Box<UpdateWithMaxWidth>>::new();

        for (line, line_size) in self.lines.iter_mut().zip(line_sizes.iter_mut()) {
            let mut flex_count = 0;

            for element in line.iter() {
                if element.flex_x {
                    flex_count += 1;
                }
            }

            if flex_count > 0 {
                let extra_line_width = total_size.x - line_size.get().x;
                let extra_width_per_flex = extra_line_width / flex_count as f32;

                let mut line_flex_elements = Vec::new();

                for element in line {
                    if element.flex_x {
                        let extra_size = vec2(extra_width_per_flex, 0.0);

                        element
                            .content_box
                            .set(element.content_box.get() + extra_size);

                        if let Some(update_with_max_width) = element.update_with_max_width.as_ref()
                        {
                            update_with_max_width(extra_size);
                        }

                        line_flex_elements.push((
                            element.content_box.clone(),
                            element.update_with_max_width.take(),
                        ));
                    }
                }

                flex_children.push(Box::new({
                    let line_size = line_size.clone();
                    move |extra_size: Vec2| {
                        let extra_size = vec2(extra_size.x / flex_count as f32, extra_size.y);

                        for (content_box, update_with_max_width) in &line_flex_elements {
                            content_box.set(content_box.get() + extra_size);

                            if let Some(update_with_max_width) = update_with_max_width.as_ref() {
                                update_with_max_width(extra_size);
                            }
                        }

                        line_size.set(line_size.get() + extra_size);
                    }
                }));

                line_size.set(line_size.get() + vec2(extra_line_width, 0.0));
            }
        }

        (self.lines, line_sizes, total_size, flex_children)
    }
}
