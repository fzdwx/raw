use crate::app::AppResult;
use crate::event::{read, Event, EventHandler};
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
mod tui;

fn main() -> AppResult<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    let handler = EventHandler::new(250);

    loop {
        match handler.next()? {
            Event::Tick => {
                println!(".\r");
            }
            Event::Key(event) => {
                println!("Event::{:?}\r", event);

                if event == KeyCode::Char('c').into() {
                    println!("Cursor position: {:?}\r", position());
                }

                if event == KeyCode::Char('q').into() {
                    break;
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(x, y) => {
                let (original_size, new_size) = flush_resize_events(Event::Resize(x, y));
                println!("Resize from: {:?}, to: {:?}", original_size, new_size);
            }
        }
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()?;

    Ok(())
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(event: Event) -> ((u16, u16), (u16, u16)) {
    if let Event::Resize(x, y) = event {
        let mut last_resize = (x, y);
        while let Ok(true) = poll(Duration::from_millis(50)) {
            if let Ok(Event::Resize(x, y)) = read() {
                last_resize = (x, y);
            }
        }

        return ((x, y), last_resize);
    }
    ((0, 0), (0, 0))
}