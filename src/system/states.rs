use crate::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    Disabled,
    Main,
    CharSelect,
    WorldSelect,
    WorldLoading,
    LevelSelect,
}
