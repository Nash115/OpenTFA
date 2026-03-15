use crate::prelude::*;

pub fn set_animation(
    sprite: &mut Sprite,
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    index: usize,
) -> bool {
    let current_layout = sprite
        .texture_atlas
        .as_ref()
        .map(|atlas| atlas.layout.clone());
    let changed = sprite.image != *texture || current_layout != Some(layout.clone());

    if changed {
        sprite.image = texture.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: layout.clone(),
            index,
        });
    }

    changed
}
