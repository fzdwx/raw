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

// todo need fix

const GLOBAL: Terminal = Terminal::default();

impl Default for Terminal {
    fn default() -> Self {
        let internal_terminal = Terminal::new_internal_terminal();
        let size = internal_terminal.size().unwrap();

        let mut terminal = Self {
            size,
            internal_terminal,
        };

        terminal.prepare();
        terminal
    }
}

impl Terminal {
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

    /// get
    pub fn size(&self) -> Rect {
        self.size
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