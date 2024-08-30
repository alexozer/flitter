use std::path::Path;

use anyhow::Context;
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::split_file::{read_split_file, SplitFile};

pub struct Timer {
    // Store a hotkey manager - hotkeys are deactivated when dropped
    hotkeys_manager: GlobalHotKeyManager,
    split_file: SplitFile,
    hotkey: HotKey,
}

impl Timer {
    pub fn new(splits_file: &Path) -> anyhow::Result<Self> {
        let hotkeys_manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(None, Code::KeyF);
        hotkeys_manager.register(hotkey)?;

        Ok(Self {
            hotkeys_manager,
            split_file: read_split_file(splits_file).context("Failed to read splits file")?,
            hotkey,
        })
    }

    pub fn update(&mut self, delta_seconds: f32) {
        let global_hotkey_channel = GlobalHotKeyEvent::receiver();
        while let Ok(hotkey) = global_hotkey_channel.try_recv() {
            if hotkey.id == self.hotkey.id {
                println!("{hotkey:?}");
            }
        }
    }
}
