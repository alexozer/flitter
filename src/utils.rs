use std::{cmp::Ordering, f64::consts::PI, time::Duration};

use crossterm::style::Color;

use crate::timer_state::{TimerMode, TimerState};

pub fn format_duration(duration: Duration, ms_digits: u32, neg: bool, show_plus: bool) -> String {
    let day_sec = 60 * 60 * 24;
    let hour_sec = 60 * 60;
    let minute_sec = 60;

    let duration_secs = duration.as_secs();
    let days = duration_secs / day_sec;
    let hours = (duration_secs % day_sec) / hour_sec;
    let minutes = (duration_secs % hour_sec) / minute_sec;
    let seconds = duration_secs % minute_sec;
    let milliseconds = duration.subsec_millis();

    let neg_prefix = if neg { "-" } else { "" };
    let plus_prefix = if !neg && show_plus { "+" } else { "" };

    let s = match (days, hours, minutes, seconds, milliseconds) {
        (0, 0, 0, _, _) => format!(
            "{}{}{}.{:03}",
            plus_prefix, neg_prefix, seconds, milliseconds
        ),
        (0, 0, _, _, _) => format!(
            "{}{}{}:{:02}.{:03}",
            plus_prefix, neg_prefix, minutes, seconds, milliseconds
        ),
        (0, _, _, _, _) => format!(
            "{}{}{}:{:02}:{:02}.{:03}",
            plus_prefix, neg_prefix, hours, minutes, seconds, milliseconds
        ),
        _ => format!(
            "{}{}{}:{:02}:{:02}:{:02}.{:03}",
            plus_prefix, neg_prefix, days, hours, minutes, seconds, milliseconds
        ),
    };
    String::from(&s[..(s.len() - (3 - ms_digits as usize))])
}

pub fn parse_color(color_hex: &str) -> Color {
    let color = u32::from_str_radix(color_hex.trim_start_matches('#'), 16).unwrap_or(0);
    Color::Rgb {
        r: ((color >> 16) & 0xFF) as u8,
        g: ((color >> 8) & 0xFF) as u8,
        b: (color & 0xFF) as u8,
    }
}

#[derive(Default, Clone)]
pub struct SegSummary {
    pub live_split: Option<Duration>,
    pub live_seg: Option<Duration>,

    // How far ahead/behind this split is compared to PB
    pub live_delta: Option<Duration>,
    pub live_delta_neg: bool,

    // Time gained or lost this split relative to PB
    pub gained: Option<Duration>,
    pub gained_neg: bool,

    pub pb_split: Option<Duration>,
    pub pb_seg: Option<Duration>,

    pub gold: Option<Duration>,
    pub is_gold_new: bool,
}

pub fn get_run_summary(timer: &TimerState) -> Vec<SegSummary> {
    let mut summary = vec![SegSummary::default(); timer.split_file.split_names.len()];
    let pb = &timer.split_file.personal_best;

    // Calculate PB split times
    for (i, time) in pb.splits.iter().enumerate() {
        summary[i].pb_split = time.as_ref().map(|t| t.time);
    }

    // Calculate PB segment times
    for i in 0..summary.len() {
        if i == 0 {
            summary[i].pb_seg = summary[i].pb_split;
        } else if let (Some(t1), Some(t2)) = (summary[i].pb_split, summary[i - 1].pb_split) {
            summary[i].pb_seg = Some(t1 - t2);
        } else {
            summary[i].pb_seg = None;
        }
    }

    // Calculate live split times
    for (i, split) in timer.splits.iter().enumerate() {
        summary[i].live_split = split.clone();
    }
    if let TimerMode::Running { start_time } = timer.mode {
        summary[timer.splits.len()].live_split = Some(start_time.elapsed());
    }

    // Calculate live segment times
    for i in 0..summary.len() {
        if i == 0 {
            summary[i].live_seg = summary[i].live_split;
        } else if let (Some(t1), Some(t2)) = (summary[i].live_split, summary[i - 1].live_split) {
            summary[i].live_seg = Some(t1 - t2);
        }
    }

    // Calculate live deltas
    for i in 0..summary.len() {
        if let (Some(live_split), Some(pb_split)) = (summary[i].live_split, summary[i].pb_split) {
            // Do math in signed milliseconds because Duration is unsigned
            let ms = live_split.as_millis() as i64 - pb_split.as_millis() as i64;
            summary[i].live_delta = Some(Duration::from_millis(ms.unsigned_abs()));
            summary[i].live_delta_neg = ms < 0;
        }
    }

    // Calculate live gained/lost
    for i in 1..summary.len() {
        if let (Some(delta1), Some(delta2)) = (summary[i].live_delta, summary[i - 1].live_delta) {
            let delta1_ms =
                delta1.as_millis() as i64 * if summary[i].live_delta_neg { -1 } else { 1 };
            let delta2_ms =
                delta2.as_millis() as i64 * if summary[i - 1].live_delta_neg { -1 } else { 1 };
            let gained_ms = delta1_ms - delta2_ms;
            summary[i].gained = Some(Duration::from_millis(gained_ms.unsigned_abs()));
            summary[i].gained_neg = gained_ms < 0;
        }
    }

    // Calculate golds
    for i in 0..summary.len() {
        let live_seg = summary[i].live_seg;
        let gold_seg = timer.split_file.golds[i].as_ref().map(|g| g.duration);
        match (live_seg, gold_seg) {
            (Some(live_seg), Some(gold_seg)) => {
                summary[i].gold = Some(live_seg.min(gold_seg));
                summary[i].is_gold_new = live_seg < gold_seg;
            }
            (None, Some(seg)) | (Some(seg), None) => {
                summary[i].gold = Some(seg);
                summary[i].is_gold_new = false;
            }
            (None, None) => {
                summary[i].gold = None;
                summary[i].is_gold_new = false;
            }
        }
    }

    summary
}
