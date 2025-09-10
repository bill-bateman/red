use std::io::{Error, stdout, Write};
use std::ops::Drop;

use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event::Key, Event::Resize, KeyCode::Char, KeyModifiers, KeyCode};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::view::View;

pub struct Editor {
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

        Ok(Editor{view: View::new(filename)?})
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
            self.view.render()?;

            match read()? {
                Key(event) => {
                    if event.kind == KeyEventKind::Press { // check for press events for Windows
                        if event.code == Char('q') && event.modifiers == KeyModifiers::CONTROL {
                            break;
                        }
                        else if event.code == KeyCode::Up { self.view.cursor_up()? }
                        else if event.code == KeyCode::Down { self.view.cursor_down()? }
                        else if event.code == KeyCode::Left { self.view.cursor_left()? }
                        else if event.code == KeyCode::Right { self.view.cursor_right()? }
                        else if event.code == KeyCode::PageUp { self.view.cursor_page_up()? }
                        else if event.code == KeyCode::PageDown { self.view.cursor_page_down()? }
                        else if event.code == KeyCode::Home { self.view.cursor_home()? }
                        else if event.code == KeyCode::End { self.view.cursor_end()? }
                    }
                },
                Resize(width, height) => {
                    self.view.handle_resize(width, height);
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