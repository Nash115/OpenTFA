use crate::prelude::*;

pub struct CharacterData {
    pub name: String,
    pub sprite_path: String,
}

pub struct WorldData {
    pub name: String,
    pub ldtk_path: String,
}

#[derive(Resource)]
pub struct GameRegistry {
    pub characters: Vec<CharacterData>,
    pub worlds: Vec<WorldData>,
}
impl Default for GameRegistry {
    fn default() -> Self {
        Self {
            characters: vec![
                CharacterData {
                    name: "Blue Archer".into(),
                    sprite_path: "sprites/blue_archer".into(),
                },
                CharacterData {
                    name: "Green Archer".into(),
                    sprite_path: "sprites/green_archer".into(),
                },
            ],
            worlds: vec![WorldData {
                name: "Cave".into(),
                ldtk_path: "levels/cave.ldtk".into(),
            }],
        }
    }
}
