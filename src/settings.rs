use anyhow::anyhow;
use crossterm::style::Color;
use std::{collections::HashMap, path::Path, sync::LazyLock};

use device_query::Keycode;
use serde::Deserialize;

// TODO: can this be done with serde or something?
pub fn keycode_from_str(s: &str) -> Option<Keycode> {
    match s.to_lowercase().as_str() {
        "0" => Some(Keycode::Key0),
        "1" => Some(Keycode::Key1),
        "2" => Some(Keycode::Key2),
        "3" => Some(Keycode::Key3),
        "4" => Some(Keycode::Key4),
        "5" => Some(Keycode::Key5),
        "6" => Some(Keycode::Key6),
        "7" => Some(Keycode::Key7),
        "8" => Some(Keycode::Key8),
        "9" => Some(Keycode::Key9),
        "a" => Some(Keycode::A),
        "b" => Some(Keycode::B),
        "c" => Some(Keycode::C),
        "d" => Some(Keycode::D),
        "e" => Some(Keycode::E),
        "f" => Some(Keycode::F),
        "g" => Some(Keycode::G),
        "h" => Some(Keycode::H),
        "i" => Some(Keycode::I),
        "j" => Some(Keycode::J),
        "k" => Some(Keycode::K),
        "l" => Some(Keycode::L),
        "m" => Some(Keycode::M),
        "n" => Some(Keycode::N),
        "o" => Some(Keycode::O),
        "p" => Some(Keycode::P),
        "q" => Some(Keycode::Q),
        "r" => Some(Keycode::R),
        "s" => Some(Keycode::S),
        "t" => Some(Keycode::T),
        "u" => Some(Keycode::U),
        "v" => Some(Keycode::V),
        "w" => Some(Keycode::W),
        "x" => Some(Keycode::X),
        "y" => Some(Keycode::Y),
        "z" => Some(Keycode::Z),
        "f1" => Some(Keycode::F1),
        "f2" => Some(Keycode::F2),
        "f3" => Some(Keycode::F3),
        "f4" => Some(Keycode::F4),
        "f5" => Some(Keycode::F5),
        "f6" => Some(Keycode::F6),
        "f7" => Some(Keycode::F7),
        "f8" => Some(Keycode::F8),
        "f9" => Some(Keycode::F9),
        "f10" => Some(Keycode::F10),
        "f11" => Some(Keycode::F11),
        "f12" => Some(Keycode::F12),
        "f13" => Some(Keycode::F13),
        "f14" => Some(Keycode::F14),
        "f15" => Some(Keycode::F15),
        "f16" => Some(Keycode::F16),
        "f17" => Some(Keycode::F17),
        "f18" => Some(Keycode::F18),
        "f19" => Some(Keycode::F19),
        "f20" => Some(Keycode::F20),
        "escape" => Some(Keycode::Escape),
        "space" => Some(Keycode::Space),
        "lcontrol" => Some(Keycode::LControl),
        "rcontrol" => Some(Keycode::RControl),
        "lshift" => Some(Keycode::LShift),
        "rshift" => Some(Keycode::RShift),
        "lalt" => Some(Keycode::LAlt),
        "ralt" => Some(Keycode::RAlt),
        "command" => Some(Keycode::Command),
        "loption" => Some(Keycode::LOption),
        "roption" => Some(Keycode::ROption),
        "lmeta" => Some(Keycode::LMeta),
        "rmeta" => Some(Keycode::RMeta),
        "enter" => Some(Keycode::Enter),
        "up" => Some(Keycode::Up),
        "down" => Some(Keycode::Down),
        "left" => Some(Keycode::Left),
        "right" => Some(Keycode::Right),
        "backspace" => Some(Keycode::Backspace),
        "capslock" => Some(Keycode::CapsLock),
        "tab" => Some(Keycode::Tab),
        "home" => Some(Keycode::Home),
        "end" => Some(Keycode::End),
        "pageup" => Some(Keycode::PageUp),
        "pagedown" => Some(Keycode::PageDown),
        "insert" => Some(Keycode::Insert),
        "delete" => Some(Keycode::Delete),
        "numpad0" => Some(Keycode::Numpad0),
        "numpad1" => Some(Keycode::Numpad1),
        "numpad2" => Some(Keycode::Numpad2),
        "numpad3" => Some(Keycode::Numpad3),
        "numpad4" => Some(Keycode::Numpad4),
        "numpad5" => Some(Keycode::Numpad5),
        "numpad6" => Some(Keycode::Numpad6),
        "numpad7" => Some(Keycode::Numpad7),
        "numpad8" => Some(Keycode::Numpad8),
        "numpad9" => Some(Keycode::Numpad9),
        "numpadsubtract" => Some(Keycode::NumpadSubtract),
        "numpadadd" => Some(Keycode::NumpadAdd),
        "numpaddivide" => Some(Keycode::NumpadDivide),
        "numpadmultiply" => Some(Keycode::NumpadMultiply),
        "numpadequals" => Some(Keycode::NumpadEquals),
        "numpadenter" => Some(Keycode::NumpadEnter),
        "numpaddecimal" => Some(Keycode::NumpadDecimal),
        "grave" => Some(Keycode::Grave),
        "minus" => Some(Keycode::Minus),
        "equal" => Some(Keycode::Equal),
        "leftbracket" => Some(Keycode::LeftBracket),
        "rightbracket" => Some(Keycode::RightBracket),
        "backslash" => Some(Keycode::BackSlash),
        "semicolon" => Some(Keycode::Semicolon),
        "apostrophe" => Some(Keycode::Apostrophe),
        "comma" => Some(Keycode::Comma),
        "dot" => Some(Keycode::Dot),
        "slash" => Some(Keycode::Slash),
        _ => None,
    }
}

