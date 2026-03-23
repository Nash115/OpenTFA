use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiIconSprite {
    NoController,
    Ghost,
    KeyboardButtonEmpty,
    KeyboardButtonEmptyPressed,
    KeyboardSpace,
    KeyboardSpacePressed,
    KeyboardEnter,
    KeyboardEnterPressed,
    KeyboardBackspace,
    KeyboardBackspacePressed,
    GamepadButtonA,
    GamepadButtonAPressed,
    GamepadButtonB,
    GamepadButtonBPressed,
    GamepadButtonX,
    GamepadButtonXPressed,
    GamepadButtonY,
    GamepadButtonYPressed,
}
impl UiIconSprite {
    pub fn atlas_index(&self) -> usize {
        match self {
            UiIconSprite::NoController => 0,
            UiIconSprite::Ghost => 5,
            UiIconSprite::KeyboardButtonEmpty => 1,
            UiIconSprite::KeyboardButtonEmptyPressed => 6,
            UiIconSprite::KeyboardSpace => 2,
            UiIconSprite::KeyboardSpacePressed => 7,
            UiIconSprite::KeyboardEnter => 3,
            UiIconSprite::KeyboardEnterPressed => 8,
            UiIconSprite::KeyboardBackspace => 4,
            UiIconSprite::KeyboardBackspacePressed => 9,
            UiIconSprite::GamepadButtonA => 10,
            UiIconSprite::GamepadButtonAPressed => 15,
            UiIconSprite::GamepadButtonB => 11,
            UiIconSprite::GamepadButtonBPressed => 16,
            UiIconSprite::GamepadButtonX => 12,
            UiIconSprite::GamepadButtonXPressed => 17,
            UiIconSprite::GamepadButtonY => 13,
            UiIconSprite::GamepadButtonYPressed => 18,
        }
    }
}

#[derive(Resource, Default)]
pub struct UiIconAssets {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}
