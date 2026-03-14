use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::utils::{Z_TILES_BACK, Z_TILES_FG};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                ..default()
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<SpawnPointBundle>("PlayerSpawn")
            .add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(
                Update,
                setup_world_limits.run_if(
                    in_state(GameState::InGame).and(not(any_with_component::<WorldLimits>)),
                ),
            )
            .add_systems(Update, sort_ldtk_layers.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_level);
    }
}

#[derive(Component, Default)]
pub struct Collider;
#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    collider: Collider,
}

#[derive(Component, Default)]
pub struct SpawnPoint;
#[derive(Default, Component)]
pub struct SpawnFacingDir(pub f32);
#[derive(Bundle, LdtkEntity, Default)]
pub struct SpawnPointBundle {
    #[with(extract_facing_dir)]
    pub spawn_facing_dir: SpawnFacingDir,
    spawn_point: SpawnPoint,
}

#[derive(Component)]
struct ActiveLevel;

#[derive(Component)]
pub struct WorldLimits {
    pub width: f32,
    pub height: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/cave.ldtk").into(),
            ..Default::default()
        },
        ActiveLevel,
    ));
}

fn setup_world_limits(
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

fn extract_facing_dir(entity_instance: &EntityInstance) -> SpawnFacingDir {
    let facing_dir_value = entity_instance
        .get_float_field("facing_dir")
        .copied()
        .unwrap_or(0.0);
    if facing_dir_value != 1.0 && facing_dir_value != -1.0 {
        return SpawnFacingDir(1.0);
    }
    SpawnFacingDir(facing_dir_value)
}

fn despawn_level(mut commands: Commands, query: Query<Entity, With<ActiveLevel>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn sort_ldtk_layers(mut query: Query<(&mut Transform, &LayerMetadata), Added<LayerMetadata>>) {
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

fn resolve_level_size(
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
