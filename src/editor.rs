use std::io::{Error, stdout};

use crossterm::event::{read, Event::Key, KeyCode::Char, KeyModifiers};
use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor { }

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    fn setup(&self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        self.draw_tildes()?;
        Ok(())
    }

    fn teardown(&self) -> Result<(), Error> {
        self.clear_screen()?;
        print!("Goodbye.\n\r");
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen(&self) -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn draw_tildes(&self) -> Result<(), Error> {
        let (_width, height) = terminal::size()?;
        for row in 0..height {
            execute!(stdout(), cursor::MoveTo(1, row))?;
            print!("~");
        }
        Ok(())
    }

    fn run(&self) -> Result<(), Error> {
        loop {
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
