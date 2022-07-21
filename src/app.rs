use std::error;
use std::error::Error;
use std::ops::ControlFlow::Continue;
use std::ops::Deref;

use crate::args::Args;
use crossterm::cursor::position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::event::{flush_resize_events, Event, EventHandler};
use crate::render::banner::Banner;
use crate::render::switcher::DocumentSwitcher;
use crate::render::Render;
use crate::screen;
use crate::screen::{Offset, Position, Screen};

/// global result.
pub type AppResult<T> = Result<T, anyhow::Error>;

/// the application.
pub struct App {
    // app is running?
    running: bool,
    // events reader
    events: EventHandler,
    // the screen
    screen: Screen,
    // the banner render
    banner: Banner,
    // document container.
    doc_switcher: DocumentSwitcher,
    actual: Offset,
    relative: Position,
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
        let mut doc_switcher = DocumentSwitcher::default();

        let args = Args::load();
        doc_switcher.load(args.filenames);

        Self {
            running: true,
            events: EventHandler::new(tick_rate),
            screen: Default::default(),
            banner: Default::default(),
            doc_switcher,
            actual: Default::default(),
            relative: Default::default(),
        }
    }

    /// run app
    pub fn run(&mut self) -> AppResult<()> {
        while self.running {
            self.draw_some();

            if let Err(err) = self.dispatch_events() {
                exit_with_err(err)
            }
        }

        self.exit()
    }

    /// dispatch events.
    fn dispatch_events(&mut self) -> AppResult<()> {
        match self.events.next()? {
            Event::Tick => {
                // println!(".\r");
            }
            Event::Key(event) => {
                self.on_keypress(event);
            }
            Event::Mouse(_) => {}
            Event::Resize(x, y) => {
                // let (original_size, new_size) = flush_resize_events(Event::Resize(x, y));
                // if original_size != new_size {
                //     println!("Resize from: {:?}, to: {:?}", original_size, new_size);
                // };
            }
        }

        Ok(())
    }

    /// draw ui.
    fn draw_some(&mut self) {
        let buf = self.screen.get_buf();
        if self.doc_switcher.is_empty() {
            self.banner.render(buf, buf.area);
            self.screen.refresh();
            return;
        }

        self.doc_switcher.render(buf, buf.area);
        self.screen.refresh_and_move_t_origin();
    }

    /// on key press
    fn on_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.running = false;
            }

            // (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
            //     self.show_banner = !self.show_banner;
            // }

            // move cursor
            // (KeyCode::Up, _)
            // | (KeyCode::Down, _)
            // | (KeyCode::Left, _)
            // | (KeyCode::Right, _)
            // | (KeyCode::PageUp, _)
            // | (KeyCode::PageDown, _)
            // | (KeyCode::End, _)
            // | (KeyCode::Home, _) => self.move_cursor(key.code),
            (KeyCode::Left, modifier) => {
                // switch buffer
                if modifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.doc_switcher.prev();
                }
            }

            (KeyCode::Right, keyModifier) => {
                // switch buffer
                if keyModifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.doc_switcher.next();
                }
            }

            _ => {}
        };
    }
    fn exit(&self) -> AppResult<()> {
        screen::exit()
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