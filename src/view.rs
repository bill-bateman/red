use std::io::{Error, stdout};

use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{Clear, ClearType};

pub struct View {

}

impl View {
    pub fn default() -> Self {
        View{}
    }

    fn draw_version(&self) -> Result<(), Error> {
        let (width, height) = terminal::size()?;

        let version = "red v0.1";
        let x = if version.len() as u16 > width { 0 } else { width / 2 - version.len() as u16 / 2};
        let y = height / 2;

        queue!(stdout(), cursor::MoveTo(x, y), style::Print(version))?;
        Ok(())
    }

    fn draw_tildes(&self) -> Result<(), Error> {
        let (_width, height) = terminal::size()?;

        queue!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
        for row in 0..=height {
            queue!(stdout(), cursor::MoveTo(0, row), style::Print("~"))?;
        }
        Ok(())
    }

    fn draw_text(&self) -> Result<(), Error> {
        queue!(stdout(), cursor::MoveTo(0, 0), style::Print("Hello, world!"))
    }

    pub fn render(&self) -> Result<(), Error> {
        queue!(stdout(), cursor::Hide)?;
        self.draw_tildes()?;
        self.draw_version()?;
        self.draw_text()?;
        Ok(())
    }
}