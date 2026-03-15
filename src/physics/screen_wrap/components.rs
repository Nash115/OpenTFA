use crate::prelude::*;

#[derive(Component)]
pub struct Wrapable;

#[derive(Component)]
pub struct VisualClone {
    pub parent_entity: Entity,
    pub offset: Vec3,
}
