use crate::prelude::*;

use crate::level::Collider;
use crate::player::components::{Inventory, Player};
use crate::system::aabb::Aabb;

use super::components::{Arrow, ArrowState};

pub fn update_arrows(
    time: Res<Time>,
    mut arrow_query: Query<(&mut Transform, &mut Arrow)>,
    collider_query: Query<&Transform, (With<Collider>, Without<Arrow>)>,
) {
    for (mut transform, mut arrow) in &mut arrow_query {
        if arrow.state == ArrowState::Stuck {
            continue;
        }

        if arrow.velocity.length_squared() > 0.0 {
            let angle = arrow.velocity.y.atan2(arrow.velocity.x);
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_4);
        }

        // ### GRAVITY ###

        arrow.float_timer.tick(time.delta());
        if arrow.float_timer.is_finished() {
            arrow.velocity.y += GRAVITY * FORCE_MULTIPLIER * 0.5 * time.delta_secs();
        }

        // ### COLLISION RESOLUTION ###

        // --- MOVE HORIZONTALLY ---
        let delta = arrow.velocity * time.delta_secs();

        transform.translation.x += delta.x;
        let arrow_box = Aabb::new_sprite_box(
            transform.translation + (arrow.velocity.normalize_or_zero() * ARROW_TIP_DISTANCE),
            ARROW_TIP_SIZE,
        );
        for collider_transform in &collider_query {
            let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
            if wall_box.intersects(&arrow_box) {
                if delta.x > 0.0 {
                    transform.translation.x = wall_box.left - (ARROW_ENTIRE_SIZE.x / 2.0);
                } else if delta.x < 0.0 {
                    transform.translation.x = wall_box.right + (ARROW_ENTIRE_SIZE.x / 2.0);
                }
                arrow.velocity.x = 0.0;
                arrow.state = ArrowState::Stuck;
                break;
            }
        }

        // --- MOVE VERTICALLY ---
        transform.translation.y += delta.y;
        let arrow_box = Aabb::new_sprite_box(
            transform.translation + (arrow.velocity.normalize_or_zero() * ARROW_TIP_DISTANCE),
            ARROW_TIP_SIZE,
        );
        for collider_transform in &collider_query {
            let wall_box = Aabb::new_tile_box(collider_transform.translation, TILE_SIZE);
            if wall_box.intersects(&arrow_box) {
                if delta.y > 0.0 {
                    transform.translation.y = wall_box.bottom - (ARROW_ENTIRE_SIZE.y / 2.0);
                } else if delta.y < 0.0 {
                    transform.translation.y = wall_box.top + (ARROW_ENTIRE_SIZE.y / 2.0);
                }
                arrow.velocity.y = 0.0;
                arrow.state = ArrowState::Stuck;
                break;
            }
        }
    }
}

pub fn collect_arrows(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Inventory), With<Player>>,
    arrow_query: Query<(Entity, &Transform, &Arrow)>,
) {
    for (entity, arrow_transform, arrow) in &arrow_query {
        if arrow.state == ArrowState::Stuck {
            for (player_transform, mut inventory) in &mut player_query {
                let arrow_box =
                    Aabb::new_sprite_box(arrow_transform.translation, ARROW_ENTIRE_SIZE);
                let player_box = Aabb::new_sprite_box(player_transform.translation, PLAYER_SIZE);
                if arrow_box.intersects(&player_box) {
                    inventory.arrows.push(arrow.arrow_type);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn despawn_arrows(mut commands: Commands, query: Query<Entity, With<Arrow>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
