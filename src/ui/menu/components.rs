use crate::prelude::*;

#[derive(Component)]
pub struct MenuEntity;

#[derive(Component)]
pub struct SelectEntity;

#[derive(Component)]
pub struct PlayerIndex(pub usize);

#[derive(Component)]
pub struct ReadyCooldown(pub Timer);

#[derive(Component)]
pub struct SelectContainer;
