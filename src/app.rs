use std::error;
use std::error::Error;
use std::ops::ControlFlow::Continue;
use std::ops::Deref;

use crossterm::cursor::position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::args::Args;
use crate::event::{flush_resize_events, Event, EventHandler};
use crate::render::banner::Banner;
use crate::render::document::Document;
use crate::render::message::MessageBar;
use crate::render::switcher::DocumentSwitcher;
use crate::render::Render;
use crate::screen::{Offset, Position, Screen};
use crate::{screen, DEFAULT_FILENAME, DEFAULT_FILETYPE};

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
    // actual position of the current cursor
    actual: Offset,
    // relative position of the current cursor(screen)
    relative: Position,
}

#[derive(Clone, Copy)]
pub struct AppCtx {
    // actual position of the current cursor
    pub actual: Offset,
    // relative position of the current cursor(screen)
    pub relative: Position,
    // screen size
    pub screen_size: (u16, u16),
    pub doc_size: (usize, usize),
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
            self.refresh_screen();

            if let Err(err) = self.dispatch_events() {
                exit_with_err(err)
            }
        }

        self.exit()
    }

    /// dispatch events.
    fn dispatch_events(&mut self) -> AppResult<()> {
        match self.events.next()? {
            Event::Tick => self.on_tick(),
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
    fn refresh_screen(&mut self) -> AppResult<()> {
        let ctx = self.new_ctx();
        let buf = self.screen.get_buf();
        if self.doc_switcher.is_empty() {
            self.banner.render(ctx, buf, buf.area);
            self.screen.refresh()?;

            return Ok(());
        }
        screen::move_to(self.relative)?;

        self.doc_switcher.render(ctx, buf, buf.area);

        self.screen.refresh_and_set_cursor(self.relative)
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
                    self.move_cursor(KeyCode::Null);
                }
            }

            (KeyCode::Right, key_modifier) => {
                // switch buffer
                if key_modifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.doc_switcher.next();
                    self.move_cursor(KeyCode::Null);
                }
            }

            _ => {}
        };
    }

    /// on tick event
    fn on_tick(&mut self) {
        self.refresh_screen().unwrap();
    }

    /// move cursor
    fn move_cursor(&mut self, key_code: KeyCode) {
        let Position { mut x, mut y } = self.relative;
        let (screen_width, screen_height) = screen::size().unwrap();
        let (doc_width, doc_height) = self.doc_switcher.current_doc_size(y);

        /// todo 要获取下个字符的具体长度, 比如说如果是中文那么下一个就有可能不是直接+1
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
                // 支持在第一行向上移动时,跳到最后一行.
                if y == 0 {
                    y = doc_height;
                } else {
                    y = y.saturating_sub(1);
                }
                // y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                if y < doc_height {
                    y = y.saturating_add(1);
                }
            }
            // todo 获取这一行的第一个不是空格的idx
            KeyCode::Home => x = 0,
            KeyCode::End => x = doc_width,
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            _ => {}
        }

        let doc_width = self.doc_switcher.current_doc_row_len(y);
        if x > doc_width {
            x = doc_width;
        }

        // 索引是从0开始的,所以减1,
        if y > doc_height - 1 {
            y = doc_height - 1
        }

        let bottom_height = self.doc_switcher.get_bottom_height();
        if y > screen_height as usize - bottom_height {
            y = (screen_height as usize - bottom_height)
        }

        self.relative = Position { x, y };
    }

    fn exit(&self) -> AppResult<()> {
        screen::exit()
    }

    fn new_ctx(&self) -> AppCtx {
        let doc_size = self.doc_switcher.current_doc_size(self.relative.y);

        AppCtx {
            actual: self.actual,
            relative: self.relative,
            screen_size: screen::size().unwrap(),
            doc_size,
        }
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
