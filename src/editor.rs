use std::io::{Error, stdout, Write};

use crossterm::event::{read, Event::Key, KeyCode::Char, KeyModifiers};
use crossterm::{queue, cursor, terminal, style};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor { }

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    fn setup(&self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        Ok(())
    }

    fn teardown(&self) -> Result<(), Error> {
        self.clear_screen()?;
        stdout().flush()?;
        print!("Goodbye.\n\r");
        disable_raw_mode()?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        queue!(stdout(), cursor::Hide)?;
        self.draw_tildes()?;
        self.draw_version()?;
        queue!(stdout(), cursor::Show)?;
        stdout().flush()?;
        Ok(())
    }

    fn clear_screen(&self) -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    fn draw_tildes(&self) -> Result<(), Error> {
        let (_width, height) = terminal::size()?;

        queue!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
        for row in 0..=height {
            queue!(stdout(), cursor::MoveTo(0, row), style::Print("~"))?;
        }
        Ok(())
    }

    fn draw_version(&self) -> Result<(), Error> {
        let (width, height) = terminal::size()?;

        let version = "red v0.1";
        let x = if version.len() as u16 > width { 0 } else { width / 2 - version.len() as u16 / 2};
        let y = height / 2;

        queue!(stdout(), cursor::MoveTo(x, y), style::Print(version))?;
        Ok(())
    }

    fn run(&self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if let Key(event) = read()? {
                if event.code == Char('q') && event.modifiers == KeyModifiers::CONTROL {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn start(&self) -> Result<(), Error> {
        self.setup()?;
        let res = self.run();
        self.teardown()?;
        res
    }
}
