use crate::{
    math::{Vec2, Vec4},
    ui::{bounding_box::BoundingBox, id::Id, Font, Image},
};

use super::Align;

pub struct DrawApi {
    pub(crate) boxes: Vec<(Id, BoundingBox)>,
    pub pass: i32,
    pub scale: f32,
    tint: Vec4,
}

impl DrawApi {
    pub fn new() -> DrawApi {
        DrawApi {
            boxes: Vec::new(),
            pass: 1,
            scale: 1.0,
            tint: Vec4::ONE,
        }
    }

    #[inline]
    pub fn calc_text_size(&self, text: &str, text_height: f32, max_width: f32, font: Font) -> Vec2 {
        todo!()
    }

    #[inline]
    pub fn line(&self, from: Vec2, to: Vec2, width: f32, color: Vec4) {}

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
        circle_segments: i32,
    ) {
    }

    #[inline]
    pub fn circle(&self, pos: Vec2, radius: f32, width: f32, color: Vec4, circle_segments: i32) {}

    #[inline]
    pub fn square(&self, pos: Vec2, size: Vec2, width: f32, color: Vec4) {}

    #[inline]
    pub fn rectangle(&self, pos: Vec2, size: Vec2, color: Vec4) {}

    #[inline]
    pub fn rectangle_rounded(&self, pos: Vec2, size: Vec2, rounding: f32, color: Vec4) {}

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
    }

    #[inline]
    pub fn lines(&self, points: &[Vec2], width: f32, color: Vec4) {}

    #[inline]
    pub fn rectangle_border(&self, pos: Vec2, size: Vec2, thickness: f32, color: Vec4) {}

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