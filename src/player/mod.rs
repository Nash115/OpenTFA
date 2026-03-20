mod components;
pub mod controls;
mod systems;
mod utils;

use crate::prelude::*;

use self::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_players.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                (update_players, animate_players)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_players);
    }
}
