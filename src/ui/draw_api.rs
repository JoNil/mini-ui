use super::Align;
use crate::{
    cairo::Context,
    math::{vec2, Vec2, Vec4},
    ui::{bounding_box::BoundingBox, id::Id, Font, Image},
};
use std::f32::consts::FRAC_PI_2;

pub struct DrawApi<'a> {
    context: &'a Context,
    pub(crate) boxes: Vec<(Id, BoundingBox)>,
    pub pass: i32,
    pub scale: f32,
    tint: Vec4,
}

impl<'a> DrawApi<'a> {
    pub fn new(context: &'a Context) -> DrawApi {
        DrawApi {
            context,
            boxes: Vec::new(),
            pass: 1,
            scale: 1.0,
            tint: Vec4::ONE,
        }
    }

    #[inline]
    pub fn calc_text_size(&self, text: &str, text_height: f32, max_width: f32, font: Font) -> Vec2 {
        self.context.set_font_size(text_height as _);
        let extent = self.context.text_extents(text).unwrap();
        vec2(
            (extent.x_advance().max(extent.width()) as f32).min(max_width),
            extent.y_advance().max(extent.height()) as _,
        )
    }

    #[inline]
    pub fn line(&self, from: Vec2, to: Vec2, width: f32, color: Vec4) {
        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context.set_line_width(width as _);
        self.context.new_path();
        self.context.move_to(from.x as _, -from.y as _);
        self.context.line_to(to.x as _, -to.y as _);
        self.context.stroke().unwrap();
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn circle_segment(
        &self,
        pos: Vec2,
        radius: f32,
        from_angle_rad: f32,
        to_angle_rad: f32,
        width: f32,
        color: Vec4,
    ) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context.set_line_width(width as _);

        self.context.new_path();
        self.context.arc(
            (radius + width / 2.0 + pos.x) as _,
            (radius + width / 2.0 - pos.y) as _,
            (radius) as _,
            (-from_angle_rad - FRAC_PI_2) as _,
            (-to_angle_rad - FRAC_PI_2) as _,
        );

        self.context.stroke().unwrap();
    }

    #[inline]
    pub fn circle(&self, pos: Vec2, radius: f32, width: f32, color: Vec4) {
        self.circle_segment(pos, radius, 0.0, 2.0 * std::f32::consts::PI, width, color);
    }

    #[inline]
    pub fn rectangle(&self, pos: Vec2, size: Vec2, color: Vec4) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context
            .rectangle(pos.x as _, -pos.y as _, size.x as _, size.y as _);
        self.context.fill().unwrap();
    }

    #[inline]
    pub fn rectangle_rounded(&self, pos: Vec2, size: Vec2, rounding: f32, color: Vec4) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context
            .rectangle(pos.x as _, -pos.y as _, size.x as _, size.y as _);
        self.context.fill().unwrap();
    }

    #[inline]
    pub fn image(&self, pos: Vec2, size: Vec2, image: Image) {}

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn text(
        &self,
        text: &str,
        pos: Vec2,
        bounding_box: Vec2,
        text_height: f32,
        alignment: Align,
        color: Vec4,
        font: Font,
    ) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context.set_font_size(text_height as _);
        let extent = self.context.text_extents(text).unwrap();

        let size = vec2(
            (extent.x_advance().max(extent.width()) as f32).min(bounding_box.x),
            extent.y_advance().max(extent.height()) as _,
        );

        let offset = match alignment {
            Align::Right => bounding_box.x - size.x,
            Align::Left => 0.0,
            Align::Center => bounding_box.x / 2.0 - size.x / 2.0,
        };

        self.context.move_to(
            (offset + pos.x) as _,
            (-pos.y - extent.y_bearing() as f32) as _,
        );

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context.show_text(text).unwrap();
    }

    #[inline]
    pub fn rectangle_border(&self, pos: Vec2, size: Vec2, thickness: f32, color: Vec4) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context.new_path();
        self.context
            .rectangle(pos.x as _, -pos.y as _, size.x as _, size.y as _);
        self.context.set_line_width(thickness as _);
        self.context.stroke().unwrap();
    }

    #[inline]
    pub fn rectangle_border_rounded(
        &self,
        pos: Vec2,
        size: Vec2,
        thickness: f32,
        inner_corner_radius: f32,
        inner_corner_segments: i32,
        color: Vec4,
    ) {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());

        let color = self.tint * color;

        self.context
            .set_source_rgba(color.x as _, color.y as _, color.z as _, color.w as _);
        self.context.new_path();
        self.context
            .rectangle(pos.x as _, -pos.y as _, size.x as _, size.y as _);
        self.context.set_line_width(thickness as _);
        self.context.stroke().unwrap();
    }

    #[inline]
    pub fn set_tint(&mut self, tint: Vec4) {
        if self.pass == 0 {
            return;
        }

        self.set_tint_internal(tint)
    }

    #[inline]
    pub(crate) fn set_tint_internal(&mut self, tint: Vec4) {
        self.tint = tint;
    }

    #[inline]
    pub(crate) fn set_pass(&mut self, pass: i32) {
        self.pass = pass;
    }

    #[inline]
    pub(crate) fn pass(&self) -> i32 {
        self.pass
    }

    #[inline]
    pub(crate) fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}
