use crate::prelude::*;

use crate::arrow::components::{Arrow, ArrowType};
use crate::level::{Collider, SpawnFacingDir, SpawnPoint};
use crate::physics::screen_wrap::components::Wrapable;
use crate::system::aabb::Aabb;
use crate::system::resources::GameRegistry;
use crate::ui::{
    controls::UIControls,
    menu::resources::{MatchConfig, PlayerDevice},
};

use super::components::*;
use super::controls::PlayerControls;
use super::utils::*;

pub fn spawn_players(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &SpawnFacingDir), Added<SpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game_registry: Res<GameRegistry>,
    match_config: Res<MatchConfig>,
) {
    let Ok((spawn_entity, spawn_transform, spawn_facing_dir)) = query.single() else {
        return;
    };
    for (player_i, _) in match_config.players.iter().flatten().enumerate() {
        let Some(ref player) = match_config.players[player_i] else {
            continue;
        };
        let char_id = player.char_register_id;
        let char_path = game_registry.characters[char_id].sprite_path.clone();
        let walking_texture = asset_server.load(format!("{}/walking.png", char_path));
        let walking_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            4,
            1,
            None,
            None,
        ));
        let jumping_texture = asset_server.load(format!("{}/jumping.png", char_path));
        let jumping_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            1,
            1,
            None,
            None,
        ));
        let falling_texture = asset_server.load(format!("{}/falling.png", char_path));
        let falling_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            2,
            1,
            None,
            None,
        ));
        let sliding_texture = asset_server.load(format!("{}/sliding.png", char_path));
        let sliding_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            2,
            1,
            None,
            None,
        ));

        let player_spawn_point = Transform::from_xyz(
            spawn_transform.translation.x,
            spawn_transform.translation.y,
            Z_ENTITIES,
        );

        let mut input_map = InputMap::default();
        match player.device {
            PlayerDevice::Keyboard => {
                input_map
                    .insert(PlayerControls::Left, KeyCode::ArrowLeft)
                    .insert(PlayerControls::Left, KeyCode::KeyA)
                    .insert(PlayerControls::Right, KeyCode::ArrowRight)
                    .insert(PlayerControls::Right, KeyCode::KeyD)
                    .insert(PlayerControls::Up, KeyCode::ArrowUp)
                    .insert(PlayerControls::Up, KeyCode::KeyW)
                    .insert(PlayerControls::Down, KeyCode::ArrowDown)
                    .insert(PlayerControls::Down, KeyCode::KeyS)
                    .insert(PlayerControls::Jump, KeyCode::Space)
                    .insert(PlayerControls::Shoot, KeyCode::Enter)
                    .insert(PlayerControls::Respawn, KeyCode::KeyR);
            }
            PlayerDevice::Gamepad(gamepad_entity) => {
                input_map
                    .insert(PlayerControls::Left, GamepadButton::DPadLeft)
                    .insert(
                        PlayerControls::Left,
                        GamepadControlDirection::LEFT_LEFT.threshold(0.25),
                    )
                    .insert(PlayerControls::Right, GamepadButton::DPadRight)
                    .insert(
                        PlayerControls::Right,
                        GamepadControlDirection::LEFT_RIGHT.threshold(0.25),
                    )
                    .insert(PlayerControls::Up, GamepadButton::DPadUp)
                    .insert(
                        PlayerControls::Up,
                        GamepadControlDirection::LEFT_UP.threshold(0.25),
                    )
                    .insert(PlayerControls::Down, GamepadButton::DPadDown)
                    .insert(
                        PlayerControls::Down,
                        GamepadControlDirection::LEFT_DOWN.threshold(0.25),
                    )
                    .insert(PlayerControls::Jump, GamepadButton::South)
                    .insert(PlayerControls::Shoot, GamepadButton::West)
                    .insert(PlayerControls::Respawn, GamepadButton::Select)
                    .set_gamepad(gamepad_entity);
            }
        };

        commands.spawn((
            ActionState::<PlayerControls>::default(),
            input_map,
            Sprite {
                image: walking_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: walking_layout.clone(),
                    index: 0,
                }),
                flip_x: spawn_facing_dir.0 == -1.0,
                ..default()
            },
            player_spawn_point,
            Player::new(
                Vec3::new(
                    spawn_transform.translation.x,
                    spawn_transform.translation.y,
                    Z_ENTITIES,
                ),
                spawn_facing_dir.0,
            ),
            PlayerSprites {
                walking_texture,
                walking_layout,
                jumping_texture,
                jumping_layout,
                falling_texture,
                falling_layout,
                sliding_texture,
                sliding_layout,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Inventory::default(),
            Wrapable,
        ));
    }
    commands.entity(spawn_entity).despawn();
}

