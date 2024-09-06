use std::time::Duration;

use crossterm::style::{Attribute, Color};

use crate::{
    bigtext::get_big_text,
    rotty::{Block, Image, TextAlign},
    timer_state::TimerState,
    utils::format_duration,
};

static TIMER_WIDTH: u32 = 48;
static COL_WIDTH: u32 = 12;

pub struct Theme {
    pub bg: Color,
    pub normal_text: Color,
    pub label_text: Color,
    pub behind_lose: Color,
    pub behind_gain: Color,
    pub ahead_lose: Color,
    pub ahead_gain: Color,
}

pub static MONOKAI_THEME: Theme = Theme {
    bg: Color::Rgb {
        r: 0x6,
        g: 0x6,
        b: 0x4,
    },
    normal_text: Color::Rgb {
        r: 0xF8,
        g: 0xF8,
        b: 0xF3,
    },
    label_text: Color::Rgb {
        r: 0x9E,
        g: 0x9E,
        b: 0x9B,
    },
    behind_lose: Color::Rgb {
        r: 0xF9,
        g: 0x25,
        b: 0x72,
    },
    behind_gain: Color::Rgb {
        r: 0xF8,
        g: 0x7A,
        b: 0xA6,
    },
    ahead_lose: Color::Rgb {
        r: 0xC6,
        g: 0xEA,
        b: 0x7C,
    },
    ahead_gain: Color::Rgb {
        r: 0xA9,
        g: 0xE2,
        b: 0x36,
    },
};

pub fn render_view(timer_state: &TimerState, theme: &Theme) -> Block {
    let title = &timer_state.split_file.title;
    let category = &timer_state.split_file.category;
    let title_block = Image::new(title, TIMER_WIDTH, TextAlign::Center)
        .attr(Attribute::Bold)
        .build();
    let category_block = Image::new(category, TIMER_WIDTH, TextAlign::Center)
        .attr(Attribute::Bold)
        .build();

    let spacer_block = Image::new(
        &" ".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .build();

    let headers = ["", "Delta", "Sgmt", "Time"].map(|h| {
        Image::new(h, COL_WIDTH, TextAlign::Right)
            .fg_color(theme.label_text)
            .build()
    });
    let header_row = Block::hcat(headers);

    let line_sep = Image::new(
        &"─".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .fg_color(theme.label_text)
    .build();

    let split_rows: Vec<Block> = (0..timer_state.split_file.split_names.len())
        .map(|i| get_split_row(timer_state, i as u32))
        .collect();

    let timer = get_big_text(&format_duration(Duration::from_secs(0), 2));
    let timer = timer.left_pad(TIMER_WIDTH);

    let mut sections = vec![
        title_block,
        category_block,
        spacer_block.clone(),
        header_row,
        line_sep,
    ];
    sections.extend(split_rows);
    sections.push(spacer_block);
    sections.push(timer);
    Block::vcat(sections)
}

fn get_split_row(timer_state: &TimerState, idx: u32) -> Block {
    let split_name = &timer_state.split_file.split_names[idx as usize];
    let name_col = Image::new(split_name, COL_WIDTH, TextAlign::Left).build();
    let delta_col = Image::new("-", COL_WIDTH, TextAlign::Right).build();

    let pb = &timer_state.split_file.personal_best;

    // Currently only shows PB splits
    let prev_time = if idx == 0 {
        Some(Duration::from_secs(0))
    } else {
        pb.splits[(idx - 1) as usize]
            .as_ref()
            .map(|split| split.time)
    };
    let curr_time = pb.splits[idx as usize].as_ref().map(|split| split.time);
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

    Block::hcat(vec![name_col, delta_col, sgmt_col, time_col])
}
