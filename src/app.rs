use crate::terminal::Terminal;
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::io::Error;
use std::time::Duration;

/// the 'raw' application.
pub struct App {
    terminal: Terminal,
    should_quit: bool,
    mouse_event: Option<MouseEvent>,
}

impl App {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(),
            should_quit: false,
            mouse_event: None,
        }
    }

    /// run editor
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.process_event() {
                self.die(&error);
            }

            if self.should_quit() {
                break;
            }
        }
    }

    /// process user events.
    fn process_event(&mut self) -> Result<(), Error> {
        return match self.terminal.read() {
            Ok(event) => {
                match event {
                    Event::Key(input_key) => self.process_keypress(input_key),

                    Event::Mouse(m) => self.process_mouse_moved(m),

                    Event::Resize(_, _) => self.process_resize(event),
                }
                Ok(())
            }
            Err(err) => Err(err),
        };
    }

    /// process keypress event.
    fn process_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                self.should_quit = true;
            }
            // discard
            _ => {}
        };
    }

    /// process mouse move event.
    fn process_mouse_moved(&mut self, mouse: MouseEvent) {
        if self.should_discard_mouse_move_event(mouse) {
            return;
        }

        println!("mouse moved: {:?}", mouse);
    }

    /// process resize events.
    fn process_resize(&self, event: Event) {
        let (original_size, new_size) = self.flush_resize_events(event);
        println!("Resize from: {:?}, to: {:?}", original_size, new_size);
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
                if let Ok(Event::Resize(x, y)) = self.terminal.read() {
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
        return match self.mouse_event {
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
        };
    }

    /// if should quit do terminal destroy
    fn should_quit(&mut self) -> bool {
        match self.should_quit {
            true => {
                self.terminal.clear_all().expect("clear error");
                self.terminal.destroy();
                println!("bye!\r");
            }
            false => {}
        }

        self.should_quit
    }

    /// has error,panic it.
    fn die(&mut self, error: &Error) {
        self.terminal.destroy();
        panic!("{}", error);
    }
}