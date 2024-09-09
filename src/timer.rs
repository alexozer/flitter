use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::settings::{self, Action, Settings};
use crate::split_file::{write_split_file, Gold, Split};
use crate::timer_state::{TimerMode, TimerState};
use crate::utils::{get_run_summary, parse_color};
use crate::{rotty::Renderer, split_file::read_split_file, view};

pub struct Timer {
    device_state: DeviceState,
    renderer: Renderer,
    timer_state: TimerState,
    settings: Settings,
    prev_keys: HashSet<Keycode>,
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
            prev_keys: HashSet::new(),
        })
    }

    pub fn update(&mut self, _: f32) -> anyhow::Result<bool> {
        if read_terminal_key_chars()?.contains(&'q') {
            return Ok(false);
        }

        let global_keys: HashSet<Keycode> = self.device_state.get_keys().into_iter().collect();
        let actions: Vec<Action> = global_keys
            .iter()
            .filter(|key| !self.prev_keys.contains(key))
            .flat_map(|key| self.settings.global_hotkeys.get(key).copied())
            .collect();
        self.prev_keys = global_keys;

        for action in actions {
            self.apply_action(action)?;
        }

        self.renderer.set_default_colors(
            parse_color(self.settings.theme.normal_text),
            parse_color(self.settings.theme.bg),
        );

        let block = view::render_view(&self.timer_state, self.settings.theme);
        self.renderer.render(&block)?;
        Ok(true)
    }

    pub fn apply_action(&mut self, action: Action) -> anyhow::Result<()> {
        match self.timer_state.mode {
            #[allow(clippy::single_match)]
            TimerMode::Initial => match action {
                Action::Split => {
                    self.timer_state.mode = TimerMode::Running {
                        start_time: Instant::now(),
                    };
                }
                _ => {}
            },
            TimerMode::Running { start_time } => match action {
                Action::Split => {
                    let elapsed = start_time.elapsed();
                    self.timer_state.splits.push(Some(elapsed));
                    if self.timer_state.splits.len()
                        == self.timer_state.split_file.split_names.len()
                    {
                        self.timer_state.mode = TimerMode::Finished { start_time };
                    }
                }
                Action::UndoSplit => {
                    if self.timer_state.splits.is_empty() {
                        self.reset_to_initial_mode();
                    } else {
                        self.timer_state.splits.pop();
                    }
                }
                Action::DeleteSplit => {
                    if !self.timer_state.splits.is_empty() {
                        let len = self.timer_state.splits.len();
                        self.timer_state.splits[len - 1] = None;
                    }
                }
                Action::ResetAndSave => {
                    self.timer_state.split_file.attempts += 1;
                    self.save_golds()?; // Also saves attempts
                    self.reset_to_initial_mode();
                }
                Action::ResetAndDelete => {
                    self.reset_to_initial_mode();
                }
            },
            TimerMode::Finished { start_time } => match action {
                Action::UndoSplit => {
                    self.timer_state.splits.pop();
                    self.timer_state.mode = TimerMode::Running { start_time };
                }
                Action::ResetAndSave => {
                    self.timer_state.split_file.attempts += 1;
                    self.timer_state.split_file.completed += 1;
                    self.save_golds()?; // Also saves attempts/completed
                    self.save_personal_best()?;
                    self.reset_to_initial_mode();
                }
                Action::ResetAndDelete => {
                    self.reset_to_initial_mode();
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn reset_to_initial_mode(&mut self) {
        self.timer_state.mode = TimerMode::Initial;
        self.timer_state.splits.clear();
    }

    fn save_golds(&mut self) -> anyhow::Result<()> {
        let run_summary = get_run_summary(&self.timer_state);

        for (i, seg) in run_summary.iter().enumerate() {
            let file_golds = &mut self.timer_state.split_file.golds;
            file_golds[i] = seg.gold.as_ref().map(|&g| Gold { duration: g });
        }

        write_split_file(&self.timer_state.split_file)?;

        Ok(())
    }

    fn save_personal_best(&mut self) -> anyhow::Result<()> {
        let splits = &self.timer_state.splits;
        let pb = &mut self.timer_state.split_file.personal_best;

        let curr_time = splits.last().unwrap().unwrap();
        let pb_time = pb.splits.last().unwrap().as_ref().unwrap().time;
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
