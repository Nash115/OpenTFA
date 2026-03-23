use crate::prelude::*;

#[derive(Component)]
pub struct UiAtlasAnimation {
    pub frames: Vec<usize>,
    pub timer: Timer,
    pub current_idx: usize,
}
impl UiAtlasAnimation {
    pub fn new(frames: Vec<usize>, interval: f32) -> Self {
        Self {
            frames,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            current_idx: 0,
        }
    }
}
