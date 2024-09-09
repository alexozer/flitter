use std::time::Duration;

use crossterm::style::Attribute;

use crate::{
    bigtext::get_big_text,
    rotty::{Block, Image, TextAlign},
    settings::Theme,
    timer_state::{TimerMode, TimerState},
    utils::{format_duration, get_latest_golds, get_split_time, parse_color},
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

    let timer_block = get_big_text(&format_duration(elapsed, 2, false));
    let timer_block = timer_block.left_pad(TIMER_WIDTH);
    let timer_block = timer_block.fg_color(parse_color(theme.ahead_gain));

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
    sections.push(spacer_block.clone());
    sections.push(timer_block);
    sections.push(spacer_block);
    sections.push(get_prev_segment_block(timer));
    sections.push(get_sum_of_best_block(timer));
    Block::vcat(sections)
}

fn get_split_row(timer: &TimerState, idx: u32, theme: &Theme) -> Block {
    let split_name = &timer.split_file.split_names[idx as usize];
    let name_col = Image::new(split_name, COL_WIDTH, TextAlign::Left).build();
    let delta_col = get_delta_block(timer, idx, theme);

    let pb_splits = get_pb_splits(timer);

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
        (Some(prev_time), Some(curr_time)) => format_duration(curr_time - prev_time, 2, false),
        _ => "-".to_string(),
    };
    let time_text = match curr_time {
        Some(curr_time) => format_duration(curr_time, 2, false),
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

fn get_pb_splits(timer: &TimerState) -> Vec<Option<Duration>> {
    timer
        .split_file
        .personal_best
        .splits
        .iter()
        .map(|opt_split| opt_split.as_ref().map(|s| s.time))
        .collect()
}

fn get_prev_segment_block(timer: &TimerState) -> Block {
    let curr_split = get_split_time(timer.splits.len() as i32 - 1, &timer.splits);
    let prev_split = get_split_time(timer.splits.len() as i32 - 2, &timer.splits);
    let pb_splits = get_pb_splits(timer);
    let curr_pb = get_split_time(timer.splits.len() as i32 - 1, &pb_splits);
    let prev_pb = get_split_time(timer.splits.len() as i32 - 2, &pb_splits);

    let s = if let (Some(curr_split), Some(prev_split), Some(curr_pb), Some(prev_pb)) =
        (curr_split, prev_split, curr_pb, prev_pb)
    {
        // Do math in signed milliseconds because Duration is unsigned
        let curr_split_ms = curr_split.as_millis() as i32;
        let prev_split_ms = prev_split.as_millis() as i32;
        let curr_pb_ms = curr_pb.as_millis() as i32;
        let prev_pb_ms = prev_pb.as_millis() as i32;
        let delta_ms = (curr_split_ms - curr_pb_ms) - (prev_split_ms - prev_pb_ms);
        let delta_dur = Duration::from_millis(delta_ms.unsigned_abs() as u64);
        format_duration(delta_dur, 2, delta_ms < 0)
    } else {
        "-".to_string()
    };

    let label_col = Image::new("Previous Segment", TIMER_WIDTH / 2, TextAlign::Left).build();
    let prev_seg_col = Image::new(&s, TIMER_WIDTH - TIMER_WIDTH / 2, TextAlign::Right).build();
    label_col.horiz(prev_seg_col)
}

fn get_sum_of_best_block(timer: &TimerState) -> Block {
    let latest_golds = get_latest_golds(timer);
    let sob_text = if latest_golds.iter().all(Option::is_some) {
        let sob = latest_golds
            .iter()
            .map(|g| g.as_ref().unwrap().duration)
            .sum();
        format_duration(sob, 2, false)
    } else {
        "-".to_string()
    };

    let label_col = Image::new("Sum of Best Segments", TIMER_WIDTH / 2, TextAlign::Left).build();
    let sob_col = Image::new(&sob_text, TIMER_WIDTH - TIMER_WIDTH / 2, TextAlign::Right).build();
    label_col.horiz(sob_col)
}
