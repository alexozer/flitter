use std::{
    path::{Path, PathBuf},
    thread,
    time::{self, Duration},
};

use anyhow::{anyhow, Context};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use split_file::SplitFile;
use timer::Timer;
use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

mod split_file;
mod timer;

static TARGET_FPS: i32 = 60;

struct App {
    timer: Timer,
    last_frame_time: Duration,
}

impl App {
    fn new(splits_file_path: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            timer: Timer::new(splits_file_path)?,
            last_frame_time: Duration::from_secs_f32(1.0 / (TARGET_FPS as f32)),
        })
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let start_instant = time::Instant::now();

        let target_frame_time = Duration::from_secs_f32(1.0 / (TARGET_FPS as f32));
        let delta_seconds = target_frame_time.max(self.last_frame_time).as_secs_f32();
        self.timer.update(delta_seconds);

        let frame_time = start_instant.elapsed();
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
        self.last_frame_time = frame_time;
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <path_to_splits_file>", args[0]));
    }
    let path = PathBuf::from(&args[1]);

    let mut app = App::new(&path)?;
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    Ok(())
}
