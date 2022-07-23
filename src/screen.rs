use std::io::stdout;

use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;

use crate::app::AppResult;

type Terminal = tui::terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Screen {
    terminal: Terminal,
}

impl Screen {}

/// relative position of the current cursor
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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
    pub fn refresh(&mut self) -> AppResult<()> {
        self.terminal.draw(|frame| {})?;

        Ok(())
    }

    pub fn refresh_and_set_cursor(&mut self, pos: Position) -> AppResult<()> {
        self.terminal
            .draw(|frame| frame.set_cursor(pos.x as u16, pos.y as u16))?;

        Ok(())
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
pub fn position() -> AppResult<(u16, u16)> {
    let (x, y) = crossterm::cursor::position()?;

    Ok((x, y))
}

/// move cursor to position
pub fn move_to(pos: Position) -> AppResult<()> {
    execute!(
        stdout(),
        crossterm::cursor::MoveTo(pos.x as u16, pos.y as u16),
        crossterm::cursor::Show
    )?;

    Ok(())
}

/// init screen.
pub fn init() -> AppResult<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        EnableMouseCapture,
        EnterAlternateScreen,
        crossterm::cursor::EnableBlinking,
        crossterm::cursor::SetCursorShape(crossterm::cursor::CursorShape::Block)
    )?;
    Ok(())
}

/// exit screen.
pub fn exit() -> AppResult<()> {
    execute!(
        stdout(),
        DisableMouseCapture,
        LeaveAlternateScreen,
        Clear(All),
         crossterm::cursor::SetCursorShape(crossterm::cursor::CursorShape::Block)
    )?;

    disable_raw_mode()?;

    Ok(())
}