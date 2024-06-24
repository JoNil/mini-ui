use crate::{math::{vec2, vec4, Vec2}, ui::{
    bounding_box::BoundingBox, debug, draw_api::DrawApi, id::Id, spacing, Align, Image, Style, Ui,
    VertAlign,
}};

#[derive(Clone, Copy, Debug)]
pub enum FrameStyle {
    None,
    Rectangle,
    RoundedRectangle(f32),
    Circle,
    Image(Image),
}

pub(crate) fn show(
    ui: &mut Ui,
    hide_frame: bool,
    style: Style,
    fixed_size: Option<Vec2>,
    id: Option<Id>,
    func: impl FnOnce(&mut Ui),
) {
    let current_id = ui.current_id();

    let mut child_ui = Ui {
        resources: ui.resources,
        draw: ui.draw,
        responses: ui.responses,
        style,
        current_line: Vec::new(),
        lines: Vec::new(),
        parent_id: ui.parent_id.with_child(current_id),
    };
    func(&mut child_ui);

    let (lines, mut line_sizes, mut total_size) = child_ui.layout();

    let border_extra = if hide_frame {
        Vec2::ZERO
    } else {
        border_extra(&style, total_size)
    };

    total_size += 2.0 * border_extra;

    if let Some(fixed_size) = fixed_size {
        if fixed_size.x > 0.0 {
            total_size.x = fixed_size.x;
        }
        if fixed_size.y > 0.0 {
            if fixed_size.y > total_size.y {
                line_sizes.last_mut().unwrap().y += fixed_size.y - total_size.y;
            }

            total_size.y = fixed_size.y;
        }
    }

    ui.push_ui_area(
        style,
        total_size,
        id,
        border_extra,
        false,
        move |draw, parent_cursor, _| {
            let pass = draw.pass();

            {
                if pass == 0 {
                    draw.set_tint_internal(style.shadow_color);
                    draw.set_scale(style.shadow_scale);
                }

                if !hide_frame && (pass == 1 || (pass == 0 && style.shadow_dir.is_some())) {
                    draw_frame(
                        draw,
                        &style,
                        parent_cursor - vec2(style.padding.left, -style.padding.top),
                        total_size + vec2(style.padding.width(), style.padding.height()),
                    );
                }

                if pass == 0 {
                    draw.set_tint_internal(vec4(1.0, 1.0, 1.0, 1.0));
                    draw.set_scale(1.0);
                }
            }

            if pass == 1 {
                for pass in 0..2 {
                    draw.set_pass(pass);

                    let mut cursor_y = -border_extra.y;

                    for (line, line_size) in lines.iter().zip(line_sizes.iter()) {
                        let mut cursor_x = match style.align.unwrap_or(Align::Left) {
                            Align::Right => total_size.x - line_size.x - border_extra.x,
                            Align::Left => border_extra.x,
                            Align::Center => total_size.x / 2.0 - line_size.x / 2.0,
                        };

                        for element in line {
                            let bounding_box = spacing::bounding_box(
                                element.content_box,
                                element.style.margin,
                                element.style.padding,
                            );

                            let content_height = line_size.y
                                - element.style.margin.height()
                                - element.style.padding.height();

                            let align_y = if element.flex_y {
                                0.0
                            } else {
                                match element.style.vert_align {
                                    VertAlign::Top => 0.0,
                                    VertAlign::Bottom => element.content_box.y - content_height,
                                    VertAlign::Center => {
                                        element.content_box.y / 2.0 - content_height / 2.0
                                    }
                                }
                            };

                            let cursor = vec2(cursor_x, cursor_y + align_y);

                            if element.style.debug && pass == 1 {
                                debug::draw_content_boxes(
                                    draw,
                                    parent_cursor + cursor,
                                    element,
                                    content_height,
                                )
                            }

                            let cursor = cursor
                                + vec2(
                                    element.style.padding.left + element.style.margin.left,
                                    -element.style.padding.top - element.style.margin.top,
                                )
                                + if pass == 0 {
                                    element.style.shadow_dir.unwrap_or(Vec2::ZERO)
                                } else {
                                    Vec2::ZERO
                                };

                            if pass == 0 {
                                draw.set_tint_internal(element.style.shadow_color);
                                draw.set_scale(element.style.shadow_scale);
                            }

                            if pass == 1 || (pass == 0 && element.style.shadow_dir.is_some()) {
                                let element_cursor = parent_cursor + cursor;

                                if pass == 1 {
                                    if let Some(id) = element.id {
                                        draw.boxes.push((
                                            id,
                                            BoundingBox::new(
                                                element_cursor,
                                                vec2(
                                                    element.content_box.x,
                                                    if element.flex_y {
                                                        content_height
                                                    } else {
                                                        element.content_box.y
                                                    },
                                                ),
                                            ),
                                        ));
                                    }
                                }

                                if let Some(render) = element.render.as_ref() {
                                    render(draw, element_cursor, content_height);
                                }
                            }

                            if pass == 0 {
                                draw.set_tint_internal(vec4(1.0, 1.0, 1.0, 1.0));
                                draw.set_scale(1.0);
                            }

                            cursor_x += bounding_box.width();
                        }

                        cursor_y -= line_size.y;
                    }
                }
            }
        },
    );
}

