use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerControls {
    Left,
    Right,
    Up,
    Down,
    Jump,
    Respawn,
}
