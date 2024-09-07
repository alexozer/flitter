use std::time::{Duration, Instant};

use crate::split_file::SplitFile;

pub enum TimerMode {
    Initial,
    Running {
        start_time: Instant,
    },
    Paused {
        start_time: Instant,
        paused_at: Duration,
    },
    Finished,
}

pub struct TimerState {
    pub split_file: SplitFile,
    pub mode: TimerMode,
    pub splits: Vec<Option<Duration>>,
}
