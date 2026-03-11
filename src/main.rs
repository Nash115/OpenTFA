mod camera;
mod level;
mod menu;
mod player;
mod utils;

use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;

use camera::CameraPlugin;
use level::LevelPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(32*30, 23*30),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>() 
        .add_plugins(LevelPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_plugins(MenuPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
