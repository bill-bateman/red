use std::io::{Error, stdout, Write};

use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyModifiers, KeyCode};
use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use crate::view::View;

pub struct Editor {
    cx: u16,
    cy: u16,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Editor{cx:0, cy:0, view:View::default()}
    }

    fn setup(&self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        Ok(())
    }

    fn teardown(&self) -> Result<(), Error> {
        self.clear_screen()?;
        queue!(stdout(), cursor::MoveTo(0, 0), style::Print("Goodbye.\n\r"))?;
        stdout().flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen(&self) -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            let (width, height) = terminal::size()?;
            
            self.view.render()?;
            // reset cursor and flush output
            queue!(stdout(), cursor::Show, cursor::MoveTo(self.cx, self.cy))?;
            stdout().flush()?;

            if let Key(event) = read()? && event.kind == KeyEventKind::Press {
                // check for press events for Windows

                if event.code == Char('q') && event.modifiers == KeyModifiers::CONTROL {
                    break;
                }
                else if event.code == KeyCode::Up && self.cy > 0 { self.cy -= 1; }
                else if event.code == KeyCode::Down && self.cy < height-1 { self.cy += 1; }
                else if event.code == KeyCode::Left && self.cx > 0 { self.cx -= 1; }
                else if event.code == KeyCode::Right && self.cx < width-1 { self.cx += 1; }
                else if event.code == KeyCode::PageUp { self.cy = 0; }
                else if event.code == KeyCode::PageDown { self.cy = height-1; }
                else if event.code == KeyCode::Home { self.cx = 0; }
                else if event.code == KeyCode::End { self.cx = width-1; }
            }
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;
        let res = self.run();
        self.teardown()?;
        res
    }
}
