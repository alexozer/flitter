use core::time;
use std::time::{Duration, Instant};

use crate::split_file::SplitFile;

pub enum TimerMode {
    Initial,
    Running {
        start_time: Instant,
        pause_time: Duration,
    },
    Paused {
        start_time: Instant,
        pause_time: Duration,
        paused_at: Duration,
    },
    Finished,
}

pub struct TimerState {
    pub split_file: SplitFile,
    pub splits: Vec<Option<Duration>>,
    pub mode: TimerMode,
}
