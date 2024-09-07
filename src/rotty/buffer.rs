use anyhow::Context;
use crossterm::{
    cursor,
    style::{
        Attribute, Attributes, Color, Print, SetAttribute, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    QueueableCommand,
};

pub struct RenderBuffer {
    cells: Vec<Cell>,
    width: u16,
    dummy: Cell,
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: vec![Cell::default(); (width * height) as usize],
            width,
            dummy: Cell::default(),
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.cells.len() as u16 / self.width
    }

    pub fn at_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let idx = (y * self.width + x) as usize;
        if idx >= self.cells.len() {
            &mut self.dummy
        } else {
            &mut self.cells[idx]
        }
    }

    pub fn at_ref(&self, x: u16, y: u16) -> &Cell {
        let idx = (y * self.width + x) as usize;
        if idx >= self.cells.len() {
            &self.dummy
        } else {
            &self.cells[idx]
        }
    }

    pub fn render<T>(&self, prev: &RenderBuffer, mut out: T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let mut cursor_x = 0u16;
        let mut cursor_y = 0u16;
        let mut last_fg_color = Color::Reset;
        let mut last_bg_color = Color::Reset;
        let mut last_attrs = Attributes::none();
        out.queue(SetAttribute(Attribute::Reset))?
            .queue(cursor::MoveTo(cursor_x, cursor_y))?;

        for y in 0..(self.cells.len() / self.width as usize) {
            for x in 0..self.width {
                let curr_cell = self.at_ref(x, y as u16);
                let prev_cell = prev.at_ref(x, y as u16);
                if curr_cell == prev_cell {
                    continue;
                }

                let y = y as u16;

                if x != cursor_x || y != cursor_y {
                    out.queue(cursor::MoveTo(x, y))?;
                    cursor_x = x + 1; // Printing will advance cursor
                    cursor_y = y;
                }
                if curr_cell.attrs != last_attrs {
                    out.queue(SetAttribute(Attribute::Reset))?
                        .queue(SetAttributes(curr_cell.attrs))?;
                    last_attrs = curr_cell.attrs;
                    last_fg_color = Color::Reset;
                    last_bg_color = Color::Reset;
                }
                if curr_cell.fg_color != last_fg_color {
                    out.queue(SetForegroundColor(curr_cell.fg_color))?;
                    last_fg_color = curr_cell.fg_color;
                }
                if curr_cell.bg_color != last_bg_color {
                    out.queue(SetBackgroundColor(curr_cell.bg_color))?;
                    last_bg_color = curr_cell.bg_color;
                }
                out.queue(Print(curr_cell.ch))?;
            }
        }

        out.flush().context("Failed to flush to terminal")?;

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub attrs: Attributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg_color: Color::White,
            bg_color: Color::Black,
            attrs: Attributes::none(),
        }
    }
}
