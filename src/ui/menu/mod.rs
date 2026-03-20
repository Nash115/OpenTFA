mod components;
pub mod resources;
mod systems;
mod utils;

use crate::prelude::*;

use self::systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            |mut next: ResMut<NextState<MenuState>>| {
                next.set(MenuState::Main);
            },
        )
        // --- Main menu ---
        .add_systems(OnEnter(MenuState::Main), setup_main_menu)
        .add_systems(
            Update,
            handle_main_menu.run_if(in_state(GameState::Menu).and(in_state(MenuState::Main))),
        )
        .add_systems(OnExit(MenuState::Main), cleanup_menu)
        // --- Character selection ---
        .add_systems(OnEnter(MenuState::CharSelect), setup_char_select)
        .add_systems(
            Update,
            (handle_players_join, handle_char_select, show_char_select)
                .chain()
                .run_if(in_state(GameState::Menu).and(in_state(MenuState::CharSelect))),
        )
        .add_systems(OnExit(MenuState::CharSelect), cleanup_menu)
        // --- World selection ---
        .add_systems(OnEnter(MenuState::WorldSelect), setup_world_select)
        .add_systems(
            Update,
            (show_world_select, handle_world_select)
                .run_if(in_state(GameState::Menu).and(in_state(MenuState::WorldSelect))),
        )
        .add_systems(OnExit(MenuState::WorldSelect), cleanup_menu)
        // --- World loading ---
        .add_systems(OnEnter(MenuState::WorldLoading), setup_world_loading)
        .add_systems(
            Update,
            handle_world_loading
                .run_if(in_state(GameState::Menu).and(in_state(MenuState::WorldLoading))),
        )
        .add_systems(OnExit(MenuState::WorldLoading), cleanup_menu)
        // --- Level selection ---
        .add_systems(OnEnter(MenuState::LevelSelect), setup_level_select)
        .add_systems(
            Update,
            (show_level_select, handle_level_select)
                .run_if(in_state(GameState::Menu).and(in_state(MenuState::LevelSelect))),
        )
        .add_systems(OnExit(MenuState::LevelSelect), cleanup_menu)
        // --- Last cleaning ---
        .add_systems(OnEnter(MenuState::Disabled), cleanup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}
