use anyhow::Context;
use crossterm::{
    cursor::{self},
    style::{Attributes, Color, ResetColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::Stdout;

use super::{block::JoinDir, buffer::RenderBuffer, Block, Image, TextAlign};

#[derive(Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy)]
pub struct Aabb {
    top_left: Point,
    size: Point,
}

pub struct Renderer {
    stdout: Stdout,
    initialized: bool,

    default_fg_color: Color,
    default_bg_color: Color,

    buf1: RenderBuffer,
    buf2: RenderBuffer,
    is_buf1_curr: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            initialized: false,

            default_fg_color: Color::Reset,
            default_bg_color: Color::Reset,

            buf1: RenderBuffer::new(0, 0),
            buf2: RenderBuffer::new(0, 0),
            is_buf1_curr: true,
        }
    }

    pub fn set_default_colors(&mut self, fg_color: Color, bg_color: Color) {
        self.default_fg_color = fg_color;
        self.default_bg_color = bg_color;
    }

    pub fn render(&mut self, block: &Block) -> anyhow::Result<()> {
        // Initialize terminal state
        if !self.initialized {
            self.stdout.execute(EnterAlternateScreen)?;
            terminal::enable_raw_mode()?;
            self.stdout.execute(cursor::Hide)?;
            self.initialized = true;
        }

        let (width, height) = terminal::size()?;
        if width != self.buf1.width() || height != self.buf1.height() {
            self.buf1 = RenderBuffer::new(width, height);
            self.buf2 = RenderBuffer::new(width, height);
            self.is_buf1_curr = true;
        }

        let curr_buf = if self.is_buf1_curr {
            &mut self.buf1
        } else {
            &mut self.buf2
        };

        // Fill with blank, default BG color
        for y in 0..height {
            for x in 0..width {
                curr_buf.at_mut(x, y).fg_color = self.default_fg_color;
                curr_buf.at_mut(x, y).bg_color = self.default_bg_color;
                curr_buf.at_mut(x, y).ch = ' ';
                curr_buf.at_mut(x, y).attrs = Attributes::none();
            }
        }

        // Render block
        self.render_block(block, Point { x: 0, y: 0 });

        let (curr_buf, prev_buf) = if self.is_buf1_curr {
            (&self.buf1, &self.buf2)
        } else {
            (&self.buf2, &self.buf1)
        };

        curr_buf
            .render(prev_buf, &self.stdout)
            .context("Failed to render buffer")?;
        self.is_buf1_curr = !self.is_buf1_curr;

        Ok(())
    }

    fn render_block(&mut self, block: &Block, top_left: Point) -> Aabb {
        match block {
            Block::Image(image) => self.render_image(image, top_left),
            Block::Join { dir, blocks } => self.render_join(dir, blocks, top_left),
        }
    }

    fn render_image(&mut self, image: &Image, top_left: Point) -> Aabb {
        let chars = image.text.chars().collect::<Vec<char>>();

        // Calculate text start based on alignment
        let x: u16 = match image.align {
            TextAlign::Left => 0,
            TextAlign::Center => {
                if (chars.len() as u16) < image.width {
                    ((image.width - chars.len() as u16) / 2)
                } else {
                    0
                }
            }
            TextAlign::Right => {
                let offset = image.width as i32 - chars.len() as i32;
                offset.max(0) as u16
            }
        };

        let fg_color = image.fg_color.unwrap_or(self.default_fg_color);
        let bg_color = image.bg_color.unwrap_or(self.default_bg_color);

        for (i, ch) in image.text.chars().enumerate() {
            let xx = top_left.x + x + i as u16;
            let yy = top_left.y;

            let curr_buf = if self.is_buf1_curr {
                &mut self.buf1
            } else {
                &mut self.buf2
            };
            let cell = curr_buf.at_mut(xx, yy);

            cell.ch = ch;
            cell.fg_color = fg_color;
            cell.bg_color = bg_color;
            cell.attrs = image.attrs;
        }

        Aabb {
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

    fn render_join(&mut self, dir: &JoinDir, blocks: &Vec<Block>, top_left: Point) -> Aabb {
        let mut aabb = Aabb {
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
                    let sub_aabb = self.render_block(b, render_pos);
                    aabb.size.x += sub_aabb.size.x;
                    aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
                }
                JoinDir::Vert => {
                    let render_pos = Point {
                        x: aabb.top_left.x,
                        y: aabb.top_left.y + aabb.size.y,
                    };
                    let sub_aabb = self.render_block(b, render_pos);
                    aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                    aabb.size.y += sub_aabb.size.y;
                }
                JoinDir::Stack => {
                    let render_pos = aabb.top_left;
                    let sub_aabb = self.render_block(b, render_pos);
                    aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                    aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
                }
            }
        }
        aabb
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.stdout
            .execute(ResetColor)
            .unwrap()
            .execute(cursor::Show)
            .unwrap()
            .execute(LeaveAlternateScreen)
            .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
