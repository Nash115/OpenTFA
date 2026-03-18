use crate::prelude::*;

use crate::system::consts::Z_UI;
use crate::system::resources::GameRegistry;

use super::components::{MenuEntity, SelectEntity};
use super::resources::MatchConfig;

// --- Main menu ---

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Text2d::new("OpenTFA\nPress Return to play"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
}

pub fn handle_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(MenuState::CharSelect);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}

// --- Character selection ---

pub fn setup_char_select(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Select a character\nPress Return to confirm"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
}

pub fn show_char_select(
    mut commands: Commands,
    registry: Res<GameRegistry>,
    match_cfg: Res<MatchConfig>,
    query: Query<Entity, With<SelectEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    for (i, character) in registry.characters.iter().enumerate() {
        let text_color = if match_cfg.char_register_id == i {
            Color::linear_rgb(200.0, 200.0, 0.0)
        } else {
            Color::WHITE
        };
        commands.spawn((
            Text2d::new(&character.name),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(text_color),
            Transform::from_xyz(0.0, -60.0 - (i as f32 * 30.0), Z_UI),
            MenuEntity,
            SelectEntity,
        ));
    }
}

pub fn handle_char_select(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MenuState>>,
    registry: Res<GameRegistry>,
    mut match_cfg: ResMut<MatchConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(MenuState::WorldSelect);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if match_cfg.char_register_id == 0 {
            match_cfg.char_register_id = registry.characters.len() - 1;
        } else {
            match_cfg.char_register_id =
                (match_cfg.char_register_id - 1) % registry.characters.len();
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        match_cfg.char_register_id = (match_cfg.char_register_id + 1) % registry.characters.len();
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(MenuState::Main);
    }
}

// --- World selection ---

pub fn setup_world_select(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Select a world\nPress Return to confirm"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
}

pub fn show_world_select(
    mut commands: Commands,
    registry: Res<GameRegistry>,
    match_cfg: Res<MatchConfig>,
    query: Query<Entity, With<SelectEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    for (i, world) in registry.worlds.iter().enumerate() {
        let text_color = if match_cfg.world_register_id == i {
            Color::linear_rgb(200.0, 200.0, 0.0)
        } else {
            Color::WHITE
        };
        commands.spawn((
            Text2d::new(&world.name),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(text_color),
            Transform::from_xyz(0.0, -60.0 - (i as f32 * 30.0), Z_UI),
            MenuEntity,
            SelectEntity,
        ));
    }
}

pub fn handle_world_select(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MenuState>>,
    registry: Res<GameRegistry>,
    mut match_cfg: ResMut<MatchConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(MenuState::WorldLoading);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if match_cfg.world_register_id == 0 {
            match_cfg.world_register_id = registry.worlds.len() - 1;
        } else {
            match_cfg.world_register_id = (match_cfg.world_register_id - 1) % registry.worlds.len();
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        match_cfg.world_register_id = (match_cfg.world_register_id + 1) % registry.worlds.len();
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(MenuState::Main);
    }
}

// --- World loading ---

pub fn setup_world_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Res<GameRegistry>,
    mut match_cfg: ResMut<MatchConfig>,
) {
    commands.spawn((
        Text2d::new("The world is loading..."),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
    let world_ldtk_path = &registry.worlds[match_cfg.world_register_id].ldtk_path;
    let handle: Handle<LdtkProject> = asset_server.load(world_ldtk_path);
    match_cfg.ldtk_handle = Some(handle)
}

pub fn handle_world_loading(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<MatchConfig>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(handle) = &config.ldtk_handle {
        if ldtk_projects.get(handle).is_some() {
            next_menu_state.set(MenuState::LevelSelect);
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_menu_state.set(MenuState::Main);
    }
}

// --- Level selection ---

pub fn setup_level_select(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Select a level\nPress Return to confirm"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
}

pub fn show_level_select(
    mut commands: Commands,
    match_cfg: Res<MatchConfig>,
    query: Query<Entity, With<SelectEntity>>,
    ldtk_projects: Res<Assets<LdtkProject>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    let mut levels_identifiers: Vec<String> = Vec::new();
    if let Some(handle) = &match_cfg.ldtk_handle {
        if let Some(project) = ldtk_projects.get(handle) {
            for level in project.data().root_levels() {
                levels_identifiers.push(level.identifier.clone());
            }
        }
    }

    for (i, level_identifier) in levels_identifiers.iter().enumerate() {
        let text_color = if match_cfg.level_index == i {
            Color::linear_rgb(200.0, 200.0, 0.0)
        } else {
            Color::WHITE
        };
        commands.spawn((
            Text2d::new(level_identifier),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(text_color),
            Transform::from_xyz(0.0, -60.0 - (i as f32 * 30.0), Z_UI),
            MenuEntity,
            SelectEntity,
        ));
    }
}

pub fn handle_level_select(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut match_cfg: ResMut<MatchConfig>,
    ldtk_projects: Res<Assets<LdtkProject>>,
) {
    let mut levels_available: usize = 0;
    if let Some(handle) = &match_cfg.ldtk_handle {
        if let Some(project) = ldtk_projects.get(handle) {
            levels_available = project.data().root_levels().len();
        }
    }
    if levels_available == 0 {
        next_menu_state.set(MenuState::Main);
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_menu_state.set(MenuState::Disabled);
        next_game_state.set(GameState::InGame);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if match_cfg.level_index == 0 {
            match_cfg.level_index = levels_available - 1;
        } else {
            match_cfg.level_index = (match_cfg.level_index - 1) % levels_available;
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        match_cfg.level_index = (match_cfg.level_index + 1) % levels_available;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_menu_state.set(MenuState::Main);
    }
}

// --- Cleanup ---

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
