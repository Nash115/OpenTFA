use crate::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub speed: f32,
    pub velocity: Vec3,
    pub spawn_point: Vec3,
    pub is_airborne: bool,
    pub on_wall: Option<f32>,
    pub is_sliding: bool,
    pub facing_dir: f32,
    pub is_grabbing_ledge: bool,
}
impl Player {
    pub fn new(spawn_point: Vec3, spawn_facing_dir: f32) -> Self {
        Self {
            speed: 70.0,
            velocity: Vec3::ZERO,
            spawn_point,
            is_airborne: false,
            on_wall: None,
            is_sliding: false,
            facing_dir: spawn_facing_dir,
            is_grabbing_ledge: false,
        }
    }

    pub fn is_walking(self) -> bool {
        self.velocity.x.abs() > 0.0 + f32::EPSILON
    }
    pub fn is_falling(self) -> bool {
        self.velocity.y < 0.0 - f32::EPSILON
    }
    pub fn is_jumping(self) -> bool {
        self.velocity.y > 0.0 + f32::EPSILON
    }

    pub fn respawn(&mut self, translation: &mut Vec3) {
        self.velocity = Vec3::ZERO;
        self.is_airborne = false;
        self.on_wall = None;
        self.is_sliding = false;
        self.facing_dir = -1.0;
        self.is_grabbing_ledge = false;
        *translation = self.spawn_point;
    }
}

#[derive(Component)]
pub struct PlayerSprites {
    pub walking_texture: Handle<Image>,
    pub walking_layout: Handle<TextureAtlasLayout>,
    pub jumping_texture: Handle<Image>,
    pub jumping_layout: Handle<TextureAtlasLayout>,
    pub falling_texture: Handle<Image>,
    pub falling_layout: Handle<TextureAtlasLayout>,
    pub sliding_texture: Handle<Image>,
    pub sliding_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
