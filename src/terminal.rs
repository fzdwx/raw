use std::io::{stdout, Error, Stdout};

use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::event::Event;
use crossterm::style::Color;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{style, Command, ErrorKind, ExecutableCommand, QueueableCommand};
use tui::backend::CrosstermBackend;

const ORIGIN: Position = Position { x: 0, y: 0 };

/// the terminal size
#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// the terminal
pub struct Terminal {
    size: Size,
    pub tui_terminal: tui::Terminal<CrosstermBackend<Stdout>>,
}

/// the cursor position
#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn new(title: &str) -> Result<Terminal, ErrorKind> {
        let size = crossterm::terminal::size().unwrap();
        crossterm::terminal::enable_raw_mode()?;

        match tui::Terminal::new(CrosstermBackend::new(stdout())) {
            Ok(t) => {
                let mut t = Self {
                    tui_terminal: t,
                    size: Size {
                        width: size.0,
                        height: size.1.saturating_sub(2),
                    },
                };

                t.refresh_title(title);

                Ok(t)
            }

            Err(err) => Err(err),
        }
    }

    /// add command to execute queue.
    pub fn queue(&self, command: impl Command) -> Result<(), ErrorKind> {
        match stdout().queue(command) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// execute command.
    pub fn execute(&self, command: impl Command) -> Result<(), ErrorKind> {
        match stdout().execute(command) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// flush Terminal buffers
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.tui_terminal.flush()
    }

    /// refresh title.
    pub fn refresh_title(&mut self, title: &str) {
        self.execute(crossterm::terminal::SetTitle(title)).ok();
    }

    /// disable raw mode
    pub fn disable_raw_mode(&self) -> crossterm::Result<()> {
        crossterm::terminal::disable_raw_mode()
    }

    /// get current cursor position
    pub fn position(&mut self) {
        self.tui_terminal.get_cursor().ok();
    }

    /// resize terminal
    pub fn resize(&mut self, width: u16, height: u16) {
        self.size = Size { width, height };
        self.tui_terminal.autoresize().ok();
    }

    /// read event. blocks until get event.
    pub fn read_event(&self) -> Result<Event, Error> {
        crossterm::event::read()
    }

    /// set terminal background color
    pub fn set_bg_color(&self, color: Color) {
        self.queue(style::SetBackgroundColor(color)).ok();
    }

    /// reset terminal foreground color
    pub fn reset_bg_color(&self) {
        self.set_bg_color(Color::Reset);
    }

    /// set terminal foreground color
    pub fn set_fg_color(&self, color: Color) {
        self.queue(style::SetForegroundColor(color)).ok();
    }

    /// reset terminal background color
    pub fn reset_fg_color(&self) {
        self.set_fg_color(Color::Reset);
    }

    /// clear terminal buffers by type
    pub fn clear_screen(&self, t: ClearType) {
        self.queue(Clear(t)).ok();
    }

    /// clear terminal all cells
    pub fn clear_screen_all(&self) {
        self.clear_screen(ClearType::All);
    }

    /// clear terminal current line
    pub fn clear_screen_current_line(&self) {
        self.clear_screen(ClearType::CurrentLine);
    }

    /// hide cursor
    pub fn hide_cursor(&mut self) {
        self.tui_terminal.hide_cursor().ok();
    }

    /// show cursor
    pub fn show_cursor(&mut self) {
        self.tui_terminal.show_cursor().ok();
    }

    /// moves the terminal cursor to the given position (column, row).
    pub fn move_to(&self, p: &Position) {
        self.queue(MoveTo(p.x as u16, p.y as u16)).ok();
    }

    ///  moves the terminal cursor to the given column on the current row.
    pub fn move_to_column(&self, col: u16) {
        self.queue(MoveToColumn(col)).ok();
    }

    /// moves the terminal cursor to the origin.
    pub fn move_to_origin(&self) {
        self.move_to(&ORIGIN);
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}