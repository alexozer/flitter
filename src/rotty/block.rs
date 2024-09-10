use crossterm::style;

#[derive(Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub enum JoinDir {
    Horiz,
    Vert,
    Stack,
}

/// A Block is a recursive composition of other blocks and images.
#[derive(Clone)]
pub enum Block {
    Image(Image),
    Join { dir: JoinDir, blocks: Vec<Block> },
}

#[allow(dead_code)]
impl Block {
    pub fn horiz(self, right: Block) -> Block {
        Block::Join {
            dir: JoinDir::Horiz,
            blocks: vec![self, right],
        }
    }

    pub fn hcat(blocks: impl IntoIterator<Item = Block>) -> Block {
        Block::Join {
            dir: JoinDir::Horiz,
            blocks: blocks.into_iter().collect(),
        }
    }

    pub fn vert(self, down: Block) -> Block {
        Block::Join {
            dir: JoinDir::Vert,
            blocks: vec![self, down],
        }
    }

    pub fn vcat(blocks: impl IntoIterator<Item = Block>) -> Block {
        Block::Join {
            dir: JoinDir::Vert,
            blocks: blocks.into_iter().collect(),
        }
    }

    pub fn stack(self, bottom: Block) -> Block {
        Block::Join {
            dir: JoinDir::Stack,
            blocks: vec![self, bottom],
        }
    }

    pub fn zcat(blocks: impl IntoIterator<Item = Block>) -> Block {
        Block::Join {
            dir: JoinDir::Stack,
            blocks: blocks.into_iter().collect(),
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            Block::Image(img) => img.width,
            Block::Join { dir, blocks } => match dir {
                JoinDir::Horiz => blocks.iter().map(|b| b.width()).sum(),
                JoinDir::Vert | JoinDir::Stack => {
                    blocks.iter().map(|b| b.width()).max().unwrap_or(0)
                }
            },
        }
    }

    // Eat your heart out, npm.
    pub fn left_pad(self, width: u16) -> Self {
        let block_width = self.width();
        if width <= block_width {
            self
        } else {
            let pad = Image::new(
                &" ".repeat((width - block_width) as usize),
                width - block_width,
                TextAlign::Left,
            )
            .build();
            pad.horiz(self)
        }
    }

    pub fn fg_color(self, color: style::Color) -> Self {
        match self {
            Block::Image(img) => Block::Image(img.fg_color(color)),
            Block::Join { dir, blocks } => Block::Join {
                dir,
                blocks: blocks.into_iter().map(|b| b.fg_color(color)).collect(),
            },
        }
    }

    pub fn bg_color(self, color: style::Color) -> Self {
        match self {
            Block::Image(img) => Block::Image(img.bg_color(color)),
            Block::Join { dir, blocks } => Block::Join {
                dir,
                blocks: blocks.into_iter().map(|b| b.bg_color(color)).collect(),
            },
        }
    }
}

#[derive(Clone)]
pub struct Image {
    pub fg_color: Option<style::Color>,
    pub bg_color: Option<style::Color>,
    pub attrs: style::Attributes,

    pub text: String,
    pub width: u16,
    pub align: TextAlign,
}

impl Image {
    pub fn new(t: &str, width: u16, align: TextAlign) -> Self {
        Image {
            text: t.to_string(),
            width,
            align,
            fg_color: None,
            bg_color: None,
            attrs: style::Attributes::none(),
        }
    }

    pub fn fg_color(mut self, color: style::Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn bg_color(mut self, color: style::Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn attr(mut self, attr: style::Attribute) -> Self {
        self.attrs = self.attrs | attr;
        self
    }

    pub fn build(self) -> Block {
        Block::Image(self)
    }
}
