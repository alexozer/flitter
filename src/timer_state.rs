use std::time::{Duration, Instant};

use crate::split_file::SplitFile;

#[derive(Debug)]
pub enum TimerMode {
    Initial,
    Running { start_time: Instant },
    Paused { elapsed_at_pause: Duration },
    Finished { start_time: Instant },
}

pub struct TimerState {
    pub split_file: SplitFile,
    pub mode: TimerMode,
    pub splits: Vec<Option<Duration>>,
    pub anim_ref_time: Instant,
}
