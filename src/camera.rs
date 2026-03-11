use bevy::{
    camera::{ScalingMode, Viewport},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_ecs_ldtk::prelude::*;

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, fit_camera_to_level.run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::Menu), reset_camera_for_menu);
    }
}

#[derive(Component)]
struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn fit_camera_to_level(
    mut camera_query: Query<(&mut Transform, &mut Projection, &mut Camera), With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    world_query: Query<&LdtkProjectHandle>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    if window.height() <= 0.0 {
        return;
    }

    let Some(level_size) = resolve_level_size(&world_query, &ldtk_projects, &level_selection) else {
        return;
    };

    let Ok((mut transform, mut projection, mut camera)) = camera_query.single_mut() else {
        return;
    };

    transform.translation.x = level_size.x * 0.5;
    transform.translation.y = level_size.y * 0.5;

    if let Projection::Orthographic(orthographic) = &mut *projection {
        orthographic.scaling_mode = ScalingMode::Fixed {
            width: level_size.x,
            height: level_size.y,
        };
    }

    let level_aspect = level_size.x / level_size.y;
    let window_aspect = window.width() / window.height();

    let window_physical_width = window.physical_width();
    let window_physical_height = window.physical_height();

    let (viewport_width, viewport_height) = if window_aspect > level_aspect {
        let height = window_physical_height;
        let width = (height as f32 * level_aspect).round() as u32;
        (width, height)
    } else {
        let width = window_physical_width;
        let height = (width as f32 / level_aspect).round() as u32;
        (width, height)
    };

    let viewport_size = UVec2::new(viewport_width, viewport_height);
    let viewport_position = UVec2::new(
        (window_physical_width.saturating_sub(viewport_width)) / 2,
        (window_physical_height.saturating_sub(viewport_height)) / 2,
    );

    camera.viewport = Some(Viewport {
        physical_position: viewport_position,
        physical_size: viewport_size,
        ..default()
    });
}

fn resolve_level_size(
    world_query: &Query<&LdtkProjectHandle>,
    ldtk_projects: &Res<Assets<LdtkProject>>,
    level_selection: &Res<LevelSelection>,
) -> Option<Vec2> {
    for project_handle in world_query.iter() {
        let project = ldtk_projects.get(project_handle.id())?;
        let level = project.find_raw_level_by_level_selection(level_selection.as_ref())?;

        if level.px_wid > 0 && level.px_hei > 0 {
            return Some(Vec2::new(level.px_wid as f32, level.px_hei as f32));
        }
    }

    None
}

fn reset_camera_for_menu(
    mut camera_query: Query<(&mut Transform, &mut Projection, &mut Camera), With<Camera2d>>,
) {
    let Ok((mut transform, mut projection, mut camera)) = camera_query.single_mut() else {
        return;
    };

    transform.translation.x = 0.0;
    transform.translation.y = 0.0;

    if let Projection::Orthographic(orthographic) = &mut *projection {
        orthographic.scaling_mode = bevy::camera::ScalingMode::WindowSize;
    }

    camera.viewport = None;
}
