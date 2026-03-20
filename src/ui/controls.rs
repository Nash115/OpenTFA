use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UIControls {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
    Menu,
}
impl UIControls {
    pub fn default_input_map() -> InputMap<Self> {
        let mut map = InputMap::default();

        map.insert(UIControls::Up, KeyCode::ArrowUp);
        map.insert(UIControls::Up, KeyCode::KeyW);
        map.insert(UIControls::Up, GamepadButton::DPadUp);
        map.insert(
            UIControls::Up,
            GamepadControlDirection::LEFT_UP.threshold(0.25),
        );

        map.insert(UIControls::Down, KeyCode::ArrowDown);
        map.insert(UIControls::Down, KeyCode::KeyS);
        map.insert(UIControls::Down, GamepadButton::DPadDown);
        map.insert(
            UIControls::Down,
            GamepadControlDirection::LEFT_DOWN.threshold(0.25),
        );

        map.insert(UIControls::Left, KeyCode::ArrowLeft);
        map.insert(UIControls::Left, KeyCode::KeyA);
        map.insert(UIControls::Left, GamepadButton::DPadLeft);
        map.insert(
            UIControls::Left,
            GamepadControlDirection::LEFT_LEFT.threshold(0.25),
        );

        map.insert(UIControls::Right, KeyCode::ArrowRight);
        map.insert(UIControls::Right, KeyCode::KeyD);
        map.insert(UIControls::Right, GamepadButton::DPadRight);
        map.insert(
            UIControls::Right,
            GamepadControlDirection::LEFT_RIGHT.threshold(0.25),
        );

        map.insert(UIControls::Confirm, KeyCode::Enter);
        map.insert(UIControls::Confirm, KeyCode::Space);
        map.insert(UIControls::Confirm, GamepadButton::South);

        map.insert(UIControls::Back, KeyCode::Escape);
        map.insert(UIControls::Back, KeyCode::Backspace);
        map.insert(UIControls::Back, GamepadButton::East);

        map.insert(UIControls::Menu, KeyCode::Escape);
        map.insert(UIControls::Menu, GamepadButton::Start);

        map
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CharSelectAction {
    Up,
    Down,
    ToggleReady,
    Leave,
}
