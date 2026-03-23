use crate::prelude::*;

use crate::system::resources::GameRegistry;
use crate::ui::{
    controls::{CharSelectAction, UIControls},
    resources::UiIconAssets,
    templates::{GamepadBtns, KeyboardBtns, UiIcons, spawn_input_icon, spawn_input_icons},
};

use super::components::{MenuEntity, PlayerIndex, ReadyCooldown, SelectContainer, SelectEntity};
use super::resources::{MatchConfig, PlayerConfig, PlayerDevice};
use super::utils::controller_already_joined;

// --- Main menu ---

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    icon_assets: Res<UiIconAssets>,
) {
    let bg_image = asset_server.load("ui/menu-wall.png");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ImageNode::new(bg_image),
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("OpenTFA"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Play Versus"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                    spawn_input_icons(
                        row,
                        &icon_assets,
                        vec![
                            UiIcons::KeyboardButton(KeyboardBtns::Enter),
                            UiIcons::GamepadButton(GamepadBtns::A),
                        ],
                        None,
                    );
                });
        });
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

pub fn setup_char_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MatchConfig::default());

    let bg_image = asset_server.load("ui/menu-wall.png");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ImageNode::new(bg_image),
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select your character"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    column_gap: Val::Px(16.0),
                    ..default()
                },
                SelectContainer,
            ));
        });
}

pub fn show_char_select(
    mut commands: Commands,
    registry: Res<GameRegistry>,
    match_cfg: Res<MatchConfig>,
    select_query: Query<Entity, With<SelectEntity>>,
    container_query: Query<Entity, With<SelectContainer>>,
    icon_assets: Res<UiIconAssets>,
) {
    if !match_cfg.is_changed() {
        return;
    }

    for entity in &select_query {
        commands.entity(entity).despawn();
    }

    let Ok(container) = container_query.single() else {
        return;
    };

    let player_colors = [
        Color::linear_rgb(1.0, 0.35, 0.35),
        Color::linear_rgb(0.35, 0.55, 1.0),
        Color::linear_rgb(0.35, 0.9, 0.35),
        Color::linear_rgb(1.0, 0.85, 0.15),
    ];

    commands.entity(container).with_children(|parent| {
        for player_i in 0..4usize {
            let player_color = player_colors[player_i];

            if let Some(ref player) = match_cfg.players[player_i] {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            min_width: Val::Px(210.0),
                            padding: UiRect::all(Val::Px(16.0)),
                            row_gap: Val::Px(6.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                        SelectEntity,
                    ))
                    .with_children(|panel| {
                        // Player header
                        panel.spawn((
                            Text::new(format!("Player {}", player_i + 1)),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(player_color),
                        ));

                        // Separator
                        panel.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(2.0),
                                margin: UiRect::vertical(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));

                        // Character list
                        for (char_i, character) in registry.characters.iter().enumerate() {
                            let is_selected = player.char_register_id == char_i;
                            let text_color = if is_selected {
                                if player.ready {
                                    Color::linear_rgb(0.1, 0.9, 0.1)
                                } else {
                                    Color::linear_rgb(1.0, 1.0, 0.0)
                                }
                            } else {
                                Color::srgba(0.8, 0.8, 0.8, 0.6)
                            };
                            let prefix = if is_selected { "> " } else { "  " };
                            panel.spawn((
                                Text::new(format!("{}{}", prefix, character.name)),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(text_color),
                            ));
                        }

                        // Separator
                        panel.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(2.0),
                                margin: UiRect::vertical(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));

                        // Status
                        let (status_text, status_color) = if player.ready {
                            ("READY!", Color::linear_rgb(0.1, 0.9, 0.1))
                        } else {
                            ("Choosing...", Color::linear_rgb(1.0, 1.0, 1.0))
                        };
                        panel.spawn((
                            Text::new(status_text),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(status_color),
                        ));

                        // Ready / Unready hint
                        let ready_label = if player.ready { "Unready" } else { "Ready" };
                        panel
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|row| {
                                match player.device {
                                    PlayerDevice::Keyboard => {
                                        spawn_input_icon(
                                            row,
                                            &icon_assets,
                                            UiIcons::KeyboardButton(KeyboardBtns::Space),
                                            Some(24.0),
                                        );
                                    }
                                    PlayerDevice::Gamepad(_) => {
                                        spawn_input_icon(
                                            row,
                                            &icon_assets,
                                            UiIcons::GamepadButton(GamepadBtns::A),
                                            Some(24.0),
                                        );
                                    }
                                }
                                row.spawn((
                                    Text::new(ready_label),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                                ));
                            });

                        // Leave hint (only when not ready)
                        if !player.ready {
                            panel
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    match player.device {
                                        PlayerDevice::Keyboard => {
                                            spawn_input_icon(
                                                row,
                                                &icon_assets,
                                                UiIcons::KeyboardButton(KeyboardBtns::Backspace),
                                                Some(24.0),
                                            );
                                        }
                                        PlayerDevice::Gamepad(_) => {
                                            spawn_input_icon(
                                                row,
                                                &icon_assets,
                                                UiIcons::GamepadButton(GamepadBtns::B),
                                                Some(24.0),
                                            );
                                        }
                                    }
                                    row.spawn((
                                        Text::new("Leave"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                                    ));
                                });
                        }
                    });
            } else {
                // Empty slot
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            min_width: Val::Px(210.0),
                            min_height: Val::Px(220.0),
                            padding: UiRect::all(Val::Px(16.0)),
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                        SelectEntity,
                    ))
                    .with_children(|panel| {
                        spawn_input_icon(panel, &icon_assets, UiIcons::Ghost, Some(52.0));

                        panel.spawn((
                            Text::new(format!("P{}", player_i + 1)),
                            TextFont {
                                font_size: 36.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));

                        panel
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|row| {
                                spawn_input_icons(
                                    row,
                                    &icon_assets,
                                    vec![
                                        UiIcons::KeyboardButton(KeyboardBtns::Enter),
                                        UiIcons::GamepadButton(GamepadBtns::A),
                                    ],
                                    Some(24.0),
                                );
                            });

                        panel.spawn((
                            Text::new("to join"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));
                    });
            }
        }
    });
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
                ReadyCooldown(Timer::from_seconds(0.3, TimerMode::Once)),
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
                    ReadyCooldown(Timer::from_seconds(0.3, TimerMode::Once)),
                ));
            }
        }
    }
}

