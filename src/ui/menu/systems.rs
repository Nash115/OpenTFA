use crate::prelude::*;

use crate::utils::consts::Z_UI;

use super::components::MenuEntity;

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Text2d::new("OpenTFA\nPress Return to play"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI),
        MenuEntity,
    ));
}

pub fn menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::InGame);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
