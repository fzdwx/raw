use crate::app::AppResult;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
use crossterm::{
    cursor::position,
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

impl Screen {
    pub fn default() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
        }
    }
    /// refresh buf
    pub fn refresh(&mut self) {
        self.terminal.draw(|frame| {}).unwrap();
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

/// init screen.
pub fn init() -> AppResult<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture, EnterAlternateScreen)?;
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