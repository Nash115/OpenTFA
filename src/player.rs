use bevy::prelude::*;

use crate::GameState;

use crate::level::{Collider, SpawnPoint};
use crate::utils::{Aabb, Z_ENTITIES};

pub struct PlayerPlugin;

const JUMP_VELOCITY: f32 = 3.5;
const SLIDE_MAX_VELOCITY: f32 = -1.0;
const GRAVITY: f32 = -9.81;
const FORCE_MULTIPLIER: f32 = 50.0;
const PLAYER_SIZE:Vec2 = Vec2::new(10.0, 15.0);
const TILE_SIZE:Vec2 = Vec2::new(8.0, 8.0);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_player.run_if(in_state(GameState::InGame)))
            .add_systems(Update, (update_player, animate_player).chain().run_if(in_state(GameState::InGame)),)
            .add_systems(OnExit(GameState::InGame), despawn_players);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub speed: f32,
    pub velocity: Vec3,
    pub spawn_point: Vec3,
    pub is_airborne: bool,
    pub on_wall: Option<f32>,
    pub facing_dir: f32,
    pub is_grabbing_ledge: bool,
}
impl Player {
    fn new(spawn_point: Vec3) -> Self {
        Self {
            speed: 80.0,
            velocity: Vec3::ZERO,
            spawn_point,
            is_airborne: false,
            on_wall: None,
            facing_dir: -1.0,
            is_grabbing_ledge: false,
        }
    }

    fn is_walking(self) -> bool {
        self.velocity.x.abs() > 0.0 + f32::EPSILON
    }
    fn is_falling(self) -> bool {
        self.velocity.y < 0.0 - f32::EPSILON
    }
    fn is_jumping(self) -> bool {
        self.velocity.y > 0.0 + f32::EPSILON
    }

    fn respawn(&mut self, translation: &mut Vec3) {
        self.velocity = Vec3::ZERO;
        self.is_airborne = false;
        self.on_wall = None;
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
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<SpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (spawn_entity, spawn_transform) in query {

        let walking_texture = asset_server.load("sprites/blue_archer/walking.png");
        let walking_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None));
        let jumping_texture = asset_server.load("sprites/blue_archer/jumping.png");
        let jumping_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 1, None, None));
        let falling_texture = asset_server.load("sprites/blue_archer/falling.png");
        let falling_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 1, None, None));

        let player_spawn_point = Transform::from_xyz(spawn_transform.translation.x, spawn_transform.translation.y, Z_ENTITIES);

        commands.spawn((
            Sprite {
                image: walking_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: walking_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            player_spawn_point,
            Player::new(Vec3::new(
                spawn_transform.translation.x,
                spawn_transform.translation.y,
                Z_ENTITIES
            )),
            PlayerSprites {
                walking_texture,
                walking_layout,
                jumping_texture,
                jumping_layout,
                falling_texture,
                falling_layout
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
        commands.entity(spawn_entity).despawn();
    }
}

fn set_animation(
    sprite: &mut Sprite,
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    index: usize,
) -> bool {
    let current_layout = sprite.texture_atlas.as_ref().map(|atlas| atlas.layout.clone());
    let changed = sprite.image != *texture || current_layout != Some(layout.clone());

    if changed {
        sprite.image = texture.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: layout.clone(),
            index,
        });
    }

    changed
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite, &PlayerSprites, &Player)>,
) {
    for (mut timer, mut sprite, player_sprites, player) in &mut query {

        if player.is_jumping() {
            if set_animation(
                &mut sprite,
                &player_sprites.jumping_texture,
                &player_sprites.jumping_layout,
                0,
            ) {
                timer.0.reset();
            }
            continue;
        }
        else if player.is_falling() {
            if set_animation(
                &mut sprite,
                &player_sprites.falling_texture,
                &player_sprites.falling_layout,
                0,
            ) {
                timer.0.reset();
            }
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = (atlas.index + 1) % 2;
                }
            }
            continue;
        }

        if set_animation(
            &mut sprite,
            &player_sprites.walking_texture,
            &player_sprites.walking_layout,
            0,
        ) {
            timer.0.reset();
        }

        if player.is_walking() {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = (atlas.index + 1) % 4;
                }
            }
        }
        else {
            timer.0.reset();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 0;
            }
        }
    }
}

