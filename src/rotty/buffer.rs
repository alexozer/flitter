struct Buffer {
  cells: Vec<Cell>,
  width: u32,
  dummy: Cell,
}

impl Buffer {
  fn new(width: u32, height: u32) -> Self {
    Self {
      cells: vec![Cell; width * height],
      width,
      dummy: Cell::default(),
    }
  }

  fn at(&mut self, x: u32, y: u32) -> &mut Cell {
    let idx = y * width + x;
    if idx >= self.cells.len() {
      &mut self.dummy
    } else {
      &mut self.cells[idx]
    }
  }
}

struct Cell {
  ch: char,
  fg_color: Color,
  bg_color: Color,
  attrs: Attributes,
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

