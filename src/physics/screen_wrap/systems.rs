use crate::prelude::*;

use crate::level::WorldLimits;

use super::components::*;

pub fn screen_wrap_system(
    mut query: Query<&mut Transform, With<Wrapable>>,
    world_limits: Query<&WorldLimits>,
) {
    let world_limits = match world_limits.single() {
        Ok(limits) => limits,
        Err(_) => return,
    };

    for mut transform in &mut query {
        if transform.translation.x > world_limits.right {
            transform.translation.x = world_limits.left;
        } else if transform.translation.x < world_limits.left {
            transform.translation.x = world_limits.right;
        }
        if transform.translation.y > world_limits.top {
            transform.translation.y = world_limits.bottom;
        } else if transform.translation.y < world_limits.bottom {
            transform.translation.y = world_limits.top;
        }
    }
}

pub fn manage_visual_clones(
    mut commands: Commands,
    world_limits_query: Query<&WorldLimits>,
    parent_query: Query<(Entity, &Transform, &Sprite), With<Wrapable>>,
    mut clone_query: Query<(Entity, &mut Transform, &mut Sprite, &VisualClone), Without<Wrapable>>,
) {
    let world_limits = match world_limits_query.single() {
        Ok(limits) => limits,
        Err(_) => return,
    };
    let threshold_x = 8.0;
    let threshold_y = 8.0;

    for (clone_entity, mut clone_transform, mut clone_sprite, clone_meta) in &mut clone_query {
        if let Ok((_parent_entity, parent_transform, parent_sprite)) =
            parent_query.get(clone_meta.parent_entity)
        {
            let is_parent_near_x = parent_transform.translation.x
                > world_limits.right - threshold_x
                || parent_transform.translation.x < world_limits.left + threshold_x;
            let is_parent_near_y = parent_transform.translation.y > world_limits.top - threshold_y
                || parent_transform.translation.y < world_limits.bottom + threshold_y;

            if is_parent_near_x || is_parent_near_y {
                clone_transform.translation = parent_transform.translation + clone_meta.offset;

                clone_sprite.flip_x = parent_sprite.flip_x;
                if let (Some(clone_atlas), Some(parent_atlas)) = (
                    &mut clone_sprite.texture_atlas,
                    &parent_sprite.texture_atlas,
                ) {
                    clone_atlas.index = parent_atlas.index;
                }
            } else {
                commands.entity(clone_entity).despawn();
            }
        } else {
            commands.entity(clone_entity).despawn();
        }
    }

    for (parent_entity, parent_transform, parent_sprite) in &parent_query {
        let pos = parent_transform.translation;

        let mut offset = Vec3::ZERO;

        if pos.x > world_limits.right - threshold_x {
            offset.x = -world_limits.width;
        } else if pos.x < world_limits.left + threshold_x {
            offset.x = world_limits.width;
        }

        if pos.y > world_limits.top - threshold_y {
            offset.y = -world_limits.height;
        } else if pos.y < world_limits.bottom + threshold_y {
            offset.y = world_limits.height;
        }

        if offset != Vec3::ZERO {
            let clone_exists = clone_query.iter().any(|(_, _, _, meta)| {
                meta.parent_entity == parent_entity && meta.offset == offset
            });

            if !clone_exists {
                commands.spawn((
                    parent_sprite.clone(),
                    Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, pos.z),
                    VisualClone {
                        parent_entity,
                        offset,
                    },
                ));
            }
        }
    }
}
