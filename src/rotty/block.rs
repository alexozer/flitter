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
}

#[derive(Clone)]
pub struct Image {
    pub fg_color: Option<style::Color>,
    pub bg_color: Option<style::Color>,
    pub attrs: style::Attributes,

    pub text: String,
    pub width: u32,
    pub align: TextAlign,
}

impl Image {
    pub fn new(t: &str, width: u32, align: TextAlign) -> Self {
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
