use std::env::args;
use std::io::Error;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;

use crate::document::Document;
use crate::row::Row;
use crate::terminal::{Position, Size, Terminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    // quit flag
    should_quit: bool,
    // terminal
    terminal: Terminal,
    // curr cursor position
    cursor_position: Position,
    // curr edit document
    document: Document,
}

impl Editor {
    /// create default Editor
    pub fn default() -> Editor {
        let args: Vec<String> = args().collect();

        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
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
                    Event::Key(input_key) => self.process_keypress(input_key),

                    Event::Mouse(_) => {
                        println!("{:?}", event);
                    }

                    Event::Resize(_, _) => {
                        println!("{:?}", event);
                    }
                }

                Ok(())
            }

            Err(err) => Err(err),
        };
    }

    fn process_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                self.should_quit = true
            }

            (KeyCode::Up, _)
            | (KeyCode::Down, _)
            | (KeyCode::Left, _)
            | (KeyCode::Right, _)
            | (KeyCode::PageUp, _)
            | (KeyCode::PageDown, _)
            | (KeyCode::End, _)
            | (KeyCode::Home, _) => self.move_cursor(key.code),

            // handle other event
            _ => {
                println!("{:?}", key);
            }
        };
    }

    /// move cursor by key code
    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        let h = size.height.saturating_sub(1) as usize;
        let w = size.width.saturating_sub(1) as usize;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < h {
                    y = y.saturating_add(1)
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < w {
                    x = x.saturating_add(1)
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = h,
            KeyCode::Home => x = 0,
            KeyCode::End => x = w,
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    /// Confirm whether you need to exit the editor.
    pub fn check_quit(&self) -> bool {
        match self.should_quit {
            true => {
                Terminal::clear_screen_all();
                println!("bye!\r");

                Terminal::cursor_show();
                Terminal::disable_raw_mode();
            }
            false => {}
        }

        self.should_quit
    }

    /// draw row to terminal
    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    /// draw document to terminal
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        let x = height / 3;
        for terminal_row in 0..height - 1 {
            Terminal::clear_screen_current_line();

            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row)
            } else if self.document.is_empty() && terminal_row == x {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    /// write welcome message
    fn draw_welcome_message(&self) {
        println!(
            "{}\r",
            format_to_center(
                format!("{} editor -- version {}", "raw".cyan().bold(), VERSION),
                self.terminal.size()
            )
        );
        println!(
            "{}\r",
            format_to_center(
                format!("{}", "hello world".to_string().red()),
                self.terminal.size()
            )
        );
    }
}

/// format to center
fn format_to_center(mut str: String, size: &Size) -> String {
    let width = size.width as usize;
    let len = str.len();
    let padding = width.saturating_sub(len) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    str = format!("~{}{}", spaces, str);
    str.truncate(width);

    str
}

/// handle error
fn die(e: Error) {
    Terminal::clear_screen_all();
    panic!("{}", e);
}