fn update_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {

    for (mut transform, mut player, mut sprite) in &mut player_query {
        
        // ### HORISONTAL PLAYER CONTROL ###

        let mut input_x = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            input_x -= 1.0;
            sprite.flip_x = true;
            player.facing_dir = -1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            input_x += 1.0;
            sprite.flip_x = false;
            player.facing_dir = 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::Menu);
        }
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            player.respawn(&mut transform.translation);
        }

        player.velocity.x = input_x * player.speed;

        // ### LEDGE GRABBING ###

        if player.is_grabbing_ledge {
            player.velocity = Vec3::ZERO;
            if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
                player.velocity.y = JUMP_VELOCITY * FORCE_MULTIPLIER * 0.8;
                player.velocity.x = player.facing_dir * player.speed * 1.2;
                player.is_grabbing_ledge = false;
            }
            if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) || (input_x != 0.0 && player.facing_dir != input_x) {
                player.is_grabbing_ledge = false;
            }
            transform.translation += player.velocity * time.delta_secs();
            continue;
        }

        // ### GRAVITY ###
        
        player.velocity.y += GRAVITY * FORCE_MULTIPLIER * time.delta_secs();

        // ### WALL INTERACTIONS # JUMP ###

        // --- Wall Sliding ---

        if let Some(wall_dir) = player.on_wall {
            if player.is_airborne && player.velocity.y < 0.0 && input_x == wall_dir {
                if player.velocity.y < (SLIDE_MAX_VELOCITY * FORCE_MULTIPLIER) {
                    player.velocity.y = SLIDE_MAX_VELOCITY * FORCE_MULTIPLIER;
                }
            }
        }

        // --- Jump and Wall Jumping ---

        if keyboard_input.just_pressed(KeyCode::Space) {
            if !player.is_airborne {
                player.velocity.y = JUMP_VELOCITY * FORCE_MULTIPLIER;
            }
            else if let Some(wall_dir) = player.on_wall {
                player.velocity.y = JUMP_VELOCITY * FORCE_MULTIPLIER * 0.9;
                player.velocity.x = -wall_dir * player.speed * 2.0;
                player.on_wall = None;
            }
        }

        // ### COLLISION RESOLUTION ###

        let delta = player.velocity * time.delta_secs();
        
        // --- MOVE HORIZONTALLY ---
        transform.translation.x += delta.x;
        let player_box = Aabb::new_sprite_box(transform.translation, PLAYER_SIZE);
        for collider_transform in &collider_query {
            let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
            if wall_box.intersects(&player_box) {
                if delta.x > 0.0 {
                    transform.translation.x = wall_box.left - (PLAYER_SIZE.x / 2.0);
                } else if delta.x < 0.0 {
                    transform.translation.x = wall_box.right + (PLAYER_SIZE.x / 2.0);
                }
                player.velocity.x = 0.0;
                break;
            }
        }

        // --- MOVE VERTICALLY ---
        transform.translation.y += delta.y;
        let player_box = Aabb::new_sprite_box(transform.translation, PLAYER_SIZE);
        for collider_transform in &collider_query {
            let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
            if wall_box.intersects(&player_box) {
                if delta.y > 0.0 {
                    transform.translation.y = wall_box.bottom - (PLAYER_SIZE.y / 2.0);
                } else if delta.y < 0.0 {
                    transform.translation.y = wall_box.top + (PLAYER_SIZE.y / 2.0);
                }
                player.velocity.y = 0.0;
                break;
            }
        }

        // ### UPDATE STATUS ###


        // --- Airborne ---

        player.is_airborne = player.velocity.y.abs() > 0.0 + f32::EPSILON;

        // --- Check wall nearby for potential wall interaction ---

        // - Look for a wall on the left and right side of the player -
        player.on_wall = None;
        let left_box = Aabb::new_sprite_box(
            Vec3::new(transform.translation.x-1.0, transform.translation.y, transform.translation.z),
            PLAYER_SIZE
        );
        let right_box = Aabb::new_sprite_box(
            Vec3::new(transform.translation.x+1.0, transform.translation.y, transform.translation.z),
            PLAYER_SIZE
        );
        for collider_transform in &collider_query {
            let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
            if wall_box.intersects(&left_box) {
                player.on_wall = Some(-1.0);
                break;
            }
            if wall_box.intersects(&right_box) {
                player.on_wall = Some(1.0);
                break;
            }
        }

        // --- Check for ledge grabbing ---

        let wall_dir = player.on_wall.unwrap_or(0.0);
        if player.is_airborne && wall_dir != 0.0 && wall_dir == input_x && wall_dir == player.facing_dir {
            let knees_box = Aabb::new_sprite_box(
                Vec3::new(transform.translation.x + player.facing_dir * (PLAYER_SIZE.x / 2.0), transform.translation.y - (PLAYER_SIZE.y / 3.0), transform.translation.z),
                PLAYER_SIZE * Vec2::new(0.5, 0.1)
            );
            let head_box = Aabb::new_sprite_box(
                Vec3::new(transform.translation.x + player.facing_dir * (PLAYER_SIZE.x / 2.0), transform.translation.y + (PLAYER_SIZE.y / 3.0), transform.translation.z),
                PLAYER_SIZE * Vec2::new(0.5, 0.1)
            );
            let bottom_box = Aabb::new_sprite_box(
                Vec3::new(transform.translation.x, transform.translation.y - (PLAYER_SIZE.y / 2.0), transform.translation.z),
                PLAYER_SIZE * Vec2::new(0.5, 1.0)
            );
            let mut bottom_not_touching_wall = true;
            let mut head_not_touching_wall = true;
            let mut knees_touching_wall = false;
            for collider_transform in &collider_query {
                let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
                if wall_box.intersects(&bottom_box) {
                    bottom_not_touching_wall = false;
                    break;
                }
                if wall_box.intersects(&head_box) {
                    head_not_touching_wall = false;
                    break;
                }
                if wall_box.intersects(&knees_box) {
                    knees_touching_wall = true;
                }
            }
            if bottom_not_touching_wall && head_not_touching_wall && knees_touching_wall {
                player.is_grabbing_ledge = true;
            }
        }

    }
}

fn despawn_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
