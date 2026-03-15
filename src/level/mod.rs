mod bundles;
mod components;
mod systems;

use crate::prelude::*;

use self::bundles::*;
pub use self::components::*;
use self::systems::*;

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
