use crate::prelude::*;

// --- Z-indexes for rendering order ---

pub const Z_TILES_BACK: f32 = 1.0;
pub const Z_ENTITIES: f32 = 5.0;
pub const Z_TILES_FG: f32 = 10.0;

// --- Player Constants ---

pub const JUMP_VELOCITY: f32 = 3.5;
pub const SLIDE_MAX_VELOCITY: f32 = -1.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(10.0, 15.0);

// --- Physics Constants ---

pub const GRAVITY: f32 = -9.81;
pub const FORCE_MULTIPLIER: f32 = 50.0;

// --- Tile Constants ---

pub const TILE_SIZE: Vec2 = Vec2::new(8.0, 8.0);
