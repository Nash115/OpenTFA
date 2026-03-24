use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowState {
    Flying,
    Stuck,
}

#[derive(Component)]
pub struct Arrow {
    pub arrow_type: ArrowType,
    pub state: ArrowState,
    pub velocity: Vec3,
    pub float_timer: Timer,
}
impl Arrow {
    pub fn new(arrow_type: ArrowType, direction: Vec3) -> Self {
        let float_duration = if direction.x.abs() < direction.y.abs() {
            0.0
        } else {
            0.2
        };
        Self {
            arrow_type,
            state: ArrowState::Flying,
            velocity: direction * ARROW_VELOCITY * FORCE_MULTIPLIER,
            float_timer: Timer::from_seconds(float_duration, TimerMode::Once),
        }
    }
}