pub(crate) fn border_extra(style: &Style, size: Vec2) -> Vec2 {
    let has_border = style.border_width > 0.0 && style.border_color.w > 0.0;
    let border_width = if has_border { style.border_width } else { 0.0 };

    match style.frame_style {
        FrameStyle::None => Vec2::ZERO,
        FrameStyle::Rectangle => vec2(border_width, border_width),
        FrameStyle::RoundedRectangle(_) => vec2(border_width, border_width),
        FrameStyle::Circle => {
            let radius = size.length() / 2.0;

            vec2(
                radius - size.x / 2.0 + border_width,
                radius - size.y / 2.0 + border_width,
            )
        }
        FrameStyle::Image(_) => vec2(border_width, border_width),
    }
}

pub(crate) fn draw_frame(draw: &DrawApi, style: &Style, cursor: Vec2, size: Vec2) {
    let has_border = style.border_width > 0.0 && style.border_color.w > 0.0;
    let border_width = if has_border { style.border_width } else { 0.0 };

    match style.frame_style {
        FrameStyle::None => {}
        FrameStyle::Rectangle => {
            draw.rectangle(
                cursor + vec2(border_width, -border_width),
                size - border_width,
                style.frame_color,
            );

            if has_border {
                draw.rectangle_border(
                    cursor,
                    size - border_width,
                    border_width,
                    style.border_color,
                );
            }
        }
        FrameStyle::RoundedRectangle(rounding) => {
            draw.rectangle_rounded(
                cursor + vec2(border_width, -border_width),
                size - 2.0 * border_width,
                rounding,
                style.frame_color,
            );

            if has_border {
                draw.rectangle_border_rounded(
                    cursor,
                    size,
                    border_width,
                    rounding,
                    5,
                    style.border_color,
                );
            }
        }
        FrameStyle::Circle => {
            let radius = size.x.min(size.y) / 2.0 - border_width;

            draw.circle(
                cursor + vec2(border_width, -border_width),
                radius / 2.0,
                radius,
                style.frame_color,
                32,
            );

            if has_border {
                draw.circle(
                    cursor,
                    radius + border_width / 2.0,
                    border_width,
                    style.border_color,
                    32,
                );
            }
        }
        FrameStyle::Image(image) => {
            draw.image(
                cursor + vec2(border_width, -border_width),
                size - border_width,
                image,
            );

            if has_border {
                draw.rectangle_border(
                    cursor,
                    size - border_width,
                    style.border_width,
                    style.border_color,
                );
            }
        }
    }
}