pub struct Theme {
    pub bg: &'static str,
    pub normal_text: &'static str,
    pub label_text: &'static str,
    pub behind_lose: &'static str,
    pub behind_gain: &'static str,
    pub ahead_lose: &'static str,
    pub ahead_gain: &'static str,
    pub highlight: &'static str,
}

static FLITTER_THEME: Theme = Theme {
    bg: "#060604",
    normal_text: "#F8F8F3",
    label_text: "#9E9E9B",
    behind_lose: "#F92572",
    behind_gain: "#F87AA6",
    ahead_lose: "#ABF7B3",
    ahead_gain: "#1CE82C",
    highlight: "#5B60FF",
};

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Split,
    UndoSplit,
    DeleteSplit,
    ResetAndSave,
    ResetAndDelete,
}

#[derive(Deserialize)]
pub enum ThemeName {
    Flitter,
}

#[derive(Deserialize)]
pub struct ParsedSettings {
    pub theme: ThemeName,
    pub global_hotkeys: HashMap<String, Action>,
}

#[derive(Clone)]
pub struct Settings {
    pub theme: &'static Theme,
    pub global_hotkeys: HashMap<Keycode, Action>,
}

pub static DEFAULT_SETTINGS: LazyLock<Settings> = LazyLock::new(|| Settings {
    theme: &FLITTER_THEME,
    global_hotkeys: HashMap::from([
        (Keycode::Space, Action::Split),
        (Keycode::PageUp, Action::UndoSplit),
        (Keycode::End, Action::DeleteSplit),
        (Keycode::Backspace, Action::ResetAndSave),
        (Keycode::Delete, Action::ResetAndDelete),
    ]),
});

pub fn read_settings_file(path: &Path) -> anyhow::Result<Settings> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let parsed: ParsedSettings = serde_json::from_reader(reader)?;

    let theme = match parsed.theme {
        ThemeName::Flitter => &FLITTER_THEME,
    };

    let mut global_hotkeys = HashMap::<Keycode, Action>::new();
    for hotkey in parsed.global_hotkeys.iter() {
        if let Some(key) = keycode_from_str(hotkey.0) {
            global_hotkeys.insert(key, *hotkey.1);
        } else {
            return Err(anyhow!("Invalid hotkey: {}", &hotkey.0));
        }
    }

    Ok(Settings {
        theme,
        global_hotkeys,
    })
}
