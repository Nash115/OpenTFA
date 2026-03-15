pub mod screen_wrap;

use crate::prelude::*;

use self::screen_wrap::systems::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            screen_wrap_system.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            manage_visual_clones.run_if(in_state(GameState::InGame)),
        );
    }
}
