use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use device_query::DeviceState;

use crate::settings::{self, Settings};
use crate::timer_state::{TimerMode, TimerState};
use crate::{rotty::Renderer, split_file::read_split_file, view};

pub struct Timer {
    device_state: DeviceState,
    renderer: Renderer,
    timer_state: TimerState,
    settings: Settings,
}

impl Timer {
    pub fn new(splits_file: &Path, config_path: &Path) -> anyhow::Result<Self> {
        let split_file = read_split_file(splits_file).context("Failed to read splits file")?;

        let settings: Settings;
        if config_path.exists() {
            settings = settings::read_settings_file(&config_path)
                .context("Unable to read settings file")?;
        } else {
            settings = settings::DEFAULT_SETTINGS.clone();
        }

        Ok(Self {
            device_state: DeviceState::new(),
            renderer: Renderer::new(),
            timer_state: TimerState {
                split_file,
                splits: Vec::new(),
                mode: TimerMode::Initial,
            },
            settings,
        })
    }

    pub fn update(&mut self, delta_seconds: f32) -> anyhow::Result<bool> {
        self.renderer
            .set_default_colors(self.settings.theme.normal_text, self.settings.theme.bg);
        if read_chars()?.contains(&'q') {
            return Ok(false);
        }

        let block = view::render_view(&self.timer_state, self.settings.theme);
        self.renderer.render(&block)?;
        Ok(true)
    }
}

fn read_chars() -> anyhow::Result<Vec<char>> {
    let mut input_chars = Vec::new();

    while event::poll(Duration::from_secs(0))? {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        }) = event::read()?
        {
            input_chars.push(c);
        }
    }

    Ok(input_chars)
}
