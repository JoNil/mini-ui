use crate::{
    math::{vec4, Vec2, Vec4},
    ui::{Align, Font, FrameStyle, Spacing, VertAlign},
};

pub const LIGHT_GRAY: Vec4 = vec4(0.8, 0.8, 0.8, 1.0);
pub const DARK_GRAY: Vec4 = vec4(0.2, 0.2, 0.2, 0.5);
pub const MEDIUM_GRAY: Vec4 = vec4(0.4, 0.4, 0.4, 1.0);

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub frame_style: FrameStyle,
    pub frame_color: Vec4,
    pub border_width: f32,
    pub border_color: Vec4,
    pub text_color: Vec4,
    pub inactive_color: Vec4,
    pub margin: Spacing,
    pub padding: Spacing,
    pub text_height: f32,
    pub align: Option<Align>,
    pub vert_align: VertAlign,
    pub font: Option<Font>,
    pub icon_font: Option<Font>,
    pub shadow_dir: Option<Vec2>,
    pub shadow_scale: f32,
    pub shadow_color: Vec4,
    pub debug: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            frame_style: FrameStyle::Rectangle,
            frame_color: DARK_GRAY,
            border_width: 0.0,
            border_color: MEDIUM_GRAY,
            text_color: LIGHT_GRAY,
            inactive_color: MEDIUM_GRAY,
            margin: Spacing::symmetrical(0.0),
            padding: Spacing::symmetrical(0.0),
            text_height: 20.0,
            align: None,
            vert_align: VertAlign::Top,
            font: None,
            icon_font: None,
            shadow_dir: None,
            shadow_scale: 1.0,
            shadow_color: vec4(0.1, 0.1, 0.1, 0.9),
            debug: false,
        }
    }
}

impl Style {
    #[must_use]
    #[inline]
    pub fn frame_style(&self, frame_style: FrameStyle) -> Style {
        let mut res = *self;
        res.frame_style = frame_style;
        res
    }

    #[must_use]
    #[inline]
    pub fn frame_color(&self, frame_color: Vec4) -> Style {
        let mut res = *self;
        res.frame_color = frame_color;
        res
    }

    #[must_use]
    #[inline]
    pub fn border_width(&self, border_width: f32) -> Style {
        let mut res = *self;
        res.border_width = border_width;
        res
    }

    #[must_use]
    #[inline]
    pub fn border_color(&self, border_color: Vec4) -> Style {
        let mut res = *self;
        res.border_color = border_color;
        res
    }

    #[must_use]
    #[inline]
    pub fn text_color(&self, text_color: Vec4) -> Style {
        let mut res = *self;
        res.text_color = text_color;
        res
    }

    #[must_use]
    #[inline]
    pub fn inactive_color(&self, inactive_color: Vec4) -> Style {
        let mut res = *self;
        res.inactive_color = inactive_color;
        res
    }

    #[must_use]
    #[inline]
    pub fn margin(&self, margin: Spacing) -> Style {
        let mut res = *self;
        res.margin = margin;
        res
    }

    #[must_use]
    #[inline]
    pub fn padding(&self, padding: Spacing) -> Style {
        let mut res = *self;
        res.padding = padding;
        res
    }

    #[must_use]
    #[inline]
    pub fn spacing(&self, spacing: Spacing) -> Style {
        let mut res = *self;
        res.margin = spacing;
        res.padding = spacing;
        res
    }

    #[must_use]
    #[inline]
    pub fn text_height(&self, text_height: f32) -> Style {
        let mut res = *self;
        res.text_height = text_height;
        res
    }

    #[must_use]
    #[inline]
    pub fn align(&self, align: Align) -> Style {
        let mut res = *self;
        res.align = Some(align);
        res
    }

    #[must_use]
    #[inline]
    pub fn vert_align(&self, vert_align: VertAlign) -> Style {
        let mut res = *self;
        res.vert_align = vert_align;
        res
    }

    #[must_use]
    #[inline]
    pub fn font(&self, font: Font) -> Style {
        let mut res = *self;
        res.font = Some(font);
        res
    }

    #[must_use]
    #[inline]
    pub fn icon_font(&self, icon_font: Font) -> Style {
        let mut res = *self;
        res.icon_font = Some(icon_font);
        res
    }

    #[must_use]
    #[inline]
    pub fn no_shadow(&self) -> Style {
        let mut res = *self;
        res.shadow_dir = None;
        res
    }

    #[must_use]
    #[inline]
    pub fn shadow_dir(&self, shadow_dir: Vec2) -> Style {
        let mut res = *self;
        res.shadow_dir = Some(shadow_dir);
        res
    }

    #[must_use]
    #[inline]
    pub fn shadow_scale(&self, shadow_scale: f32) -> Style {
        let mut res = *self;
        res.shadow_scale = shadow_scale;
        res
    }

    #[must_use]
    #[inline]
    pub fn shadow_color(&self, shadow_color: Vec4) -> Style {
        let mut res = *self;
        res.shadow_color = shadow_color;
        res
    }

    #[must_use]
    #[inline]
    pub fn debug(&self, debug: bool) -> Style {
        let mut res = *self;
        res.debug = debug;
        res
    }
}
