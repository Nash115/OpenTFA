use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::GameState;

use crate::utils::{Z_TILES_BACK, Z_TILES_FG};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                ..default()
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<SpawnPointBundle>("PlayerSpawn")
            .add_systems(OnEnter(GameState::InGame), setup_level)
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
#[derive(Bundle, LdtkEntity, Default)]
pub struct SpawnPointBundle {
    spawn_point: SpawnPoint,
}

#[derive(Component)]
struct ActiveLevel;

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/cave.ldtk").into(),
            ..Default::default()
        },
        ActiveLevel,
    ));
}

fn despawn_level(mut commands: Commands, query: Query<Entity, With<ActiveLevel>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn sort_ldtk_layers(
    mut query: Query<(&mut Transform, &LayerMetadata), Added<LayerMetadata>>,
) {
    for (mut transform, layer_meta) in query.iter_mut() {
        match layer_meta.identifier.as_str() {
            "VisualBackground" => transform.translation.z = Z_TILES_BACK,
            "VisualForeground" => transform.translation.z = Z_TILES_FG,
            "Collisions" => {},
            "Entities" => {},
            _ => {
                println!("Warning: Unrecognized layer '{}'", layer_meta.identifier);
            }
        }
    }
}
