use std::io::{Error, stdout, Write};
use crossterm::{ErrorKind, QueueableCommand};
use crossterm::event::Event;
use crossterm::terminal::{Clear, ClearType};

const Origin: &Position = &Position::origin();

/// the terminal size
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// the terminal
pub struct Terminal {
    size: Size,
}

/// the cursor position
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn origin() -> Position {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl Terminal {
    pub fn default() -> Result<Terminal, ErrorKind> {
        let size = crossterm::terminal::size().unwrap();

        match crossterm::terminal::enable_raw_mode() {
            Ok(_) => {
                Ok(Self {
                    size: Size {
                        width: size.0,
                        height: size.1,
                    }
                })
            }

            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn disable_raw_mode() {
        crossterm::terminal::disable_raw_mode().ok();
    }

    /// clear terminal buffers by type
    pub fn clear_screen(t: ClearType) {
        stdout().queue(Clear(t)).ok();
    }

    /// clear terminal all cells
    pub fn clear_screen_all() {
        Terminal::clear_screen(ClearType::All)
    }

    /// clear terminal current line
    pub fn clear_screen_current_line() {
        Terminal::clear_screen(ClearType::CurrentLine)
    }

    /// flush Terminal buffers
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    /// read event. blocks until get event.
    pub fn read_event() -> Result<Event, ErrorKind> {
        crossterm::event::read()
    }

    /// hide cursor
    pub fn cursor_hide() {
        stdout().queue(crossterm::cursor::Hide).ok();
    }

    /// show cursor
    pub fn cursor_show() {
        stdout().queue(crossterm::cursor::Show).ok();
    }

    /// moves the terminal cursor to the given position (column, row).
    pub fn move_to(p: &Position) {
        stdout().queue(crossterm::cursor::MoveTo(p.x, p.y)).ok();
    }

    /// moves the terminal cursor to the origin.
    pub fn move_to_origin() {
        Terminal::move_to(Origin)
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}