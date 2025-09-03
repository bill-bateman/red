use std::io::{Error, stdout};

use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;

pub struct View {
    buf: Buffer,
}

impl View {
    pub fn new(filename: Option<&String>) -> Self {
        let buf = match filename {
            Some(s) => Buffer::from_file(s).unwrap(), // TODO: handle error better
            None => Buffer::default(),
        };
        View{ buf }
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
        if self.buf.is_empty() {
            self.draw_version()?;
            return Ok(());
        }

        for (index, row) in self.buf.text.iter().enumerate() {
            queue!(stdout(), cursor::MoveTo(0, index as u16), style::Print(row))?;
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        queue!(stdout(), cursor::Hide)?;
        self.draw_tildes()?;
        self.draw_text()?;
        Ok(())
    }
}