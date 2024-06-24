use crate::math::Vec2;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Id {
    hash: u64,
}

impl Id {
    pub fn none() -> Id {
        Id { hash: 0 }
    }

    pub fn from_vec2(v: Vec2) -> Id {
        let mut hasher = DefaultHasher::new();
        (v.x as i32).hash(&mut hasher);
        (v.y as i32).hash(&mut hasher);
        let hash = hasher.finish();
        Id { hash }
    }

    pub fn with_child(&self, id: Id) -> Id {
        let mut hasher = DefaultHasher::new();
        self.hash.hash(&mut hasher);
        id.hash.hash(&mut hasher);
        let hash = hasher.finish();
        Id { hash }
    }
}
