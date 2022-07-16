use std::io::Error;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::terminal::{Terminal, Position};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    // quit flag
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    /// create default Editor
    pub fn default() -> Editor {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::origin(),
        }
    }

    /// run editor
    pub fn run(&mut self) {

        // loop
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            };

            if self.check_quit() {
                break;
            }

            if let Err(error) = self.process_event() {
                die(error);
            }
        }
    }

    /// refresh screen
    fn refresh_screen(&self) -> std::io::Result<()> {
        Terminal::cursor_hide();
        Terminal::clear_screen_all();
        Terminal::move_to_origin();

        if self.check_quit() {
            return Ok(());
        } else {
            self.draw_rows();
            Terminal::move_to(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    /// read event and process event
    fn process_event(&mut self) -> Result<(), Error> {
        return match Terminal::read_event() {
            Ok(event) => {
                match event {
                    Event::Key(input_key) => {
                        self.process_keypress(input_key)
                    }

                    Event::Mouse(_) => {
                        println!("{:?}", event);
                    }

                    Event::Resize(_, _) => {
                        println!("{:?}", event);
                    }
                }

                Ok(())
            }

            Err(err) => {
                Err(err)
            }
        };
    }

    fn process_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                self.should_quit = true
            }

            // handle other event
            _ => {
                println!("{:?}", key);
            }
        };
    }

    /// Confirm whether you need to exit the editor.
    pub fn check_quit(&self) -> bool {
        match self.should_quit {
            true => {
                Terminal::clear_screen_all();
                println!("bye!\r");

                Terminal::disable_raw_mode();
            }
            false => {}
        }

        self.should_quit
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        let x = height / 3;
        for row in 0..height - 1 {
            Terminal::clear_screen_current_line();

            if row == x {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    /// write welcome message
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Raw editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

/// handle error
fn die(e: Error) {
    Terminal::clear_screen_all();
    panic!("{}", e);
}