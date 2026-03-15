use crate::prelude::*;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component, Default)]
pub struct SpawnPoint;

#[derive(Component, Default)]
pub struct SpawnFacingDir(pub f32);

#[derive(Component)]
pub struct ActiveLevel;

#[derive(Component)]
pub struct WorldLimits {
    pub width: f32,
    pub height: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}
