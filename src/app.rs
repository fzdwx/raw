use std::error;
use std::error::Error;
use std::ops::ControlFlow::Continue;
use std::ops::Deref;

use crossterm::cursor::position;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::args::Args;
use crate::event::{flush_resize_events, Event, EventHandler};
use crate::extension::graphemes_ex::{
    next_grapheme_boundary, nth_prev_grapheme_boundary, prev_grapheme_boundary,
};
use crate::extension::rope::{Line, RopeSliceEx};
use crate::render::banner::Banner;
use crate::render::document::Document;
use crate::render::message::MessageBar;
use crate::render::switcher::DocumentSwitcher;
use crate::render::Render;
use crate::screen::{Position, Screen};
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
    /// cursor 的偏移量,即超出屏幕的部分
    offset: Position,
    ///   cursor position
    cursor: Position,
}

#[derive(Clone)]
pub struct AppCtx {
    pub current_line: Line,
    // the offset on the (x,y)
    pub offset: Position,
    //   cursor position
    pub cursor: Position,
    // screen size
    pub screen_size: (u16, u16),
    pub doc_size: (usize, usize),
    pub bottom_height: usize,
}

impl AppCtx {
    /// 适配屏幕,获取光标最大可能存在的位置.
    pub fn get_cursor(&self) -> Position {
        Position {
            x: self
                .current_line
                .get_offset(self.cursor.x.saturating_sub(self.offset.x)),
            // x: self.cursor.x.saturating_sub(self.offset.x),
            y: self.cursor.y.saturating_sub(self.offset.y),
        }
    }

    /// cal offset y
    pub fn cal_offset_y(&self) -> usize {
        self.offset.y
    }

    /// get screen height.
    pub fn get_screen_height(&self) -> usize {
        self.screen_size.1 as usize
    }
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
            offset: Default::default(),
            cursor: Default::default(),
        }
    }

    /// run app
    pub fn run(&mut self) -> AppResult<()> {
        while self.running {
            if let Err(err) = self.refresh_screen() {
                exit_with_err(err)
            }

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
            // resize mouse discard
            _ => {}
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

        let position = ctx.get_cursor();
        screen::move_to(position)?;

        self.doc_switcher.render(ctx, buf, buf.area);

        self.screen.refresh_and_set_cursor(position)
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
        self.scroll();
    }

    /// on tick event
    fn on_tick(&mut self) {
        self.refresh_screen().unwrap();
    }

    /// move cursor
    fn move_cursor(&mut self, key_code: KeyCode) {
        let Position { mut x, mut y } = self.cursor;
        let offset = self.offset;
        let (screen_width, screen_height) = screen::size().unwrap();

        // row
        let rope_slice = self.doc_switcher.current_doc_row(y);
        let doc_height = self.doc_switcher.current_doc_height();
        let doc_width = rope_slice.len_word_boundary();
        let row_width = rope_slice.len_chars();

        match key_code {
            KeyCode::Left => {
                if x > 0 {
                    x = x.saturating_sub(1);
                } else if y > 0 {
                    y = y.saturating_sub(1);
                    x = doc_width
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
            KeyCode::Home => x = 0,
            KeyCode::End => x = doc_width,
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            _ => {}
        }

        let current_row_width = self.doc_switcher.current_doc_row(y).len_word_boundary();
        if x > current_row_width {
            x = current_row_width;
        }

        // 索引是从0开始的,所以减1,
        if y > doc_height - 1 {
            y = doc_height - 1
        }

        self.cursor = Position { x, y };
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor;
        let screen_size = screen::size().unwrap();
        let h = (screen_size.1 as usize) - self.doc_switcher.get_bottom_height();
        let w = screen_size.0 as usize;
        let line = self.doc_switcher.current_doc_row_to_line(y);

        let mut offset = self.offset;
        if y < offset.y {
            offset.y = y
        } else if y >= offset.y.saturating_add(h) {
            offset.y = y.saturating_sub(h).saturating_add(1);
        };

        /// todo  offset.x 有问题
        if x < offset.x {
            offset.x = x;
        } else if line.get_offset(x) >= offset.x.saturating_add(w) {
            offset.x = line.get_offset(x).saturating_sub(w).saturating_add(1);
        }

        self.offset = offset;
    }

    fn exit(&self) -> AppResult<()> {
        screen::exit()
    }

    fn new_ctx(&self) -> AppCtx {
        let doc_size = self.doc_switcher.current_doc_size(self.cursor.y);
        let bottom_height = self.doc_switcher.get_bottom_height();

        AppCtx {
            current_line: self.doc_switcher.current_doc_row_to_line(self.cursor.y),
            offset: self.offset,
            cursor: self.cursor,
            screen_size: screen::size().unwrap(),
            doc_size,
            bottom_height,
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