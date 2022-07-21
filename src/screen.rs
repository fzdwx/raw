use crate::app::AppResult;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::stdout;
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;

type Terminal = tui::terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Screen {
    terminal: Terminal,
}

/// the actual position of the current cursor
#[derive(Copy, Clone, Default)]
pub struct Offset {
    x: usize,
    y: usize,
}

/// relative position of the current cursor
#[derive(Copy, Clone, Default)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
        }
    }
}

impl Screen {
    /// refresh screen
    pub fn refresh(&mut self) {
        self.terminal.draw(|frame| {}).unwrap();
    }

    pub fn refresh_and_move_to(&mut self, pos: (u16, u16)) {
        self.terminal
            .draw(|frame| frame.set_cursor(pos.0, pos.1))
            .unwrap();
    }

    pub fn refresh_and_move_t_origin(&mut self) {
        self.refresh_and_move_to((0, 0));
    }

    /// get current buf
    pub fn get_buf(&mut self) -> &mut Buffer {
        self.terminal.current_buffer_mut()
    }
}

/// get screen size.
pub fn size() -> AppResult<(u16, u16)> {
    let size = crossterm::terminal::size()?;
    Ok(size)
}

/// get current cursor position
pub fn position() -> AppResult<Position> {
    let (x, y) = crossterm::cursor::position()?;

    Ok(Position { x, y })
}

/// init screen.
pub fn init() -> AppResult<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        EnableMouseCapture,
        EnterAlternateScreen,
        crossterm::cursor::EnableBlinking
    )?;
    Ok(())
}

/// exit screen.
pub fn exit() -> AppResult<()> {
    execute!(
        stdout(),
        DisableMouseCapture,
        LeaveAlternateScreen,
        Clear(All)
    )?;

    disable_raw_mode()?;

    Ok(())
}