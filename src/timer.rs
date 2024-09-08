use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use device_query::{DeviceQuery, DeviceState};

use crate::settings::{self, Action, Settings};
use crate::split_file::{write_split_file, Gold, Split};
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

        let settings = if config_path.exists() {
            settings::read_settings_file(config_path).context("Failed to read settings file")?
        } else {
            settings::DEFAULT_SETTINGS.clone()
        };

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

    pub fn update(&mut self, _: f32) -> anyhow::Result<bool> {
        if read_terminal_key_chars()?.contains(&'q') {
            return Ok(false);
        }

        let global_keys = self.device_state.get_keys();
        let actions: Vec<Action> = global_keys
            .iter()
            .flat_map(|key| self.settings.global_hotkeys.get(key).copied())
            .collect();

        match self.timer_state.mode {
            TimerMode::Initial => {
                if actions.contains(&Action::Split) {
                    self.timer_state.mode = TimerMode::Running {
                        start_time: Instant::now(),
                    }
                }
            }
            TimerMode::Running { start_time } => {
                let elapsed = start_time.elapsed();

                if actions.contains(&Action::UndoSplit) {
                    self.timer_state.splits.pop();
                }
                if actions.contains(&Action::DeleteSplit) && !self.timer_state.splits.is_empty() {
                    let len = self.timer_state.splits.len();
                    self.timer_state.splits[len - 1] = None;
                }
                if actions.contains(&Action::ResetAndSave) {
                    self.save_golds()?;
                    self.reset_to_initial_mode();
                }
                if actions.contains(&Action::ResetAndDelete) {
                    self.reset_to_initial_mode();
                }
                if actions.contains(&Action::Split) {
                    self.timer_state.splits.push(Some(elapsed));
                    if self.timer_state.splits.len()
                        == self.timer_state.split_file.split_names.len()
                    {
                        self.timer_state.mode = TimerMode::Finished { start_time };
                    }
                }
            }
            TimerMode::Finished { start_time } => {
                if actions.contains(&Action::UndoSplit) {
                    self.timer_state.splits.pop();
                    self.timer_state.mode = TimerMode::Running { start_time };
                }
                if actions.contains(&Action::ResetAndSave) {
                    self.save_golds()?;
                    self.save_personal_best()?;
                    self.reset_to_initial_mode();
                }
                if actions.contains(&Action::ResetAndDelete) {
                    self.reset_to_initial_mode();
                }
            }
        }

        self.renderer
            .set_default_colors(self.settings.theme.normal_text, self.settings.theme.bg);

        let block = view::render_view(&self.timer_state, self.settings.theme);
        self.renderer.render(&block)?;
        Ok(true)
    }

    fn reset_to_initial_mode(&mut self) {
        self.timer_state.mode = TimerMode::Initial;
        self.timer_state.splits.clear();
    }

    fn save_golds(&mut self) -> anyhow::Result<()> {
        // Note that we purposely only iterate over completed splits -
        // golds may be saved even if the run isn't complete
        for i in 0..self.timer_state.splits.len() {
            let curr_duration = if i == 0 {
                self.timer_state.splits[0]
            } else {
                match (self.timer_state.splits[i], self.timer_state.splits[i - 1]) {
                    (Some(curr), Some(prev)) => Some(curr - prev),
                    _ => None,
                }
            };
            let gold_duration = self.timer_state.split_file.golds[i]
                .as_ref()
                .map(|g| g.duration);

            match (curr_duration, gold_duration) {
                (Some(curr_duration), Some(gold_duration)) => {
                    self.timer_state.split_file.golds[i] = Some(Gold {
                        duration: curr_duration.min(gold_duration),
                    });
                }
                (None, Some(duration)) => {
                    self.timer_state.split_file.golds[i] = Some(Gold { duration })
                }
                (Some(_), None) | (None, None) => {}
            }
        }

        write_split_file(&self.timer_state.split_file)?;

        Ok(())
    }

    fn save_personal_best(&mut self) -> anyhow::Result<()> {
        let splits = &self.timer_state.splits;
        let pb = &mut self.timer_state.split_file.personal_best;

        let curr_time = splits[splits.len() - 1].unwrap();
        let pb_time = pb.splits[pb.splits.len() - 1].as_ref().unwrap().time;
        if curr_time < pb_time {
            pb.splits = splits
                .iter()
                .map(|s| s.map(|dur| Split { time: dur }))
                .collect();
        }

        write_split_file(&self.timer_state.split_file)?;

        Ok(())
    }
}

fn read_terminal_key_chars() -> anyhow::Result<Vec<char>> {
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
