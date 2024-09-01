use anyhow::anyhow;
use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue, style,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

const MENU: &str = r#"Crossterm interactive test

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests:

1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)
4. input
5. synchronized output

Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

pub struct Renderer {
    stdout: Stdout,
    initialized: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            initialized: false,
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.initialized {
            self.stdout.execute(EnterAlternateScreen)?;
            terminal::enable_raw_mode()?;
            self.initialized = true;
        }
        self.stdout
            .execute(style::ResetColor)?
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::MoveTo(1, 1))?;

        for line in MENU.split("\n") {
            self.stdout
                .queue(style::Print(line))?
                .queue(cursor::MoveToNextLine(1))?;
        }

        self.stdout.flush()?;

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        execute!(
            self.stdout,
            style::ResetColor,
            cursor::Show,
            LeaveAlternateScreen,
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
