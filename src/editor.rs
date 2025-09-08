use std::io::{Error, stdout, Write};
use std::ops::Drop;

use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event::Key, Event::Resize, KeyCode::Char, KeyModifiers, KeyCode};
use crossterm::{queue, cursor, terminal};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::view::View;

pub struct Editor {
    cx: u16,
    cy: u16,
    view: View,
}

impl Editor {
    pub fn new(filename: Option<&String>) -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Editor::teardown();
            current_hook(panic_info);
        }));
        Editor::setup()?;

        Ok(Editor{cx: 0, cy: 0, view: View::new(filename)?})
    }

    fn setup() -> Result<(), Error> {
        queue!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Editor::clear_screen()?;
        Ok(())
    }

    fn teardown() -> Result<(), Error> {
        Editor::clear_screen()?;
        queue!(stdout(), LeaveAlternateScreen)?;
        stdout().flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let (width, height) = terminal::size()?;
            
            self.view.render()?;
            // reset cursor and flush output
            queue!(stdout(), cursor::Show, cursor::MoveTo(self.cx, self.cy))?;
            stdout().flush()?;

            match read()? {
                Key(event) => {
                    if event.kind == KeyEventKind::Press { // check for press events for Windows
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
                },
                Resize(_, _) => {
                    self.view.force_redraw();
                },
                _ => {}
            }
        }
        Ok(())
    }

}

impl Drop for Editor {
    fn drop(&mut self) {
        Editor::teardown().unwrap();
    }
}