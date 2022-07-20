use crate::args::Args;
use crate::buffer::banner::Banner;
use crate::buffer::statusline::StatusLine;
use crate::buffer::text::TextBufferContainer;
use crate::buffer::{Buffer, Buffered};
use crate::position::Position;
use crate::tui::Tui;
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;
use std::io::{stdout, Error, Stdout};
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
    position: Position,
}

impl Default for App {
    fn default() -> Self {
        let mut text_container = TextBufferContainer::default();
        let args = Args::load();

        text_container.load(args.filenames);

        let x = include_bytes!("../banner");
        Self {
            tui: Tui::default(),
            running: true,
            show_banner: false,
            banner: Banner::default(),
            text_container,
            status: StatusLine::default(),
            mouse_event: None,
            position: Position::default(),
        }
    }
}

impl App {
    /// run editor
    pub fn run(&mut self) {
        while self.running {
            if let Err(error) = self.draw_ui() {
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

    fn draw_ui(&mut self) -> std::io::Result<()> {
        self.tui.hide_cursor().ok();
        if self.text_container.is_empty() || self.show_banner {
            return self.tui.draw_buffer_auto(&self.banner);
        }

        self.tui.draw(|frame| {
            self.text_container.draw(frame);
            self.status.refresh(self.text_container.current());

            self.position.moved(frame, self.text_container.current());
            self.status.draw(frame);
        })?;

        Ok(())
    }

    fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.position.sub_x(1),
            KeyCode::Right => self.position.add_x(1),
            KeyCode::Up => self.position.sub_y(1),
            KeyCode::Down => self.position.add_y(1),
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            _ => {}
        }
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

fn move_to() {}