pub fn handle_char_select(
    mut commands: Commands,
    mut match_cfg: ResMut<MatchConfig>,
    registry: Res<GameRegistry>,
    mut query: Query<(
        Entity,
        &ActionState<CharSelectAction>,
        &PlayerIndex,
        &mut ReadyCooldown,
    )>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    time: Res<Time>,
    action_state_ui: Res<ActionState<UIControls>>,
) {
    let max_chars = registry.characters.len();
    if max_chars == 0 {
        return;
    }

    let mut player_leave_this_frame = false;

    for (controller_entity, action_state, player_index, mut ready_cooldown) in query.iter_mut() {
        ready_cooldown.0.tick(time.delta());
        if !ready_cooldown.0.is_finished() {
            continue;
        }

        let Some(player_cfg_read) = &match_cfg.players[player_index.0] else {
            continue;
        };

        let is_ready = player_cfg_read.ready;
        let wants_leave = action_state.just_pressed(&CharSelectAction::Leave);
        let wants_ready = action_state.just_pressed(&CharSelectAction::ToggleReady);
        let wants_up = action_state.just_pressed(&CharSelectAction::Up);
        let wants_down = action_state.just_pressed(&CharSelectAction::Down);

        if !wants_leave && !wants_ready && !wants_up && !wants_down {
            continue;
        }

        let Some(player_cfg_mut) = &mut match_cfg.players[player_index.0] else {
            continue;
        };

        if wants_leave {
            match_cfg.players[player_index.0] = None;
            commands.entity(controller_entity).despawn();
            player_leave_this_frame = true;
            continue;
        }

        if is_ready {
            if action_state.just_pressed(&CharSelectAction::ToggleReady) {
                player_cfg_mut.ready = false;
            }
            continue;
        }

        if action_state.just_pressed(&CharSelectAction::Down) {
            player_cfg_mut.char_register_id = (player_cfg_mut.char_register_id + 1) % max_chars;
        }

        if action_state.just_pressed(&CharSelectAction::Up) {
            player_cfg_mut.char_register_id =
                (player_cfg_mut.char_register_id + max_chars - 1) % max_chars;
        }

        if action_state.just_pressed(&CharSelectAction::ToggleReady) {
            player_cfg_mut.ready = true;
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

pub fn setup_world_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_image = asset_server.load("ui/menu-wall.png");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ImageNode::new(bg_image),
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select a world"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    column_gap: Val::Px(16.0),
                    ..default()
                },
                SelectContainer,
            ));
        });
}

