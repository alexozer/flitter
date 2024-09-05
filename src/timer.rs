use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{self, Color};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::{
    rotty::{Block, Image, Renderer, TextAlign},
    split_file::{read_split_file, SplitFile},
};

struct Theme {
    bg: Color,
    normal_text: Color,
    behind_lose: Color,
    behind_gain: Color,
    ahead_lose: Color,
    ahead_gain: Color,
}

static MONOKAI_THEME: Theme = Theme {
    bg: Color::Rgb {
        r: 0x6,
        g: 0x6,
        b: 0x4,
    },
    normal_text: Color::Rgb {
        r: 0xF8,
        g: 0xF8,
        b: 0xF3,
    },
    behind_lose: Color::Rgb {
        r: 0xF9,
        g: 0x25,
        b: 0x72,
    },
    behind_gain: Color::Rgb {
        r: 0xF8,
        g: 0x7A,
        b: 0xA6,
    },
    ahead_lose: Color::White, // TODO
    ahead_gain: Color::Rgb {
        r: 0xA9,
        g: 0xE2,
        b: 0x36,
    },
};

pub struct Timer {
    split_file: SplitFile,
    device_state: DeviceState,
    renderer: Renderer,
}

impl Timer {
    pub fn new(splits_file: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            split_file: read_split_file(splits_file).context("Failed to read splits file")?,
            device_state: DeviceState::new(),
            renderer: Renderer::new(),
        })
    }

    pub fn update(&mut self, delta_seconds: f32) -> anyhow::Result<bool> {
        self.renderer
            .set_default_colors(MONOKAI_THEME.normal_text, MONOKAI_THEME.bg);
        if read_chars()?.contains(&'q') {
            return Ok(false);
        }

        let row1: Vec<Block> = (0..3)
            .map(|i| Image::new(&format!("Cell {i}"), 10, TextAlign::Left).build())
            .collect();
        let row2: Vec<Block> = (3..6)
            .map(|i| Image::new(&format!("Cell {i}"), 10, TextAlign::Left).build())
            .collect();
        let row1 = Block::hcat(row1);
        let row2 = Block::hcat(row2);
        let vert = row1.vert(row2);

        self.renderer.render(&vert)?;
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
