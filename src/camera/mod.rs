mod components;
mod systems;

use crate::prelude::*;

use self::systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                fit_camera_to_level.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::Menu), reset_camera_for_menu);
    }
}
