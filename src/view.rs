use std::io::{Error, stdout};

use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;

pub struct View {
    buf: Buffer,
    redraw: bool,
}

impl View {
    pub fn new(filename: Option<&String>) -> Self {
        let buf = match filename {
            Some(s) => Buffer::from_file(s).unwrap(), // TODO: handle error better
            None => Buffer::default(),
        };
        View{ buf, redraw: true }
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

        let (width, height) = terminal::size()?;
        for (index, row) in self.buf.text.iter().enumerate() {
            queue!(stdout(), cursor::MoveTo(0, index as u16), style::Print(row.get(..width as usize).unwrap()))?;
            queue!(stdout(), cursor::MoveTo(0, index as u16), style::Print(width), style::Print(height))?;
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.redraw { return Ok(()); }
        self.redraw = false;

        queue!(stdout(), cursor::Hide)?;
        self.draw_tildes()?;
        self.draw_text()?;
        Ok(())
    }

    pub fn force_redraw(&mut self) { self.redraw = true; }
}