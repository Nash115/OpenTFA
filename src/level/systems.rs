use crate::prelude::*;

use super::components::*;

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/cave.ldtk").into(),
            ..Default::default()
        },
        ActiveLevel,
    ));
}

pub fn setup_world_limits(
    mut commands: Commands,
    world_query: Query<&LdtkProjectHandle>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    let level_size = match resolve_level_size(&world_query, &ldtk_projects, &level_selection) {
        Some(size) => size,
        None => return,
    };
    commands.spawn((
        WorldLimits {
            width: level_size.x,
            height: level_size.y,
            left: 0.0,
            right: level_size.x,
            bottom: 0.0,
            top: level_size.y,
        },
        ActiveLevel,
    ));
}

pub fn extract_facing_dir(entity_instance: &EntityInstance) -> SpawnFacingDir {
    let facing_dir_value = entity_instance
        .get_float_field("facing_dir")
        .copied()
        .unwrap_or(0.0);
    if facing_dir_value != 1.0 && facing_dir_value != -1.0 {
        return SpawnFacingDir(1.0);
    }
    SpawnFacingDir(facing_dir_value)
}

pub fn despawn_level(mut commands: Commands, query: Query<Entity, With<ActiveLevel>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn sort_ldtk_layers(mut query: Query<(&mut Transform, &LayerMetadata), Added<LayerMetadata>>) {
    for (mut transform, layer_meta) in query.iter_mut() {
        match layer_meta.identifier.as_str() {
            "VisualBackground" => transform.translation.z = Z_TILES_BACK,
            "VisualForeground" => transform.translation.z = Z_TILES_FG,
            "Collisions" => {}
            "Entities" => {}
            _ => {
                eprintln!("Warning: Unrecognized layer '{}'", layer_meta.identifier);
            }
        }
    }
}

pub fn resolve_level_size(
    world_query: &Query<&LdtkProjectHandle>,
    ldtk_projects: &Res<Assets<LdtkProject>>,
    level_selection: &Res<LevelSelection>,
) -> Option<Vec2> {
    for project_handle in world_query.iter() {
        let project = ldtk_projects.get(project_handle.id())?;
        let level = project.find_raw_level_by_level_selection(level_selection.as_ref())?;

        if level.px_wid > 0 && level.px_hei > 0 {
            return Some(Vec2::new(level.px_wid as f32, level.px_hei as f32));
        }
    }

    None
}
