use crate::prelude::*;

use crate::system::consts::Z_UI;
use crate::system::resources::GameRegistry;
use crate::ui::controls::{CharSelectAction, UIControls};

use super::components::{MenuEntity, PlayerIndex, SelectEntity};
use super::resources::{MatchConfig, PlayerConfig, PlayerDevice};
use super::utils::controller_already_joined;

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
    action_state: Res<ActionState<UIControls>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if action_state.just_pressed(&UIControls::Confirm) {
        next_state.set(MenuState::CharSelect);
    }
    if action_state.just_pressed(&UIControls::Menu) {
        app_exit.write(AppExit::Success);
    }
}

// --- Character selection ---

pub fn setup_char_select(mut commands: Commands) {
    commands.insert_resource(MatchConfig::default());
    commands.spawn((
        Text2d::new("Select your character"),
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
    if !match_cfg.is_changed() {
        return;
    }

    for entity in &query {
        commands.entity(entity).despawn();
    }

    for (player_i, _) in match_cfg.players.iter().enumerate() {
        let Some(ref player) = match_cfg.players[player_i] else {
            continue;
        };

        for (character_i, character) in registry.characters.iter().enumerate() {
            let text_color = if player.char_register_id == character_i {
                if player.ready {
                    Color::linear_rgb(0.0, 200.0, 0.0)
                } else {
                    Color::linear_rgb(200.0, 200.0, 0.0)
                }
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
                Transform::from_xyz(
                    -300.0 + (player_i as f32 * 150.0),
                    -60.0 - (character_i as f32 * 30.0),
                    Z_UI,
                ),
                MenuEntity,
                SelectEntity,
            ));
        }
    }
}

pub fn handle_players_join(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut match_cfg: ResMut<MatchConfig>,
) {
    let Some(first_empty_slot) = match_cfg.players.iter().position(|slot| slot.is_none()) else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if !controller_already_joined(&match_cfg, PlayerDevice::Keyboard) {
            match_cfg.players[first_empty_slot] = Some(PlayerConfig {
                device: PlayerDevice::Keyboard,
                char_register_id: 0,
                ready: false,
                join_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
            });

            let mut input_map = InputMap::default();
            input_map
                .insert(CharSelectAction::Up, KeyCode::KeyW)
                .insert(CharSelectAction::Up, KeyCode::ArrowUp)
                .insert(CharSelectAction::Down, KeyCode::KeyS)
                .insert(CharSelectAction::Down, KeyCode::ArrowDown)
                .insert(CharSelectAction::ToggleReady, KeyCode::Space)
                .insert(CharSelectAction::ToggleReady, KeyCode::Enter)
                .insert(CharSelectAction::Leave, KeyCode::Backspace);

            commands.spawn((
                PlayerIndex(first_empty_slot),
                ActionState::<CharSelectAction>::default(),
                input_map,
                MenuEntity,
            ));
        }
    }

    for (gamepad_entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) {
            if !controller_already_joined(&match_cfg, PlayerDevice::Gamepad(gamepad_entity)) {
                match_cfg.players[first_empty_slot] = Some(PlayerConfig {
                    device: PlayerDevice::Gamepad(gamepad_entity),
                    char_register_id: 0,
                    ready: false,
                    join_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
                });

                let mut input_map = InputMap::default();
                input_map
                    .insert(CharSelectAction::Up, GamepadButton::DPadUp)
                    .insert(
                        CharSelectAction::Up,
                        GamepadControlDirection::LEFT_UP.threshold(0.25),
                    )
                    .insert(CharSelectAction::Down, GamepadButton::DPadDown)
                    .insert(
                        CharSelectAction::Down,
                        GamepadControlDirection::LEFT_DOWN.threshold(0.25),
                    )
                    .insert(CharSelectAction::ToggleReady, GamepadButton::South)
                    .insert(CharSelectAction::Leave, GamepadButton::East)
                    .set_gamepad(gamepad_entity);

                commands.spawn((
                    PlayerIndex(first_empty_slot),
                    ActionState::<CharSelectAction>::default(),
                    input_map,
                    MenuEntity,
                ));
            }
        }
    }
}

