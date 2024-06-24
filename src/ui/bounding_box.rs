use crate::math::{vec2, Vec2};

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    pub top_left: Vec2,
    pub size: Vec2,
}

impl BoundingBox {
    #[inline]
    pub fn new(top_left: Vec2, size: Vec2) -> BoundingBox {
        BoundingBox { top_left, size }
    }

    #[inline]
    pub fn intersect(&self, p: Vec2) -> bool {
        let bottom_right = self.top_left + vec2(self.size.x, -self.size.y);
        p.x >= self.top_left.x
            && p.x <= bottom_right.x
            && p.y <= self.top_left.y
            && p.y >= bottom_right.y
    }
}
