use std::{
    io::{Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor, execute,
    style::{self, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, size},
};

#[derive(Debug)]
pub struct Animation {}

impl Animation {
    pub fn new() -> Self {
        Animation {}
    }

    pub fn run(self, keep_going: Arc<AtomicBool>) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        while keep_going.load(Ordering::Relaxed) {
            let (center_x, center_y) = size().map(|(x, y)| (x / 2, y / 2))?;
            let _ = stdout
                .queue(cursor::MoveTo(center_x, center_y))?
                .queue(style::PrintStyledContent("█".magenta()))?;

            // some other code ...

            let _ = stdout.flush()?;

            sleep(Duration::from_millis(100));
        }

        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }
}
