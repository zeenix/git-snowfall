use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
    terminal::size,
};

#[derive(Debug, Clone, Copy)]
pub struct Flake {
    x: u16,
    y: f32,
    size: Size,
}

impl Flake {
    pub fn new() -> Result<Self, std::io::Error> {
        let screen_width = size()?.0;

        Ok(Self {
            x: fastrand::u16(0..screen_width),
            y: 0.0,
            size: Size::new(),
        })
    }

    // Update the position of the flake and return true if it's still on the screen.
    pub fn update(&mut self) -> bool {
        self.y += self.size.speed();

        (self.y as u16) < size().unwrap().1
    }

    pub fn draw(&self, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
        let content = match self.size {
            Size::Small => '❄'.white().dim(),
            Size::Medium => '❄'.white(),
            Size::Large => '❄'.white().bold(),
        };

        let _ = stdout
            .queue(cursor::MoveTo(self.x, self.y as u16))?
            .queue(style::PrintStyledContent(content))?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    fn new() -> Self {
        match fastrand::u8(1..=3) {
            1 => Size::Small,
            2 => Size::Medium,
            3 => Size::Large,
            _ => unreachable!(),
        }
    }

    fn speed(&self) -> f32 {
        match self {
            Size::Small => 0.5,
            Size::Medium => 1.0,
            Size::Large => 1.5,
        }
    }
}
