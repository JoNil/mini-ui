use crate::{
    math::{vec2, vec4, Vec2},
    ui::{draw_api::DrawApi, ui::Element},
};

pub fn draw_content_boxes(draw: &DrawApi, cursor: Vec2, element: &Element, content_height: f32) {
    let content_box = vec2(
        element.content_box.get().x,
        if element.flex_y {
            content_height
        } else {
            element.content_box.get().y
        },
    ) - vec2(2.0 * element.border_extra.x, 2.0 * element.border_extra.y);
    let margin = element.style.margin;
    let padding =
        element.style.padding + vec2(2.0 * element.border_extra.x, 2.0 * element.border_extra.y);

    draw.rectangle(
        cursor,
        content_box + padding.size() + margin.size(),
        vec4(0.0, 1.0, 0.0, 0.2),
    );
    draw.rectangle(
        cursor + vec2(margin.left, -margin.top),
        content_box + padding.size(),
        vec4(0.0, 0.0, 1.0, 0.2),
    );
    draw.rectangle(
        cursor + vec2(padding.left, -padding.top) + vec2(margin.left, -margin.top),
        content_box,
        vec4(1.0, 0.0, 0.0, 0.2),
    );
}
