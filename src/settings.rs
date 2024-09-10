use anyhow::anyhow;
use std::{collections::HashMap, path::Path, str::FromStr, sync::LazyLock};

use device_query::Keycode;
use serde::Deserialize;

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
        if let Ok(key) = Keycode::from_str(hotkey.0) {
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
