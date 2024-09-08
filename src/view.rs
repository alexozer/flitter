use std::time::Duration;

use crossterm::style::Attribute;

use crate::{
    bigtext::get_big_text,
    rotty::{Block, Image, TextAlign},
    settings::Theme,
    timer_state::{TimerMode, TimerState},
    utils::{format_duration, parse_color},
};

static TIMER_WIDTH: u16 = 48;
static COL_WIDTH: u16 = 12;
pub fn render_view(timer: &TimerState, theme: &Theme) -> Block {
    let elapsed = match timer.mode {
        TimerMode::Initial => Duration::from_secs(0),
        TimerMode::Running { start_time } => start_time.elapsed(),
        TimerMode::Finished { start_time: _ } => timer.splits.last().unwrap().unwrap(),
    };

    let title = &timer.split_file.title;
    let category = &timer.split_file.category;
    let title_block = Image::new(title, TIMER_WIDTH, TextAlign::Center)
        .attr(Attribute::Bold)
        .build();
    let category_block = Image::new(category, TIMER_WIDTH, TextAlign::Center)
        .attr(Attribute::Bold)
        .build();

    let attempts_text = format!(
        "{}/{}",
        timer.split_file.completed, timer.split_file.attempts
    );
    let attempts_block = Image::new(&attempts_text, TIMER_WIDTH, TextAlign::Right).build();

    let spacer_block = Image::new(
        &" ".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .build();

    let headers = ["", "Delta", "Segment", "Split"].map(|h| {
        Image::new(h, COL_WIDTH, TextAlign::Right)
            .fg_color(parse_color(theme.label_text))
            .build()
    });
    let header_row = Block::hcat(headers);

    let line_sep = Image::new(
        &"─".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .fg_color(parse_color(theme.label_text))
    .build();

    let split_rows: Vec<Block> = (0..timer.split_file.split_names.len())
        .map(|i| get_split_row(timer, i as u32, theme))
        .collect();

    let timer = get_big_text(&format_duration(elapsed, 2));
    let timer = timer.left_pad(TIMER_WIDTH);
    let timer = timer.fg_color(parse_color(theme.ahead_gain));

    let mut sections = vec![
        title_block,
        category_block,
        attempts_block,
        spacer_block.clone(),
        header_row,
        line_sep.clone(),
    ];
    sections.extend(split_rows);
    sections.push(line_sep);
    sections.push(spacer_block);
    sections.push(timer);
    Block::vcat(sections)
}

fn get_split_time(idx: i32, splits: &[Option<Duration>]) -> Option<Duration> {
    if idx < 0 {
        Some(Duration::from_secs(0))
    } else {
        splits[idx as usize]
    }
}

fn get_split_row(timer: &TimerState, idx: u32, theme: &Theme) -> Block {
    let split_name = &timer.split_file.split_names[idx as usize];
    let name_col = Image::new(split_name, COL_WIDTH, TextAlign::Left).build();
    let delta_col = get_delta_block(timer, idx, theme);

    let pb_splits: Vec<Option<Duration>> = timer
        .split_file
        .personal_best
        .splits
        .iter()
        .map(|opt_split| opt_split.as_ref().map(|s| s.time))
        .collect();

    let curr_time;
    let prev_time;
    if (idx as usize) < timer.splits.len() {
        curr_time = get_split_time(idx as i32, &timer.splits);
        prev_time = get_split_time(idx as i32 - 1, &timer.splits);
    } else {
        curr_time = get_split_time(idx as i32, &pb_splits);
        prev_time = get_split_time(idx as i32 - 1, &pb_splits);
    }

    let sgmt_text = match (prev_time, curr_time) {
        (Some(prev_time), Some(curr_time)) => format_duration(curr_time - prev_time, 2),
        _ => "-".to_string(),
    };
    let time_text = match curr_time {
        Some(curr_time) => format_duration(curr_time, 2),
        None => "-".to_string(),
    };

    let sgmt_col = Image::new(&sgmt_text, COL_WIDTH, TextAlign::Right).build();
    let time_col = Image::new(&time_text, COL_WIDTH, TextAlign::Right).build();

    let running = matches!(timer.mode, TimerMode::Running { start_time: _ });
    let bg_color = if running && idx as usize == timer.splits.len() {
        theme.highlight
    } else {
        theme.bg
    };
    let bg = Image::new(
        &" ".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .bg_color(parse_color(bg_color))
    .build();
    bg.stack(Block::hcat(vec![name_col, delta_col, sgmt_col, time_col]))
}

fn get_delta_block(timer: &TimerState, idx: u32, theme: &Theme) -> Block {
    // TODO
    Image::new("-", COL_WIDTH, TextAlign::Right).build()
}
