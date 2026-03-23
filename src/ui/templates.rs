use crate::prelude::*;

use super::components::UiAtlasAnimation;
use super::resources::{UiIconAssets, UiIconSprite};

#[allow(dead_code)]
pub enum KeyboardBtns {
    Space,
    Enter,
    Backspace,
    Other(String),
}

#[allow(dead_code)]
pub enum GamepadBtns {
    A,
    B,
    X,
    Y,
}

#[allow(dead_code)]
pub enum UiIcons {
    NoController,
    Ghost,
    KeyboardButton(KeyboardBtns),
    GamepadButton(GamepadBtns),
}

pub fn spawn_input_icon(
    parent: &mut ChildSpawnerCommands,
    icon_assets: &Res<UiIconAssets>,
    icon: UiIcons,
    size: Option<f32>,
) {
    let icon_size: f32 = match size {
        Some(size) => size,
        None => 30.0,
    };
    match icon {
        UiIcons::NoController => {
            parent.spawn((
                ImageNode {
                    image: icon_assets.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: icon_assets.layout.clone(),
                        index: UiIconSprite::NoController.atlas_index(),
                    }),
                    ..default()
                },
                Node {
                    width: Val::Px(icon_size),
                    height: Val::Px(icon_size),
                    ..default()
                },
            ));
        }

        UiIcons::Ghost => {
            parent.spawn((
                ImageNode {
                    image: icon_assets.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: icon_assets.layout.clone(),
                        index: UiIconSprite::Ghost.atlas_index(),
                    }),
                    ..default()
                },
                Node {
                    width: Val::Px(icon_size),
                    height: Val::Px(icon_size),
                    ..default()
                },
            ));
        }

        UiIcons::KeyboardButton(btn) => {
            let frames = match btn {
                KeyboardBtns::Space => vec![
                    UiIconSprite::KeyboardSpace.atlas_index(),
                    UiIconSprite::KeyboardSpacePressed.atlas_index(),
                ],
                KeyboardBtns::Enter => vec![
                    UiIconSprite::KeyboardEnter.atlas_index(),
                    UiIconSprite::KeyboardEnterPressed.atlas_index(),
                ],
                KeyboardBtns::Backspace => vec![
                    UiIconSprite::KeyboardBackspace.atlas_index(),
                    UiIconSprite::KeyboardBackspacePressed.atlas_index(),
                ],
                KeyboardBtns::Other(_) => vec![
                    UiIconSprite::KeyboardButtonEmpty.atlas_index(),
                    UiIconSprite::KeyboardButtonEmptyPressed.atlas_index(),
                ],
            };
            match btn {
                KeyboardBtns::Space | KeyboardBtns::Enter | KeyboardBtns::Backspace => {
                    parent.spawn((
                        ImageNode {
                            image: icon_assets.texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: icon_assets.layout.clone(),
                                index: frames[0],
                            }),
                            ..default()
                        },
                        Node {
                            width: Val::Px(icon_size),
                            height: Val::Px(icon_size),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        UiAtlasAnimation::new(frames, 0.4),
                    ));
                }
                KeyboardBtns::Other(key) => {
                    parent
                        .spawn((
                            ImageNode {
                                image: icon_assets.texture.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: icon_assets.layout.clone(),
                                    index: frames[0],
                                }),
                                ..default()
                            },
                            Node {
                                width: Val::Px(icon_size),
                                height: Val::Px(icon_size),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            UiAtlasAnimation::new(frames, 0.4),
                        ))
                        .with_children(|icon_parent| {
                            icon_parent.spawn((
                                Text::new(key),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                }
            }
        }

        UiIcons::GamepadButton(btn) => {
            let frames = match btn {
                GamepadBtns::A => vec![
                    UiIconSprite::GamepadButtonA.atlas_index(),
                    UiIconSprite::GamepadButtonAPressed.atlas_index(),
                ],
                GamepadBtns::B => vec![
                    UiIconSprite::GamepadButtonB.atlas_index(),
                    UiIconSprite::GamepadButtonBPressed.atlas_index(),
                ],
                GamepadBtns::X => vec![
                    UiIconSprite::GamepadButtonX.atlas_index(),
                    UiIconSprite::GamepadButtonXPressed.atlas_index(),
                ],
                GamepadBtns::Y => vec![
                    UiIconSprite::GamepadButtonY.atlas_index(),
                    UiIconSprite::GamepadButtonYPressed.atlas_index(),
                ],
            };

            parent.spawn((
                ImageNode {
                    image: icon_assets.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: icon_assets.layout.clone(),
                        index: frames[0],
                    }),
                    ..default()
                },
                Node {
                    width: Val::Px(icon_size),
                    height: Val::Px(icon_size),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                UiAtlasAnimation::new(frames, 0.4),
            ));
        }
    }
}

pub fn spawn_input_icons(
    parent: &mut ChildSpawnerCommands,
    icon_assets: &Res<UiIconAssets>,
    icons: Vec<UiIcons>,
    size: Option<f32>,
) {
    for icon in icons {
        spawn_input_icon(parent, icon_assets, icon, size);
    }
}
