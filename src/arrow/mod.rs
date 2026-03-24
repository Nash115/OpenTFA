pub mod components;
mod systems;

use crate::prelude::*;

use self::systems::*;

pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_arrows, collect_arrows).run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), despawn_arrows);
    }
}
