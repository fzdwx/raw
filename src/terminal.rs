use crossterm::event::{read, Event};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::io::{stdout, Stdout};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::terminal::CompletedFrame;
use tui::Frame;

pub struct Terminal {
    size: Rect,
    internal_terminal: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub(crate) fn clear_all(&mut self) -> io::Result<()> {
        self.internal_terminal.clear()
    }
}

impl Terminal {
    pub fn new() -> Self {
        let internal_terminal = Terminal::new_internal_terminal();
        let size = internal_terminal.size().unwrap();

        let mut terminal = Self {
            size,
            internal_terminal,
        };

        terminal.prepare();
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
    pub fn move_to(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.internal_terminal.set_cursor(x, y)
    }

    /// show cursor
    fn show_cursor(&mut self) -> io::Result<()> {
        self.internal_terminal.show_cursor()
    }

    /// hide cursor
    fn hide_cursor(&mut self) -> io::Result<()> {
        self.internal_terminal.hide_cursor()
    }

    /// new internal terminal.
    fn new_internal_terminal() -> tui::Terminal<CrosstermBackend<Stdout>> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = tui::Terminal::new(backend).unwrap();
        terminal
    }

    fn prepare(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            self.internal_terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )
        .unwrap();
    }

    pub fn destroy(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.internal_terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
    }
}