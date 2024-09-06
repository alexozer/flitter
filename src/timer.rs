use std::path::Path;
use std::time::{self, Duration};

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{self, Color};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::timer_state::{TimerMode, TimerState};
use crate::view::{self, Theme, MONOKAI_THEME};
use crate::{
    rotty::{Block, Image, Renderer, TextAlign},
    split_file::{read_split_file, SplitFile},
};

pub struct Timer {
    device_state: DeviceState,
    renderer: Renderer,
    timer_state: TimerState,
    theme: &'static Theme,
}

impl Timer {
    pub fn new(splits_file: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            device_state: DeviceState::new(),
            renderer: Renderer::new(),
            timer_state: TimerState {
                split_file: read_split_file(splits_file).context("Failed to read splits file")?,
                splits: Vec::new(),
                mode: TimerMode::Initial,
            },
            theme: &MONOKAI_THEME,
        })
    }

    pub fn update(&mut self, delta_seconds: f32) -> anyhow::Result<bool> {
        self.renderer
            .set_default_colors(self.theme.normal_text, self.theme.bg);
        if read_chars()?.contains(&'q') {
            return Ok(false);
        }

        let block = view::render_view(&self.timer_state, self.theme);
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
