use crate::rotty::{Block, Image, TextAlign};

static BIG_FONT: &[&str] = &[
    "00000111112222233333444445555566666777778888899999  !!::..",
    ".^^.  .|  .^^. .^^. .  | |^^^ .^^  ^^^| .^^. .^^.   |     ",
    "|  |   |    .^   .^ |..| |..  |..    ][ ^..^ ^..|   | ^   ",
    "|  |   |  .^   .  |    |    | |  |   |  |  |    |   ^ ^   ",
    " ^^   ^^^ ^^^^  ^^     ^ ^^^   ^^    ^   ^^   ^^    ^   ^ ",
];

fn get_char_block(c: char) -> Option<Block> {
    let start = BIG_FONT[0].find(c)?;
    let end = BIG_FONT[0].rfind(c)? + String::from(c).len();

    let row_blocks: Vec<Block> = BIG_FONT[1..]
        .iter()
        .map(|s| {
            let substr = &s[start..end];
            let replaced: String = substr
                .chars()
                .map(|ch| match ch {
                    '[' => '\u{258C}',
                    ']' => '\u{2590}',
                    '|' => '\u{2588}',
                    '.' => '\u{2584}',
                    '^' => '\u{2580}',
                    ch => ch,
                })
                .collect();
            Image::new(&replaced, replaced.chars().count() as u32, TextAlign::Left).build()
        })
        .collect();

    Some(Block::vcat(row_blocks))
}

pub fn get_big_text(text: &str) -> Block {
    let big_char_blocks: Vec<_> = text.chars().flat_map(get_char_block).collect();
    Block::hcat(big_char_blocks)
}
