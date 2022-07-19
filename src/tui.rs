use crate::buffer::banner::Banner;
use crate::buffer::{Buffer, Buffered};
use crossterm::cursor::position;
use crossterm::event::{read, Event};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use std::io::{stdout, Stdout};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::terminal::CompletedFrame;
use tui::widgets::Widget;
use tui::Frame;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    size: Rect,
    internal_terminal: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new() -> Self {
        let internal_terminal = Tui::new_internal_terminal();
        let size = internal_terminal.size().unwrap();

        let mut terminal = Self {
            size,
            internal_terminal,
        };

        terminal.prepare().expect("tui init fail");
        terminal
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<CrosstermBackend<Stdout>>),
    {
        self.internal_terminal.draw(f)
    }

    /// read event
    pub fn read(&self) -> crossterm::Result<Event> {
        read()
    }

    /// move cursor to (x,y)
    pub fn move_to(x: u16, y: u16) {
        stdout().execute(crossterm::cursor::MoveTo(x, y)).ok();
    }

    /// get size
    pub fn size() -> crossterm::Result<(u16, u16)> {
        crossterm::terminal::size()
    }

    /// get position
    pub fn position() -> crossterm::Result<(u16, u16)> {
        position()
    }

    /// resize terminal.
    pub fn resize(&mut self) {
        self.internal_terminal
            .autoresize()
            .expect("terminal resize error");
        self.size = self
            .internal_terminal
            .size()
            .expect("terminal get size error");
    }

    /// clear all buffers.
    pub fn clear_all(&mut self) -> io::Result<()> {
        self.internal_terminal.clear()
    }

    /// show cursor
    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.internal_terminal.show_cursor()
    }

    /// hide cursor
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.internal_terminal.hide_cursor()
    }

    /// new internal terminal.
    fn new_internal_terminal() -> tui::Terminal<CrosstermBackend<Stdout>> {
        let backend = CrosstermBackend::new(stdout());
        tui::Terminal::new(backend).unwrap()
    }

    fn prepare(&mut self) -> io::Result<()> {
        enable_raw_mode().unwrap();
        execute!(
            self.internal_terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        self.internal_terminal.hide_cursor()?;
        self.internal_terminal.clear()?;
        Ok(())
    }

    pub fn destroy(&mut self) -> io::Result<()> {
        disable_raw_mode().unwrap();
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.internal_terminal.show_cursor()?;
        Ok(())
    }
}

impl Default for Tui {
    fn default() -> Self {
        let internal_terminal = Tui::new_internal_terminal();
        let size = internal_terminal.size().unwrap();

        let mut terminal = Self {
            size,
            internal_terminal,
        };

        terminal.prepare().expect("tui init fail");
        terminal
    }
}