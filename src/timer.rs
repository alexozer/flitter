use std::path::Path;

use anyhow::Context;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::render::Renderer;
use crate::split_file::{read_split_file, SplitFile};

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

    pub fn update(&mut self, delta_seconds: f32) -> bool {
        let keys = self.device_state.get_keys();
        if keys.contains(&Keycode::Q) {
            return false;
        }
        self.renderer.render().unwrap();
        true
    }
}

// pub fn read_char() -> std::io::Result<char> {
//     loop {
//         if let Ok(Event::Key(KeyEvent {
//             code: KeyCode::Char(c),
//             kind: KeyEventKind::Press,
//             modifiers: _,
//             state: _,
//         })) = event::read()
//         {
//             return Ok(c);
//         }
//     }
// }
