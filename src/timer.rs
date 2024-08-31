use std::path::Path;

use anyhow::Context;
use device_query::{DeviceQuery, DeviceState};

use crate::split_file::{read_split_file, SplitFile};

pub struct Timer {
    split_file: SplitFile,
    device_state: DeviceState,
}

impl Timer {
    pub fn new(splits_file: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            split_file: read_split_file(splits_file).context("Failed to read splits file")?,
            device_state: DeviceState::new(),
        })
    }

    pub fn update(&mut self, delta_seconds: f32) {
        let keys = self.device_state.get_keys();
        println!("Keys: {keys:?}");
    }
}
