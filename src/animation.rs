use std::{
    io::{Write, stdout},
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor,
    event::{self, Event, KeyCode, poll},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::flake::Flake;

#[derive(Debug)]
pub struct Animation {
    flakes: Vec<Flake>,
}

impl Animation {
    pub fn new() -> Self {
        Animation { flakes: Vec::new() }
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, cursor::Hide)?;

        loop {
            stdout.queue(terminal::Clear(terminal::ClearType::All))?;

            let mut len = self.flakes.len();
            let mut i = 0;
            loop {
                if i >= len {
                    break;
                }

                let flake = &mut self.flakes[i];
                if !flake.update() {
                    self.flakes.remove(i);
                    len -= 1;

                    continue;
                }

                flake.draw(&mut stdout)?;

                i += 1;
            }

            let _ = stdout.flush()?;

            if poll(Duration::from_millis(33))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
            }

            let n_new_flakes = fastrand::u32(1..=5);
            for _ in 0..n_new_flakes {
                self.flakes.push(Flake::new()?);
            }
        }

        execute!(stdout, cursor::Show)?;
        execute!(stdout, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}
