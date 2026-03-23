pub mod components;
pub mod controls;
pub mod menu;
pub mod resources;
pub mod systems;
pub mod templates;

use crate::prelude::*;

use self::systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_ui_icons)
            .add_systems(Update, animate_ui_icons);
    }
}
