use std::error;
use std::error::Error;
use std::ops::ControlFlow::Continue;
use std::ops::Deref;

use crossterm::cursor::position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::args::Args;
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
    relative: Offset,
    actual: Position,
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
            Event::Tick => {}
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
    fn draw_some(&mut self) -> AppResult<()> {
        let buf = self.screen.get_buf();
        if self.doc_switcher.is_empty() {
            self.banner.render(buf, buf.area);
            self.screen.refresh()?;

            return Ok(());
        }
        screen::move_to(self.actual)?;

        self.doc_switcher.render(buf, buf.area);

        self.screen.refresh_and_set_cursor(self.actual)
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
            (KeyCode::Up, _)
            | (KeyCode::Down, _)
            | (KeyCode::Left, KeyModifiers::NONE)
            | (KeyCode::Right, KeyModifiers::NONE)
            | (KeyCode::PageUp, _)
            | (KeyCode::PageDown, _)
            | (KeyCode::End, _)
            | (KeyCode::Home, _) => self.move_cursor(key.code),

            (KeyCode::Left, modifier) => {
                // switch buffer
                if modifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.doc_switcher.prev();
                }
            }

            (KeyCode::Right, key_modifier) => {
                // switch buffer
                if key_modifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.doc_switcher.next();
                }
            }

            _ => {}
        };
    }

    /// move cursor
    fn move_cursor(&mut self, key_code: KeyCode) {
        let Position { mut x, mut y } = self.actual;
        let (screen_width, screen_height) = screen::size().unwrap();
        let (doc_width, doc_height) = self.doc_switcher.current_doc_size(y);

        match key_code {
            KeyCode::Left => {
                if x > 0 {
                    x = x.saturating_sub(1);
                } else if y > 0 {
                    y = y.saturating_sub(1);
                    x = self.doc_switcher.current_doc_row_len(y)
                }
            }
            KeyCode::Right => {
                // 正常向右移动一位
                if x < doc_width {
                    x = x.saturating_add(1);
                    // 换到下一行
                } else if y < doc_height {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                if y < doc_height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            _ => {}
        }

        let doc_width = self.doc_switcher.current_doc_row_len(y);
        if x > doc_width {
            x = doc_width;
        }

        if y == doc_height && doc_width != 0 {
            y -= 1;
        }

        self.actual = Position { x, y };
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