pub fn update_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    action_state_ui: Res<ActionState<UIControls>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Transform,
        &mut Player,
        &mut Sprite,
        &ActionState<PlayerControls>,
        &mut Inventory,
    )>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (mut transform, mut player, mut sprite, action_state_player, mut inventory) in
        &mut player_query
    {
        // ### HORISONTAL / VERTICAL PLAYER CONTROL ###

        let mut input_x = 0.0;
        let mut input_y = 0.0;

        if action_state_player.pressed(&PlayerControls::Left) {
            input_x -= 1.0;
            sprite.flip_x = true;
            player.facing_dir = -1.0;
        }
        if action_state_player.pressed(&PlayerControls::Right) {
            input_x += 1.0;
            sprite.flip_x = false;
            player.facing_dir = 1.0;
        }
        if action_state_player.pressed(&PlayerControls::Up) {
            input_y += 1.0;
        }
        if action_state_player.pressed(&PlayerControls::Down) {
            input_y -= 1.0;
        }

        // ### SHOOT ARROW ###

        if action_state_player.just_released(&PlayerControls::Shoot) && !inventory.arrows.is_empty()
        {
            let arrow_to_shoot = inventory.arrows.pop().unwrap();
            let dir_y = input_y;
            let dir_x = match input_x {
                0.0 => match dir_y {
                    0.0 => player.facing_dir,
                    _ => 0.0,
                },
                _ => input_x,
            };
            let direction = Vec3::new(dir_x, dir_y, 0.0).normalize();
            let arrow_sprite_img = match arrow_to_shoot {
                ArrowType::Normal => asset_server.load("sprites/arrow/arrow.png"),
            };
            commands.spawn((
                Sprite {
                    image: arrow_sprite_img,
                    ..default()
                },
                Transform::from_translation(transform.translation + direction * 0.5),
                Arrow::new(arrow_to_shoot, direction),
                Wrapable,
            ));
        }
        if action_state_player.pressed(&PlayerControls::Shoot) {
            input_x = 0.0;
        }

        // ### MISC CONTROLS ###

        if action_state_ui.just_pressed(&UIControls::Menu) {
            next_state.set(GameState::Menu);
        }
        if action_state_player.just_pressed(&PlayerControls::Respawn) {
            player.respawn(&mut transform.translation);
        }

        player.velocity.x = input_x * player.speed;

        // ### LEDGE GRABBING ###

        if player.is_grabbing_ledge {
            player.velocity = Vec3::ZERO;
            if action_state_player.just_pressed(&PlayerControls::Jump)
                || action_state_player.just_pressed(&PlayerControls::Up)
            {
                player.is_grabbing_ledge = false;
                player.velocity.y = JUMP_VELOCITY * FORCE_MULTIPLIER;
                break;
            }
            let wall_dir = match player.on_wall {
                Some(dir) => dir,
                None => {
                    player.is_grabbing_ledge = false;
                    break;
                }
            };
            if action_state_player.just_pressed(&PlayerControls::Down)
                || (wall_dir == -1.0 && !action_state_player.pressed(&PlayerControls::Left))
                || (wall_dir == 1.0 && !action_state_player.pressed(&PlayerControls::Right))
            {
                player.is_grabbing_ledge = false;
            }
            continue;
        }

        // ### GRAVITY ###

        player.velocity.y += GRAVITY * FORCE_MULTIPLIER * time.delta_secs();

        // ### WALL INTERACTIONS # JUMP ###

        // --- Wall Sliding ---

        if player.is_sliding {
            if player.velocity.y < (SLIDE_MAX_VELOCITY * FORCE_MULTIPLIER) {
                player.velocity.y = SLIDE_MAX_VELOCITY * FORCE_MULTIPLIER;
            }
        }

        // --- Jump and Wall Jumping ---

        if action_state_player.just_pressed(&PlayerControls::Jump) {
            if !player.is_airborne {
                player.velocity.y = JUMP_VELOCITY * FORCE_MULTIPLIER;
            } else if let Some(wall_dir) = player.on_wall {
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
            Vec3::new(
                transform.translation.x - 1.0,
                transform.translation.y,
                transform.translation.z,
            ),
            PLAYER_SIZE,
        );
        let right_box = Aabb::new_sprite_box(
            Vec3::new(
                transform.translation.x + 1.0,
                transform.translation.y,
                transform.translation.z,
            ),
            PLAYER_SIZE,
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

        // --- Check for sliding ---
        if let Some(wall_dir) = player.on_wall {
            if player.is_airborne && player.velocity.y < 0.0 && input_x == wall_dir {
                player.is_sliding = true;
            } else {
                player.is_sliding = false;
            }
        } else {
            player.is_sliding = false;
        }

        // --- Check for ledge grabbing ---

        let wall_dir = player.on_wall.unwrap_or(0.0);
        if player.is_airborne
            && player.velocity.y < 0.0
            && wall_dir != 0.0
            && wall_dir == input_x
            && wall_dir == player.facing_dir
        {
            let knees_box = Aabb::new_sprite_box(
                Vec3::new(
                    transform.translation.x + player.facing_dir * (PLAYER_SIZE.x / 2.0),
                    transform.translation.y - (PLAYER_SIZE.y / 4.0),
                    transform.translation.z,
                ),
                PLAYER_SIZE * Vec2::new(0.5, 0.1),
            );
            let head_box = Aabb::new_sprite_box(
                Vec3::new(
                    transform.translation.x + player.facing_dir * (PLAYER_SIZE.x / 2.0),
                    transform.translation.y + (PLAYER_SIZE.y / 3.0),
                    transform.translation.z,
                ),
                PLAYER_SIZE * Vec2::new(0.5, 0.1),
            );
            let bottom_box = Aabb::new_sprite_box(
                Vec3::new(
                    transform.translation.x,
                    transform.translation.y - (PLAYER_SIZE.y / 2.0),
                    transform.translation.z,
                ),
                PLAYER_SIZE * Vec2::new(0.5, 1.0),
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

pub fn animate_players(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite, &PlayerSprites, &Player)>,
) {
    for (mut timer, mut sprite, player_sprites, player) in &mut query {
        if player.is_grabbing_ledge || player.is_sliding {
            if set_animation(
                &mut sprite,
                &player_sprites.sliding_texture,
                &player_sprites.sliding_layout,
                0,
            ) {
                timer.0.reset();
            }
            continue;
        } else if player.is_jumping() {
            if set_animation(
                &mut sprite,
                &player_sprites.jumping_texture,
                &player_sprites.jumping_layout,
                0,
            ) {
                timer.0.reset();
            }
            continue;
        } else if player.is_falling() {
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
        } else {
            timer.0.reset();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 0;
            }
        }
    }
}

pub fn update_floating_inventory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Inventory), Changed<Inventory>>,
    children_query: Query<&Children>,
    indicator_query: Query<Entity, With<ArrowIndicator>>,
) {
    let spacing = 4.0;
    let height_offset = 13.0;

    for (player_entity, inventory) in &query {
        if let Ok(children) = children_query.get(player_entity) {
            for child in children.iter() {
                if indicator_query.contains(child) {
                    commands.entity(child).despawn();
                }
            }
        }

        if inventory.arrows.len() == 0 {
            continue;
        }

        let start_x = -((inventory.arrows.len() as f32 - 1.0) * spacing) / 2.0;

        commands.entity(player_entity).with_children(|parent| {
            for (i, arrow) in inventory.arrows.iter().enumerate() {
                let x_pos = start_x + (i as f32 * spacing);

                let icon_texture = match arrow {
                    ArrowType::Normal => asset_server.load("sprites/arrow/normal_indicator.png"),
                };

                parent.spawn((
                    Sprite {
                        image: icon_texture.clone(),
                        ..default()
                    },
                    Transform::from_xyz(x_pos, height_offset, Z_UI),
                    ArrowIndicator,
                ));
            }
        });
    }
}

pub fn despawn_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
