use std::{cmp::Ordering, time::Duration};

use crossterm::style::Color;

use crate::timer_state::TimerState;

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

pub fn get_split_time(idx: i32, splits: &[Option<Duration>]) -> Option<Duration> {
    match idx.cmp(&-1) {
        Ordering::Less => None,
        Ordering::Equal => Some(Duration::from_secs(0)),
        Ordering::Greater => splits[idx as usize],
    }
}

pub struct NewGold {
    pub duration: Duration,
    pub new: bool,
}

pub fn get_latest_golds(timer: &TimerState) -> Vec<Option<NewGold>> {
    let mut golds: Vec<Option<NewGold>> = timer
        .split_file
        .golds
        .iter()
        .map(|gold| {
            gold.as_ref().map(|g| NewGold {
                duration: g.duration,
                new: false,
            })
        })
        .collect();
    for i in 0..timer.splits.len() {
        let curr_split = get_split_time(i as i32, &timer.splits);
        let prev_split = get_split_time(i as i32 - 1, &timer.splits);
        if let (Some(curr_split), Some(prev_split)) = (curr_split, prev_split) {
            let delta = curr_split - prev_split;
            match golds[i].as_ref() {
                Some(g) => {
                    if delta < g.duration {
                        golds[i] = Some(NewGold {
                            duration: delta,
                            new: true,
                        })
                    }
                }
                None => {
                    golds[i] = Some(NewGold {
                        duration: delta,
                        new: true,
                    })
                }
            }
        }
    }
    golds
}
