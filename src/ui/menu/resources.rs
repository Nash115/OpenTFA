use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerDevice {
    Keyboard,
    Gamepad(Entity),
}
#[derive(Debug)]
pub struct PlayerConfig {
    pub device: PlayerDevice,
    pub char_register_id: usize,
    pub ready: bool,
}

#[derive(Resource, Debug)]
pub struct MatchConfig {
    pub players: [Option<PlayerConfig>; 4],
    pub world_register_id: usize,
    pub ldtk_handle: Option<Handle<LdtkProject>>,
    pub level_index: usize,
}
impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            players: [None, None, None, None],
            world_register_id: 0,
            ldtk_handle: None,
            level_index: 0,
        }
    }
}
