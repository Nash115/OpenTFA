use crate::prelude::*;

use super::components::*;
use super::systems::extract_facing_dir;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    collider: Collider,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct SpawnPointBundle {
    #[with(extract_facing_dir)]
    pub spawn_facing_dir: SpawnFacingDir,
    spawn_point: SpawnPoint,
}