pub fn show_world_select(
    mut commands: Commands,
    registry: Res<GameRegistry>,
    match_cfg: Res<MatchConfig>,
    select_query: Query<Entity, With<SelectEntity>>,
    container_query: Query<Entity, With<SelectContainer>>,
    icon_assets: Res<UiIconAssets>,
) {
    if !match_cfg.is_changed() {
        return;
    }

    for entity in &select_query {
        commands.entity(entity).despawn();
    }

    let Ok(container) = container_query.single() else {
        return;
    };

    commands.entity(container).with_children(|parent| {
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    min_width: Val::Px(210.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                SelectEntity,
            ))
            .with_children(|panel| {
                // World entries
                for (i, world) in registry.worlds.iter().enumerate() {
                    let is_selected = match_cfg.world_register_id == i;
                    let text_color = if is_selected {
                        Color::linear_rgb(1.0, 1.0, 0.0)
                    } else {
                        Color::srgba(0.8, 0.8, 0.8, 0.6)
                    };
                    let prefix = if is_selected { "> " } else { "  " };
                    panel.spawn((
                        Text::new(format!("{}{}", prefix, world.name)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                }

                // Separator
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        margin: UiRect::vertical(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                ));

                // Choose hint
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(4.0),
                        ..default()
                    })
                    .with_children(|row| {
                        spawn_input_icons(
                            row,
                            &icon_assets,
                            vec![
                                UiIcons::KeyboardButton(KeyboardBtns::Enter),
                                UiIcons::GamepadButton(GamepadBtns::A),
                            ],
                            Some(24.0),
                        );
                        row.spawn((
                            Text::new("Choose world"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));
                    });
            });
    });
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
    let bg_image = asset_server.load("ui/menu-wall.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        ImageNode::new(bg_image),
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

pub fn setup_level_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_image = asset_server.load("ui/menu-wall.png");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ImageNode::new(bg_image),
            MenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select a level"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    column_gap: Val::Px(16.0),
                    ..default()
                },
                SelectContainer,
            ));
        });
}

pub fn show_level_select(
    mut commands: Commands,
    match_cfg: Res<MatchConfig>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    select_query: Query<Entity, With<SelectEntity>>,
    container_query: Query<Entity, With<SelectContainer>>,
    icon_assets: Res<UiIconAssets>,
) {
    if !match_cfg.is_changed() {
        return;
    }

    for entity in &select_query {
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

    let Ok(container) = container_query.single() else {
        return;
    };

    commands.entity(container).with_children(|parent| {
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    min_width: Val::Px(210.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                SelectEntity,
            ))
            .with_children(|panel| {
                // World entries
                for (i, level_identifier) in levels_identifiers.iter().enumerate() {
                    let is_selected = match_cfg.level_index == i;
                    let text_color = if is_selected {
                        Color::linear_rgb(1.0, 1.0, 0.0)
                    } else {
                        Color::srgba(0.8, 0.8, 0.8, 0.6)
                    };
                    let prefix = if is_selected { "> " } else { "  " };
                    panel.spawn((
                        Text::new(format!("{}{}", prefix, level_identifier)),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                }

                // Separator
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        margin: UiRect::vertical(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                ));

                // Choose hint
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(4.0),
                        ..default()
                    })
                    .with_children(|row| {
                        spawn_input_icons(
                            row,
                            &icon_assets,
                            vec![
                                UiIcons::KeyboardButton(KeyboardBtns::Enter),
                                UiIcons::GamepadButton(GamepadBtns::A),
                            ],
                            Some(24.0),
                        );
                        row.spawn((
                            Text::new("Choose world"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
                        ));
                    });
            });
    });
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
