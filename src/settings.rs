use anyhow::anyhow;
use std::{collections::HashMap, path::Path, str::FromStr, sync::LazyLock};

use device_query::Keycode;
use serde::Deserialize;

pub struct Theme {
    pub bg: &'static str,
    pub normal_text: &'static str,
    pub label_text: &'static str,
    pub paused_text: &'static str,
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
    paused_text: "#808080",
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
    Pause,
}

#[derive(Deserialize)]
pub enum ThemeName {
    Flitter,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ParsedSettings {
    pub theme: ThemeName,
    pub global_hotkeys: HashMap<String, Action>,
    pub draw_background: bool,
}

impl Default for ParsedSettings {
    fn default() -> Self {
        Self {
            theme: ThemeName::Flitter,
            global_hotkeys: HashMap::from([
                ("Space".to_string(), Action::Split),
                ("PageUp".to_string(), Action::UndoSplit),
                ("End".to_string(), Action::DeleteSplit),
                ("P".to_string(), Action::Pause),
                ("Backspace".to_string(), Action::ResetAndSave),
                ("Delete".to_string(), Action::ResetAndDelete),
            ]),
            draw_background: true,
        }
    }
}

#[derive(Clone)]
pub struct Settings {
    pub theme: &'static Theme,
    pub global_hotkeys: HashMap<Keycode, Action>,
    pub draw_background: bool,
}

pub static DEFAULT_SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| post_parse_settings(&ParsedSettings::default()).unwrap());

fn post_parse_settings(parsed: &ParsedSettings) -> anyhow::Result<Settings> {
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
        draw_background: parsed.draw_background,
    })
}

pub fn read_settings_file(path: &Path) -> anyhow::Result<Settings> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let parsed: ParsedSettings = serde_json::from_reader(reader)?;
    post_parse_settings(&parsed)
}
