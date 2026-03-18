use crate::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MatchConfig {
    pub char_register_id: usize,
    pub world_register_id: usize,
    pub ldtk_handle: Option<Handle<LdtkProject>>,
    pub level_index: usize,
}
