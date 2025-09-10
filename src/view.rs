use std::cmp::min;
use std::io::{stdout, Error, Write};

use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;

pub struct View {
    buf: Buffer,
    redraw: bool,

    scroll_x: usize,
    scroll_y: usize,

    cx: u16,
    cy: u16,
}

impl View {
    pub fn new(filename: Option<&String>) -> Result<Self, Error> {
        let buf = match filename {
            Some(s) => Buffer::new(s)?,
            None => Buffer::default(),
        };
        Ok(View{
            buf,
            redraw: true,
            scroll_x: 0,
            scroll_y: 0,
            cx: 0,
            cy: 0,
        })
    }

    fn height(&self) -> Result<u16, Error> {
        let (_, height) = terminal::size()?;
        Ok(height)
    }

    fn width(&self) -> Result<u16, Error> {
        let (width, _) = terminal::size()?;
        Ok(width)
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
            if index < self.scroll_y { continue; } // above the screen
            let pos = index - self.scroll_y;
            if pos>=height as usize { break; } // below the screen

            let i1 = self.scroll_x;
            if i1>=row.len() { continue; } // nothing to show
            let i2 = min(i1 + width as usize, row.len());
            queue!(stdout(), cursor::MoveTo(0, pos as u16), style::Print(&row[i1..i2]))?;
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.redraw { return Ok(()); }
        self.redraw = false;

        queue!(stdout(), cursor::Hide)?;
        self.draw_tildes()?;
        self.draw_text()?;
        // reset cursor and flush output
        queue!(stdout(), cursor::Show, cursor::MoveTo(self.cx, self.cy))?;
        stdout().flush()?;
        
        Ok(())
    }

    pub fn handle_resize(&mut self, width: u16, height: u16) {
        if self.cx >= width { 
            self.scroll_x += self.cx as usize-width as usize+1;
            self.cx = width-1;
        }
        if self.cy >= height { 
            self.scroll_y += self.cy as usize-height as usize+1;
            self.cy = height-1; 
        }
    }

    pub fn force_redraw(&mut self) { self.redraw = true; }

    pub fn cursor_up(&mut self) -> Result<(), Error> { 
        if self.cy > 0 { self.cy -= 1; }
        else if self.scroll_y > 0 { self.scroll_y -= 1; }
        self.force_redraw();
        Ok(())
    }
    pub fn cursor_down(&mut self) -> Result<(), Error> { 
        if self.cy < self.height()?-1 { self.cy += 1; }
        else { self.scroll_y += 1; }
        self.force_redraw();
        Ok(())
    }
    pub fn cursor_left(&mut self) -> Result<(), Error> { 
        if self.cx > 0 { self.cx -= 1; }
        else if self.scroll_x > 0 { self.scroll_x -= 1;}
        self.force_redraw();
        Ok(())
    }
    pub fn cursor_right(&mut self) -> Result<(), Error> { 
        if self.cx < self.width()?-1 { self.cx += 1; }
        else { self.scroll_x += 1; }
        self.force_redraw();
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<(), Error> {
        let height = self.height()? as usize;
        if self.scroll_y > height { self.scroll_y -= height; }
        else { self.scroll_y = 0; }
        self.force_redraw();
        Ok(())
    }
    pub fn cursor_page_down(&mut self) -> Result<(), Error> { 
        self.scroll_y += self.height()? as usize;
        self.force_redraw();
        Ok(())
    }

    pub fn cursor_home(&mut self) -> Result<(), Error> { 
        self.cx = 0; 
        self.force_redraw();
        Ok(())
    }
    pub fn cursor_end(&mut self) -> Result<(), Error> { 
        self.cx = self.width()?-1; 
        self.force_redraw();
        Ok(())
    }
}