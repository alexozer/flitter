use anyhow::anyhow;
use std::{
    path::PathBuf,
    thread,
    time::{self, Duration},
};

use timer::Timer;

mod bigtext;
mod rotty;
mod settings;
mod split_file;
mod timer;
mod timer_state;
mod utils;
mod view;

static TARGET_FPS: i32 = 60;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <path_to_splits_file>", args[0]));
    }
    let path = PathBuf::from(&args[1]);

    let config_path = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".config")
        .join("flitter-timer")
        .join("config.json");

    let mut timer = Timer::new(&path, &config_path)?;

    let target_frame_time = Duration::from_secs_f32(1.0 / (TARGET_FPS as f32));
    let mut frame_time = target_frame_time;
    loop {
        let start_instant = time::Instant::now();
        let delta_seconds = target_frame_time.max(frame_time).as_secs_f32();
        if !timer.update(delta_seconds)? {
            break;
        }

        frame_time = start_instant.elapsed();
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
    }

    Ok(())
}
