use crossterm::style::{Attribute, Color};

use crate::{
    bigtext::get_big_text,
    rotty::{Block, Image, TextAlign},
    timer_state::TimerState,
};

static TIMER_WIDTH: u32 = 40;
static COL_WIDTH: u32 = 10;

pub struct Theme {
    pub bg: Color,
    pub normal_text: Color,
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

    let headers =
        ["", "Delta", "Sgmt", "Time"].map(|h| Image::new(h, COL_WIDTH, TextAlign::Right).build());
    let header_row = Block::hcat(headers);

    let line_sep = Image::new(
        &"─".repeat(TIMER_WIDTH as usize),
        TIMER_WIDTH,
        TextAlign::Left,
    )
    .build();

    let timer = get_big_text("10:00.492");

    let sections = vec![
        title_block,
        category_block,
        spacer_block.clone(),
        header_row,
        line_sep,
        timer,
    ];
    Block::vcat(sections)
}
