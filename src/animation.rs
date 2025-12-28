use std::{
    io::{Write, stdout},
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor,
    event::{self, Event, KeyCode, poll},
    execute,
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, size},
};

#[derive(Debug)]
pub struct Animation {}

impl Animation {
    pub fn new() -> Self {
        Animation {}
    }

    pub fn run(self) -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, cursor::Hide)?;

        let (center_x, center_y) = size().map(|(x, y)| (x / 2, y / 2))?;

        let mut i = 0;
        loop {
            let (s, pos_x, pos_y) = if i % 2 == 0 {
                ("x".magenta(), center_x / 2, center_y)
            } else {
                ("Ж".cyan(), center_x, center_y)
            };
            i += 1;

            let _ = stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(pos_x, pos_y))?
                .queue(style::PrintStyledContent(s))?;

            // some other code ...

            let _ = stdout.flush()?;

            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        execute!(stdout, cursor::Show)?;
        execute!(stdout, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}
