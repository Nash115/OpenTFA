use bevy::prelude::*;

// --- Z-indexes for rendering order ---

pub const Z_TILES_BACK: f32 = 1.0;
pub const Z_ENTITIES: f32 = 5.0;
pub const Z_TILES_FG: f32 = 10.0;
#[allow(dead_code)]
pub const Z_UI: f32 = 100.0;

// --- Aabb implementation for collision detection ---

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
