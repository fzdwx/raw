use std::error;
use std::error::Error;
use std::ops::Deref;

use crossterm::cursor::position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::event::{flush_resize_events, Event, EventHandler};
use crate::screen;

/// global result.
pub type AppResult<T> = Result<T, anyhow::Error>;

/// the application.
pub struct App {
    running: bool,
    events: EventHandler,
}

impl App {
    /// app's constructor
    ///
    /// # Arguments
    ///
    /// * `tick_rate`: The trigger interval of the tick event
    ///
    /// returns: App
    ///
    /// # Examples
    ///
    /// ```
    ///use raw::app::{App, AppResult};
    ///
    ///fn main() -> AppResult<()> {
    ///    App::default().run()
    ///}
    /// ```
    pub fn new(tick_rate: u64) -> App {
        screen::init().expect("tui init fail");
        Self {
            running: true,
            events: EventHandler::new(tick_rate),
        }
    }

    /// run app
    pub fn run(&mut self) -> AppResult<()> {
        while self.running {
            if let Err(err) = self.dispatch_events() {
                exit_with_err(err)
            }
        }

        screen::exit()
    }

    /// dispatch events.
    fn dispatch_events(&mut self) -> AppResult<()> {
        match self.events.next()? {
            Event::Tick => {
                println!(".\r");
            }
            Event::Key(event) => {
                println!("Event::{:?}\r", event);

                if event == KeyCode::Char('c').into() {
                    println!("Cursor position: {:?}\r", position());
                }

                if event == KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL) {
                    self.running = !self.running;
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(x, y) => {
                let (original_size, new_size) = flush_resize_events(Event::Resize(x, y));
                println!("Resize from: {:?}, to: {:?}", original_size, new_size);
            }
        }

        Ok(())
    }
}

fn exit_with_err(err: anyhow::Error) {
    screen::exit().unwrap();
    panic!("{}", err)
}

impl Default for App {
    fn default() -> Self {
        App::new(250)
    }
}