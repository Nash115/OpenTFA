use super::resources::{MatchConfig, PlayerDevice};

pub fn controller_already_joined(match_cfg: &MatchConfig, player_device: PlayerDevice) -> bool {
    match player_device {
        PlayerDevice::Keyboard => {
            for p in &match_cfg.players {
                if let Some(p) = p {
                    if matches!(p.device, PlayerDevice::Keyboard) {
                        return true;
                    }
                }
            }
            false
        }
        PlayerDevice::Gamepad(gamepad_entity) => {
            for p in &match_cfg.players {
                if let Some(p) = p {
                    let p_gamepad_entity = match p.device {
                        PlayerDevice::Gamepad(e) => Some(e),
                        _ => None,
                    };
                    if matches!(p.device, PlayerDevice::Gamepad(_))
                        && p_gamepad_entity == Some(gamepad_entity)
                    {
                        return true;
                    }
                }
            }
            false
        }
    }
}
