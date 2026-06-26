use serde::Deserialize;

use crate::state::LightState;

#[derive(Debug, Clone, Deserialize)]
struct ThemeFile {
    green: String,
    yellow: String,
    red: String,
}

const CLASSIC: &str = include_str!("../../../src/themes/classic.json");
const MINIMAL: &str = include_str!("../../../src/themes/minimal.json");
const NEON: &str = include_str!("../../../src/themes/neon.json");

fn theme_json(name: &str) -> &'static str {
    match name {
        "minimal" => MINIMAL,
        "neon" => NEON,
        _ => CLASSIC,
    }
}

fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let hex = s.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

pub fn light_rgb(theme_name: &str, state: LightState) -> (u8, u8, u8) {
    let theme: ThemeFile =
        serde_json::from_str(theme_json(theme_name)).expect("builtin theme json must be valid");
    let hex = match state {
        LightState::Green => theme.green,
        LightState::Yellow => theme.yellow,
        LightState::Red => theme.red,
    };
    parse_hex(&hex).unwrap_or((0x3d, 0xdc, 0x67))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_green_matches_json() {
        let (r, g, b) = light_rgb("classic", LightState::Green);
        assert_eq!((r, g, b), (0x3d, 0xdc, 0x67));
    }

    #[test]
    fn minimal_red_matches_json() {
        let (r, g, b) = light_rgb("minimal", LightState::Red);
        assert_eq!((r, g, b), (0xdc, 0x26, 0x26));
    }

    #[test]
    fn unknown_theme_falls_back_to_classic() {
        let (r, g, b) = light_rgb("unknown", LightState::Yellow);
        assert_eq!((r, g, b), (0xff, 0xd3, 0x4d));
    }
}
