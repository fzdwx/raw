use crate::app::{App, AppResult};
use crate::event::{flush_resize_events, read, Event, EventHandler};
use crossterm::event::poll;
use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::stdout;
use std::time::Duration;

mod app;
mod event;
mod screen;

fn main() -> AppResult<()> {
    App::default().run()
}