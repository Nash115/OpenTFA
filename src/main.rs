mod camera;
mod level;
mod physics;
mod player;
mod prelude;
mod system;
mod ui;

use crate::prelude::*;

use camera::CameraPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use system::resources::GameRegistry;
use ui::{
    controls::UIControls,
    menu::{MenuPlugin, resources::MatchConfig},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(32 * 30, 23 * 30),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .init_state::<MenuState>()
        .init_resource::<MatchConfig>()
        .init_resource::<GameRegistry>()
        .init_resource::<ActionState<UIControls>>()
        .insert_resource(UIControls::default_input_map())
        .add_plugins(InputManagerPlugin::<UIControls>::default())
        .add_plugins(CameraPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
