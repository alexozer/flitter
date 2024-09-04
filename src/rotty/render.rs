use anyhow::Context;
use crossterm::{
    cursor::{self},
    style::{
        Attribute, Attributes, Color, Colors, Print, ResetColor, SetAttribute, SetAttributes,
        SetBackgroundColor, SetColors, SetForegroundColor,
    },
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

use super::{block::JoinDir, Block, Image, TextAlign};

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy)]
pub struct AABB {
    top_left: Point,
    size: Point,
}

pub struct Renderer {
    stdout: Stdout,
    initialized: bool,

    default_fg_color: Color,
    default_bg_color: Color,

    last_fg_color: Color,
    last_bg_color: Color,
    last_attrs: Attributes,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            initialized: false,
            default_fg_color: Color::Reset,
            default_bg_color: Color::Reset,
            last_fg_color: Color::Reset,
            last_bg_color: Color::Reset,
            last_attrs: Attributes::none(),
        }
    }

    pub fn set_default_colors(&mut self, fg_color: Color, bg_color: Color) {
        self.default_fg_color = fg_color;
        self.default_bg_color = bg_color;
    }

    pub fn render(&mut self, block: &Block) -> anyhow::Result<()> {
        if !self.initialized {
            self.stdout.execute(EnterAlternateScreen)?;
            terminal::enable_raw_mode()?;
            self.stdout.execute(cursor::Hide)?;
            self.initialized = true;
        }
        self.stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(SetColors(Colors {
                foreground: Some(self.default_fg_color),
                background: Some(self.default_bg_color),
            }))?
            .queue(SetAttributes(Attributes::none()))?;
        self.last_fg_color = self.default_fg_color;
        self.last_bg_color = self.default_bg_color;
        self.last_attrs = Attributes::none();

        self.render_block(block, Point { x: 0, y: 0 })
            .context("Failed to render block")?;

        self.stdout.flush().context("Failed to flush to terminal")?;

        Ok(())
    }

    fn render_block(&mut self, block: &Block, top_left: Point) -> anyhow::Result<AABB> {
        match block {
            Block::Image(image) => self.render_image(image, top_left),
            Block::Join { dir, blocks } => self.render_join(dir, blocks, top_left),
        }
    }

    fn render_image(&mut self, image: &Image, top_left: Point) -> anyhow::Result<AABB> {
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

        let fg_color = image.fg_color.unwrap_or(self.default_fg_color);
        let bg_color = image.bg_color.unwrap_or(self.default_bg_color);

        if fg_color != self.last_fg_color
            || bg_color != self.last_bg_color
            || image.attrs != self.last_attrs
        {
            // Set all of these together, because Attribute::Reset also resets colors
            self.stdout
                .queue(SetAttribute(Attribute::Reset))?
                .queue(SetForegroundColor(fg_color))?
                .queue(SetBackgroundColor(bg_color))?
                .queue(SetAttributes(image.attrs))?;
            self.last_fg_color = fg_color;
            self.last_bg_color = bg_color;
            self.last_attrs = image.attrs;
        }

        self.stdout
            .queue(cursor::MoveTo((top_left.x + x) as u16, top_left.y as u16))?
            // TODO handle too-long text
            .queue(Print(&image.text))?;

        Ok(AABB {
            top_left: Point {
                x: top_left.x,
                y: top_left.y,
            },
            size: Point {
                x: image.width,
                y: 1,
            },
        })
    }

    fn render_join(
        &mut self,
        dir: &JoinDir,
        blocks: &Vec<Block>,
        top_left: Point,
    ) -> anyhow::Result<AABB> {
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
                    let sub_aabb = self.render_block(b, render_pos)?;
                    aabb.size.x += sub_aabb.size.x;
                    aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
                }
                JoinDir::Vert => {
                    let render_pos = Point {
                        x: aabb.top_left.x,
                        y: aabb.top_left.y + aabb.size.y,
                    };
                    let sub_aabb = self.render_block(b, render_pos)?;
                    aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                    aabb.size.y += sub_aabb.size.y;
                }
                JoinDir::Stack => {
                    let render_pos = aabb.top_left;
                    let sub_aabb = self.render_block(b, render_pos)?;
                    aabb.size.x = aabb.size.x.max(sub_aabb.size.x);
                    aabb.size.y = aabb.size.y.max(sub_aabb.size.y);
                }
            }
        }
        Ok(aabb)
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
