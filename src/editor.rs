use std::env::args;
use std::io::Error;
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Stylize};

use crate::document::Document;
use crate::row::Row;
use crate::terminal::{Position, Size, Terminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};

pub struct Editor {
    // quit flag
    should_quit: bool,
    // terminal
    terminal: Terminal,
    // curr cursor position
    cursor_position: Position,
    // limit where the cursor can go
    offset: Position,
    // curr edit document
    document: Document,
    status_message: StatusMessage,
}

impl Editor {
    /// create default Editor
    pub fn default() -> Editor {
        let args: Vec<String> = args().collect();
        let mut initial_status = String::from("HELP: Ctrl-S = save | Ctrl-Q = quit");

        let document = if args.len() > 1 {
            let filename = &args[1];
            let doc = Document::open(filename);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", filename);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
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
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::move_to(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
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

    /// process key event.
    fn process_keypress(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // handler quit editor
            (KeyCode::Char('q'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                self.should_quit = true
            }

            (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.save(),

            // add char
            (KeyCode::Char(c), _) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(KeyCode::Right);
            }

            // delete
            (KeyCode::Delete, _) => self.document.delete(&self.cursor_position),
            (KeyCode::Backspace, _) => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(KeyCode::Left);
                    self.document.delete(&self.cursor_position);
                }
            }

            // new line
            (KeyCode::Enter, _) => {
                self.document.insert(&self.cursor_position, '\n');
                self.move_cursor(KeyCode::Right);
            }

            (KeyCode::Up, _)
            | (KeyCode::Down, _)
            | (KeyCode::Left, _)
            | (KeyCode::Right, _)
            | (KeyCode::PageUp, _)
            | (KeyCode::PageDown, _)
            | (KeyCode::End, _)
            | (KeyCode::Home, _) => self.move_cursor(key.code),

            // discard
            _ => {}
        };

        self.scroll();
    }

    /// save document
    fn save(&mut self) {
        if self.document.filename.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.filename = new_name;
        }

        match self.document.save() {
            Ok(_) => {
                self.status_message = StatusMessage::from("File saved successfully".to_string());
            }
            Err(err) => {
                self.status_message =
                    StatusMessage::from(format!("Error writing file, cause: {}", err));
            }
        }
    }

    /// scroll
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let w = self.terminal.size().width as usize;
        let h = self.terminal.size().height as usize;

        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(h) {
            offset.y = y.saturating_sub(h).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(w) {
            offset.x = x.saturating_sub(w).saturating_add(1);
        }
    }

    /// move cursor by key code
    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;
        let terminal_height = self.terminal.size().height as usize;

        let h = self.document.len();
        let mut w = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < h {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < w {
                    x += 1;
                } else if y < h {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < h {
                    y + terminal_height as usize
                } else {
                    h
                }
            }

            KeyCode::Home => x = 0,
            KeyCode::End => x = w,
            _ => (),
        }

        // making sure that x does not exceed the current row’s width.
        w = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > w {
            x = w
        }

        // fix 可以多下一行
        if y == h && h != 0 {
            y = y - 1
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
        let width = self.terminal.size().width as usize;

        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    /// draw document to terminal
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        let x = height / 3;
        for terminal_row in 0..height {
            Terminal::clear_screen_current_line();

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row)
            } else if self.document.is_empty() && terminal_row == x {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    /// draw status bar
    fn draw_status_bar(&self) {
        let mut status;
        let w = self.terminal.size().width as usize;
        let mut filename = "[No name]".to_string();
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20)
        }

        status = format!("{} - {} lines", filename, self.document.len());

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if w > len {
            status.push_str(&" ".repeat(w - len));
        }

        status = format!("{}{}", status, line_indicator);

        status.truncate(w);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    /// draw message bar
    fn draw_message_bar(&self) {
        Terminal::clear_screen_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
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

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, Error> {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let event = Terminal::read_event().unwrap();

            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Backspace => result.truncate(result.len().saturating_sub(1)),
                    KeyCode::Enter => break,
                    KeyCode::Char(c) => {
                        if !c.is_control() {
                            result.push(c);
                        }
                    }
                    KeyCode::Esc => {
                        result.truncate(0);
                        break;
                    }
                    _ => (),
                }
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
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