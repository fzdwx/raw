use crate::args::Args;
use crate::buffer::banner::Banner;
use crate::buffer::statusline::StatusLine;
use crate::buffer::text::TextBufferContainer;
use crate::buffer::{Buffer, Buffered};
use crate::tui::Tui;
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::io::{Error, Stdout};
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::terminal::CompletedFrame;
use tui::{Frame, Terminal};

/// the 'raw' application.
pub struct App {
    // the terminal helper.
    tui: Tui,
    // if is false,then exit app.
    running: bool,
    // if is true,then show banner.
    show_banner: bool,
    // the banner buffer
    banner: Banner,
    status: StatusLine,
    // text buffer container.
    text_container: TextBufferContainer,
    // mouse event, reduce the occurrence of resize events
    mouse_event: Option<MouseEvent>,
}

impl Default for App {
    fn default() -> Self {
        let mut text_container = TextBufferContainer::default();
        let args = Args::load();

        text_container.load(args.filenames);

        let x = include_bytes!("banner");
        Self {
            tui: Tui::default(),
            running: true,
            show_banner: false,
            banner: Banner::default(),
            text_container,
            status: StatusLine::default(),
            mouse_event: None,
        }
    }
}

impl App {
    /// run editor
    pub fn run(&mut self) {
        while self.running {
            if let Err(error) = self.draw_content() {
                self.die(&error);
            }

            if let Err(error) = self.process_event() {
                self.die(&error);
            }
        }

        self.tui.clear_all().expect("tui clear all error");
        self.tui.destroy().expect("tui destroy error");
        println!("bye!\r");
    }

    fn draw_content(&mut self) -> std::io::Result<()> {
        // self.tui.hide_cursor().ok();

        // todo 光标位置有问题
        self.tui.draw(|frame| {
            if self.text_container.is_empty() || self.show_banner {
                self.banner.draw(frame);
            } else {
                Tui::move_to(10, 10);
                self.text_container.draw(frame);
                self.status.refresh(self.text_container.current());
                self.status.draw(frame);
            }
        })?;

        self.tui.show_cursor()
    }

    /// process keypress event.
    fn process_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.running = false;
            }

            (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.show_banner = !self.show_banner;
            }

            (KeyCode::Left, modifier) => {
                // switch buffer
                if modifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.text_container.next();
                }
            }

            (KeyCode::Right, keyModifier) => {
                // switch buffer
                if keyModifier == KeyModifiers::CONTROL | KeyModifiers::ALT {
                    self.text_container.prev();
                }
            }

            // (, KeyModifiers::CONTROL) => {}
            // (KeyCode::Right, _) => {
            //     self.index = (self.index + 1) % self.titles.len();
            // }
            // (KeyCode::Left, _) => {
            //     if self.index > 0 {
            //         self.index -= 1;
            //     } else {
            //         self.index = self.titles.len() - 1;
            //     }
            // }
            // discard
            _ => {}
        };
    }

    /// process user events.
    fn process_event(&mut self) -> Result<(), Error> {
        match self.tui.read() {
            Ok(event) => {
                match event {
                    Event::Key(input_key) => self.process_keypress(input_key),

                    Event::Mouse(m) => self.process_mouse_moved(m),

                    Event::Resize(_, _) => self.process_resize(event),
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// process mouse move event.
    fn process_mouse_moved(&mut self, mouse: MouseEvent) {
        if self.should_discard_mouse_move_event(mouse) {
            return;
        }

        // println!("mouse moved: {:?}", mouse);
    }

    /// process resize events.
    fn process_resize(&mut self, event: Event) {
        let (original_size, new_size) = self.flush_resize_events(event);
        self.tui.resize();
        // println!("Resize from: {:?}, to: {:?}", original_size, new_size);
    }

    /// Resize events can occur in batches.
    ///
    /// With a simple loop they can be flushed.
    ///
    /// This function will keep the first and last resize event.
    fn flush_resize_events(&self, event: Event) -> ((u16, u16), (u16, u16)) {
        if let Event::Resize(x, y) = event {
            let mut last_resize = (x, y);
            while let Ok(true) = poll(Duration::from_millis(50)) {
                if let Ok(Event::Resize(x, y)) = self.tui.read() {
                    last_resize = (x, y);
                }
            }

            return ((x, y), last_resize);
        }
        ((0, 0), (0, 0))
    }

    /// Mouse move events are also generated when the mouse is stationary.
    ///
    /// So the judgment is discarded if it is the same as the last mouse movement event.
    fn should_discard_mouse_move_event(&mut self, mouse: MouseEvent) -> bool {
        match self.mouse_event {
            None => {
                self.mouse_event = Some(mouse);

                true
            }
            Some(old_mouse) => {
                if old_mouse == mouse {
                    return true;
                }

                self.mouse_event = Some(mouse);
                false
            }
        }
    }

    /// has error,panic it.
    fn die(&mut self, error: &Error) {
        self.tui.destroy().expect("tui destroy error");
        panic!("{}", error);
    }
}

fn draw(bufs: Vec<Buffer>, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    for buf in bufs {
        match buf {
            Buffer::Banner(b) => {
                b.draw(frame);
            }
            Buffer::Text(b) => {
                b.draw(frame);
            }
            Buffer::StatusLine(b) => {
                b.draw(frame);
            }
            Buffer::TextBufferContainer(b) => b.draw(frame),
        }
    }
}