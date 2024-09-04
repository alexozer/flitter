use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::{
    rotty::{Block, Image, Renderer, TextAlign},
    split_file::{read_split_file, SplitFile},
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
        if read_chars()?.contains(&'q') {
            return Ok(false);
        }
        let line1 = Image::new("Hello, world!", 20, TextAlign::Left)
            .attr(style::Attribute::Bold)
            .build();
        let line2 = Image::new("Goodbye, world!", 20, TextAlign::Right)
            .fg_color(style::Color::Red)
            .attr(style::Attribute::Underlined)
            .build();
        let vert = line1.vert(line2);
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
