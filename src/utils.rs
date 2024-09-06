use std::time::Duration;

pub fn format_duration(duration: Duration, ms_digits: u32) -> String {
    let day_sec = 60 * 60 * 24;
    let hour_sec = 60 * 60;
    let minute_sec = 60;

    let duration_secs = duration.as_secs();
    let days = duration_secs / day_sec;
    let hours = (duration_secs % day_sec) / hour_sec;
    let minutes = (duration_secs % hour_sec) / minute_sec;
    let seconds = duration_secs % minute_sec;
    let milliseconds = duration.subsec_millis();

    let s = match (days, hours, minutes, seconds, milliseconds) {
        (0, 0, 0, _, _) => format!("{}.{:03}", seconds, milliseconds),
        (0, 0, _, _, _) => format!("{}:{:02}.{:03}", minutes, seconds, milliseconds),
        (0, _, _, _, _) => format!(
            "{}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        ),
        _ => format!(
            "{}:{:02}:{:02}:{:02}.{:03}",
            days, hours, minutes, seconds, milliseconds
        ),
    };
    String::from(&s[..(s.len() - (3 - ms_digits as usize))])
}
