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