use crate::prelude::*;

use super::components::*;
use super::resources::*;

pub fn load_ui_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("ui/icons.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(UiIconAssets {
        layout: layout_handle,
        texture,
    });
}

pub fn animate_ui_icons(
    time: Res<Time>,
    // 1. On ajoute Option<&Children> pour voir si cette image contient d'autres entités
    mut query: Query<(&mut UiAtlasAnimation, &mut ImageNode, Option<&Children>)>,
    // 2. On crée une deuxième requête pour pouvoir modifier la position (Node) du texte
    mut text_node_query: Query<&mut Node, With<Text>>,
) {
    for (mut anim, mut image, children_opt) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.current_idx = (anim.current_idx + 1) % anim.frames.len();

            if let Some(atlas) = &mut image.texture_atlas {
                atlas.index = anim.frames[anim.current_idx];
            }

            if let Some(children) = children_opt {
                for child in children.iter() {
                    if let Ok(mut text_node) = text_node_query.get_mut(child) {
                        if anim.current_idx == 1 {
                            text_node.top = Val::Px(2.0);
                        } else {
                            text_node.top = Val::Px(0.0);
                        }
                    }
                }
            }
        }
    }
}
