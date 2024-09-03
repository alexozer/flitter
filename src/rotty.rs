use std::ops::Add;

use crossterm::style;

enum TextAlign {
    Left,
    Right,
    Center,
}

enum JoinDir {
    Horiz,
    Vert,
    Stack,
}

/// A Block is a recursive composition of other blocks and images.
enum Block {
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

struct Image {
    fg_color: Option<style::Color>,
    bg_color: Option<style::Color>,
    attrs: style::Attributes,

    text: String,
    width: u32,
    align: TextAlign,
}

impl Image {
    pub fn text(t: &str, width: u32, align: TextAlign) -> Self {
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

#[derive(Clone)]
struct Cell {
    ch: char, // Assuming this is always one visual character wide
    fg_color: style::Color,
    bg_color: style::Color,
    attrs: style::Attributes,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: ' ',
            fg_color: style::Color::White,
            bg_color: style::Color::Black,
            attrs: style::Attributes::none(),
        }
    }
}

struct RenderBuffer {
    cells: Vec<Cell>,
    columns: u32,
    dummy_cell: Cell, // Out-of-bounds writes go here
}

impl RenderBuffer {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            cells: vec![Cell::default(); (rows * columns) as usize],
            columns,
            dummy_cell: Cell::default(),
        }
    }

    pub fn at(&mut self, point: Point) -> &mut Cell {
        let i = (point.y * self.columns + point.x) as usize;
        if i >= self.cells.len() {
            &mut self.dummy_cell
        } else {
            &mut self.cells[i]
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy)]
struct AABB {
    top_left: Point,
    size: Point,
}

// Renders the block starting at the given point, and returns a bounding box of what
// was rendered
fn render(block: &Block, top_left: Point, buffer: &mut RenderBuffer) -> AABB {
    match block {
        Block::Image(image) => render_image(image, top_left, buffer),
        Block::Join { dir, blocks } => render_join(dir, blocks, top_left, buffer),
    }
}

fn render_image(image: &Image, top_left: Point, buffer: &mut RenderBuffer) -> AABB {
    let chars = image.text.chars().collect::<Vec<char>>();

    // Calculate text start based on alignment
    let x: u32 = match image.align {
        TextAlign::Left => 0,
        TextAlign::Center => {
            if (chars.len() as u32) < image.width {
                (image.width - chars.len() as u32) / 2
            } else {
                0
            }
        }
        TextAlign::Right => {
            let offset = image.width as i32 - chars.len() as i32;
            offset.max(0) as u32
        }
    };

    // Write to render buffer
    for (i, ch) in image.text.chars().enumerate() {
        let cell: &mut Cell = buffer.at(Point {
            x: top_left.x + x + (i as u32),
            y: top_left.y,
        });
        cell.ch = ch;
        if let Some(bg_color) = image.bg_color {
            cell.bg_color = bg_color;
        }
        if let Some(fg_color) = image.fg_color {
            cell.fg_color = fg_color;
        }
        cell.attrs = cell.attrs | image.attrs;
    }

    AABB {
        top_left: Point {
            x: top_left.x,
            y: top_left.y,
        },
        size: Point {
            x: image.width,
            y: 1,
        },
    }
}

fn render_join(
    dir: &JoinDir,
    blocks: &Vec<Block>,
    top_left: Point,
    buffer: &mut RenderBuffer,
) -> AABB {
    let mut aabb = AABB {
        top_left,
        size: Point { x: 0, y: 0 },
    };
    for b in blocks {
        match dir {
            JoinDir::Horiz => {
                let render_pos = Point {
                    x: aabb.top_left.x + aabb.size.x,
                    y: aabb.top_left.y,
                };
                let sub_aabb = render(b, render_pos, buffer);
                aabb.size.x += sub_aabb.size.x;
                aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
            }
            JoinDir::Vert => {
                let render_pos = Point {
                    x: aabb.top_left.x,
                    y: aabb.top_left.y + aabb.size.y,
                };
                let sub_aabb = render(b, render_pos, buffer);
                aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                aabb.size.y += sub_aabb.size.y;
            }
            JoinDir::Stack => {
                let render_pos = aabb.top_left;
                let sub_aabb = render(b, render_pos, buffer);
                aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
            }
        }
    }
    aabb
}