pub fn handle_char_select(
    mut commands: Commands,
    mut match_cfg: ResMut<MatchConfig>,
    registry: Res<GameRegistry>,
    query: Query<(Entity, &ActionState<CharSelectAction>, &PlayerIndex)>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    time: Res<Time>,
    action_state_ui: Res<ActionState<UIControls>>,
) {
    let max_chars = registry.characters.len();
    if max_chars == 0 {
        return;
    }

    let mut player_leave_this_frame = false;

    for (controller_entity, action_state, player_index) in query.iter() {
        let Some(Some(player_cfg)) = match_cfg.players.get_mut(player_index.0) else {
            continue;
        };

        player_cfg.join_cooldown.tick(time.delta());
        if !player_cfg.join_cooldown.is_finished() {
            continue;
        }

        if action_state.just_pressed(&CharSelectAction::Leave) {
            match_cfg.players[player_index.0] = None;
            commands.entity(controller_entity).despawn();
            player_leave_this_frame = true;
            continue;
        }

        if player_cfg.ready {
            if action_state.just_pressed(&CharSelectAction::ToggleReady) {
                player_cfg.ready = false;
            }
            continue;
        }

        if action_state.just_pressed(&CharSelectAction::Down) {
            player_cfg.char_register_id = (player_cfg.char_register_id + 1) % max_chars;
        }

        if action_state.just_pressed(&CharSelectAction::Up) {
            player_cfg.char_register_id = (player_cfg.char_register_id + max_chars - 1) % max_chars;
        }

        if action_state.just_pressed(&CharSelectAction::ToggleReady) {
            player_cfg.ready = true;
        }
    }

    let active_players_count = match_cfg.players.iter().filter(|p| p.is_some()).count();

    let ready_players_count = match_cfg
        .players
        .iter()
        .filter(|p| {
            if let Some(player) = p {
                player.ready
            } else {
                false
            }
        })
        .count();

    if active_players_count > 0 && active_players_count == ready_players_count {
        next_menu_state.set(MenuState::WorldSelect);
    }

    if action_state_ui.just_pressed(&UIControls::Menu)
        || (action_state_ui.just_pressed(&UIControls::Back)
            && active_players_count == 0
            && !player_leave_this_frame)
    {
        next_menu_state.set(MenuState::Main);
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
    action_state: Res<ActionState<UIControls>>,
    mut next_state: ResMut<NextState<MenuState>>,
    registry: Res<GameRegistry>,
    mut match_cfg: ResMut<MatchConfig>,
) {
    if action_state.just_pressed(&UIControls::Confirm) {
        next_state.set(MenuState::WorldLoading);
    }
    if action_state.just_pressed(&UIControls::Up) {
        if match_cfg.world_register_id == 0 {
            match_cfg.world_register_id = registry.worlds.len() - 1;
        } else {
            match_cfg.world_register_id = (match_cfg.world_register_id - 1) % registry.worlds.len();
        }
    }
    if action_state.just_pressed(&UIControls::Down) {
        match_cfg.world_register_id = (match_cfg.world_register_id + 1) % registry.worlds.len();
    }
    if action_state.just_pressed(&UIControls::Back) {
        next_state.set(MenuState::CharSelect);
    }
    if action_state.just_pressed(&UIControls::Menu) {
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
    action_state: Res<ActionState<UIControls>>,
    config: Res<MatchConfig>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(handle) = &config.ldtk_handle {
        if ldtk_projects.get(handle).is_some() {
            next_menu_state.set(MenuState::LevelSelect);
        }
    }
    if action_state.just_pressed(&UIControls::Back) {
        next_menu_state.set(MenuState::WorldSelect);
    }
    if action_state.just_pressed(&UIControls::Menu) {
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
    action_state: Res<ActionState<UIControls>>,
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
    if action_state.just_pressed(&UIControls::Confirm) {
        next_menu_state.set(MenuState::Disabled);
        next_game_state.set(GameState::InGame);
    }
    if action_state.just_pressed(&UIControls::Up) {
        if match_cfg.level_index == 0 {
            match_cfg.level_index = levels_available - 1;
        } else {
            match_cfg.level_index = (match_cfg.level_index - 1) % levels_available;
        }
    }
    if action_state.just_pressed(&UIControls::Down) {
        match_cfg.level_index = (match_cfg.level_index + 1) % levels_available;
    }
    if action_state.just_pressed(&UIControls::Back) {
        next_menu_state.set(MenuState::WorldSelect);
    }
    if action_state.just_pressed(&UIControls::Menu) {
        next_menu_state.set(MenuState::Main);
    }
}

// --- Cleanup ---

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
