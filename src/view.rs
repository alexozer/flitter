use std::time::Duration;

use crossterm::style::{Attribute, Color};

use crate::{
    bigtext::get_big_text,
    rotty::{Block, Image, TextAlign},
    settings::Theme,
    split_file::Gold,
    timer_state::{TimerMode, TimerState},
    utils::{format_duration, get_run_summary, parse_color, SegSummary},
};

static TIMER_WIDTH: u16 = 48;
static COL_WIDTH: u16 = 12;
pub fn render_view(timer: &TimerState, theme: &Theme) -> Block {
    let summary = get_run_summary(timer);

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
        .map(|i| get_split_row(timer, i as u32, theme, &summary))
        .collect();

    let timer_block = get_big_text(&format_duration(elapsed, 2, false, false));
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
    sections.push(get_prev_segment_block(timer, theme, &summary));
    sections.push(get_sum_of_best_block(&summary));
    Block::vcat(sections)
}

fn get_split_row(timer: &TimerState, idx: u32, theme: &Theme, summary: &[SegSummary]) -> Block {
    let split_name = &timer.split_file.split_names[idx as usize];
    let name_col = Image::new(split_name, COL_WIDTH, TextAlign::Left).build();

    // Build segment text
    let seg_dur = if (idx as usize) < timer.splits.len() {
        summary[idx as usize].live_seg
    } else {
        summary[idx as usize].pb_seg
    };
    let seg_text = match seg_dur {
        Some(seg_dur) => format_duration(seg_dur, 2, false, false),
        None => "-".to_string(),
    };

    // Build split text
    let split_dur = if (idx as usize) < timer.splits.len() {
        summary[idx as usize].live_split
    } else {
        summary[idx as usize].pb_split
    };
    let split_text = match split_dur {
        Some(split_dur) => format_duration(split_dur, 2, false, false),
        None => "-".to_string(),
    };

    let seg_col = Image::new(&seg_text, COL_WIDTH, TextAlign::Right).build();
    let split_col = Image::new(&split_text, COL_WIDTH, TextAlign::Right).build();
    let delta_col = get_delta_block(timer, idx, theme, summary);

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

    bg.stack(Block::hcat(vec![name_col, delta_col, seg_col, split_col]))
}

fn get_delta_block(timer: &TimerState, idx: u32, theme: &Theme, summary: &[SegSummary]) -> Block {
    if let Some(delta) = summary[idx as usize].live_delta {
        // If delta is for the upcoming split:
        // - Hide until segment time exceeds gold, if both exist
        // - Hide till split time exceeds PB split time, if both exist
        // - Else hide indefinitely?
        let show = if let (Some(seg), Some(gold)) =
            (summary[idx as usize].live_seg, summary[idx as usize].gold)
        {
            seg >= gold
        } else if let (Some(live_split), Some(pb_split)) = (
            summary[idx as usize].live_split,
            summary[idx as usize].pb_split,
        ) {
            live_split >= pb_split
        } else {
            true
        };

        if show {
            let delta_neg = summary[idx as usize].live_delta_neg;
            let gain_neg = if summary[idx as usize].gained.is_some() {
                summary[idx as usize].gained_neg
            } else {
                delta_neg
            };

            let color_str = match (delta_neg, gain_neg) {
                (true, true) => theme.ahead_gain,
                (true, false) => theme.ahead_lose,
                (false, true) => theme.behind_gain,
                (false, false) => theme.behind_lose,
            };

            let dur_str = format_duration(delta, 2, delta_neg, true);
            Image::new(&dur_str, COL_WIDTH, TextAlign::Right)
                .fg_color(parse_color(color_str))
                .build()
        } else {
            Image::new(" ", COL_WIDTH, TextAlign::Left).build()
        }
    } else {
        Image::new("-", COL_WIDTH, TextAlign::Right).build()
    }
}

fn get_prev_segment_block(timer: &TimerState, theme: &Theme, summary: &[SegSummary]) -> Block {
    let gained_dur = if timer.splits.is_empty() {
        None
    } else {
        summary[timer.splits.len() - 1].gained
    };

    let color;
    let s;
    if let Some(gained_dur) = gained_dur {
        let neg = summary[timer.splits.len() - 1].gained_neg;
        s = format_duration(gained_dur, 2, neg, true);
        color = if neg {
            parse_color(theme.ahead_gain)
        } else {
            parse_color(theme.behind_lose)
        };
    } else {
        color = parse_color(theme.normal_text);
        s = "-".to_string();
    };

    let label_col = Image::new("Previous Segment", TIMER_WIDTH / 2, TextAlign::Left).build();
    let prev_seg_col = Image::new(&s, TIMER_WIDTH - TIMER_WIDTH / 2, TextAlign::Right)
        .fg_color(color)
        .build();
    label_col.horiz(prev_seg_col)
}

fn get_sum_of_best_block(summary: &[SegSummary]) -> Block {
    let sob_text = if summary.iter().all(|seg| seg.gold.is_some()) {
        let sob = summary.iter().map(|seg| seg.gold.as_ref().unwrap()).sum();
        format_duration(sob, 2, false, false)
    } else {
        "-".to_string()
    };

    let label_col = Image::new("Sum of Best Segments", TIMER_WIDTH / 2, TextAlign::Left).build();
    let sob_col = Image::new(&sob_text, TIMER_WIDTH - TIMER_WIDTH / 2, TextAlign::Right).build();
    label_col.horiz(sob_col)
}
