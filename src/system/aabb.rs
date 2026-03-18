use crate::prelude::*;

pub struct Aabb {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Aabb {
    pub fn new_sprite_box(pos: Vec3, size: Vec2) -> Self {
        Self {
            left: pos.x - size.x / 2.0,
            right: pos.x + size.x / 2.0,
            bottom: pos.y - size.y / 2.0,
            top: pos.y + size.y / 2.0,
        }
    }
    pub fn new_tile_box(pos: Vec3, size: Vec2) -> Self {
        Self {
            left: pos.x,
            right: pos.x + size.x,
            bottom: pos.y,
            top: pos.y + size.y,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.left < other.right
            && self.right > other.left
            && self.bottom < other.top
            && self.top > other.bottom
    }
}
