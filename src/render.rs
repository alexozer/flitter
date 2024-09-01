use anyhow::anyhow;
use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue, style,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
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
            execute!(self.stdout, EnterAlternateScreen)?;
            terminal::enable_raw_mode()?;
        }
        queue!(
            self.stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1),
        )?;

        for line in MENU.split("\n") {
            queue!(self.stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        self.stdout.flush()?;

        match read_char()? {
            // '1' => test::cursor::run(w)?,
            // '2' => test::color::run(w)?,
            // '3' => test::attribute::run(w)?,
            // '4' => test::event::run(w)?,
            // '5' => test::synchronized_output::run(w)?,
            'q' => {
                execute!(self.stdout, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                return Err(anyhow!("Exiting loop"));
            }
            _ => {}
        };

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
