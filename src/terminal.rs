use std::io::{stdout, Error, Write};

use crossterm::event::Event;
use crossterm::style::Color;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{style, ErrorKind, QueueableCommand};

const ORIGIN: Position = Position { x: 0, y: 0 };

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
#[derive(Default)]
pub struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Terminal {
    pub fn default() -> Result<Terminal, ErrorKind> {
        let size = crossterm::terminal::size().unwrap();

        match crossterm::terminal::enable_raw_mode() {
            Ok(_) => Ok(Self {
                size: Size {
                    width: size.0,
                    height: size.1.saturating_sub(2),
                },
            }),

            Err(err) => Err(err),
        }
    }

    /// set terminal background color
    pub fn set_bg_color(color: Color) {
        stdout().queue(style::SetBackgroundColor(color)).ok();
    }

    /// reset terminal foreground color
    pub fn reset_bg_color() {
        Terminal::set_bg_color(Color::Reset)
    }

    /// set terminal foreground color
    pub fn set_fg_color(color: Color) {
        stdout().queue(style::SetForegroundColor(color)).ok();
    }

    /// reset terminal background color
    pub fn reset_fg_color() {
        Terminal::set_fg_color(Color::Reset)
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
        stdout()
            .queue(crossterm::cursor::MoveTo(p.x as u16, p.y as u16))
            .ok();
    }

    /// moves the terminal cursor to the origin.
    pub fn move_to_origin() {
        Terminal::move_to(&ORIGIN)
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}