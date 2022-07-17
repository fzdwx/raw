use std::env::args;
use std::io::Error;
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Stylize};

use crate::document::Document;
use crate::row::Row;
use crate::terminal::{Position, Size, Terminal};

const BANNER_WIDTH: usize = 45;

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

const QUIT_TIMES: u8 = 1;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

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
    // banner
    banner: Document,
    // status message
    status_message: StatusMessage,
    // check quit times
    quit_times: u8,
    highlighted_word: Option<String>,
}

impl Editor {
    /// create default Editor
    pub fn default() -> Editor {
        let args: Vec<String> = args().collect();
        let mut initial_status =
            StatusMessage::info_raw("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

        let document = if let Some(filename) = args.get(1) {
            let doc = Document::open(filename);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = StatusMessage::error(
                    format!("Could not open file {}", filename.clone().green()),
                    doc.err().unwrap(),
                );
                Document::with_file_name(filename)
            }
        } else {
            Document::default()
        };

        let banner = String::from_utf8(include_bytes!("banner").to_vec()).unwrap();

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            banner: Document::with_string(banner),
            status_message: initial_status,
            quit_times: QUIT_TIMES,
            highlighted_word: None,
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
    fn refresh_screen(&mut self) -> std::io::Result<()> {
        Terminal::cursor_hide();
        Terminal::move_to_origin();

        if self.check_quit() {
            return Ok(());
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );

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
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::warn(format!(
                        "File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return;
                }
                self.should_quit = true;
            }

            (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.save(),
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.search(),

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

            // move cursor
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
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::default();
        }
    }

    /// save document
    fn save(&mut self) {
        if self.document.filename.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::info_raw("Save aborted.");
                return;
            }
            self.document.filename = new_name;
        }

        match self.document.save() {
            Ok(_) => {
                self.status_message = StatusMessage::info_raw("File saved successfully");
            }
            Err(err) => {
                self.status_message = StatusMessage::error_raw("Writing file fail", err);
            }
        }
    }

    // search some string
    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let callback = |editor: &mut Editor, key: KeyEvent, query: &String| {
            let mut moved = false;
            match key.code {
                KeyCode::Right | KeyCode::Down => {
                    direction = SearchDirection::Forward;
                    editor.move_cursor(KeyCode::Right);
                    moved = true;
                }

                KeyCode::Left | KeyCode::Up => direction = SearchDirection::Backward,
                _ => direction = SearchDirection::Forward,
            }

            if let Some(position) = editor
                .document
                .find(query, &editor.cursor_position, direction)
            {
                editor.cursor_position = position;
                editor.scroll();
            } else if moved {
                editor.move_cursor(KeyCode::Left)
            }

            editor.highlighted_word = Some(query.to_string());
        };

        let query = self
            .prompt("Search (ESC to cancel, Arrows to navigate): ", callback)
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }

        self.highlighted_word = None;
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
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < h {
                    y.saturating_add(terminal_height)
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
            y -= 1
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
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    /// draw document to terminal
    #[allow(clippy::integer_division, clippy::integer_arithmetic)]
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        let x = height / 3;
        for terminal_row in 0..height {
            Terminal::clear_screen_current_line();

            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row)
            } else if self.document.is_empty() && terminal_row == x {
                self.draw_banner();
            } else {
                println!("~\r");
            }
        }
    }

    /// draw status bar
    fn draw_status_bar(&self) {
        let mut status;
        let w = self.terminal.size().width as usize;

        let modified_indicato = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut filename = "[No name]".to_string();
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20)
        }

        status = format!(
            "{} - {} lines{}",
            filename,
            self.document.len(),
            modified_indicato
        );

        let line_indicator = format!(
            "{} | {}/{}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        #[allow(clippy::integer_arithmetic)]
        let len = status.len() + line_indicator.len();

        status.push_str(&" ".repeat(w.saturating_sub(len)));
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
    fn draw_banner(&self) {
        let mut width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let padding = width.saturating_sub(BANNER_WIDTH) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        for row in self.banner.rows() {
            let mut row = row.render(start, end);
            row = format!("~{}{}", spaces, row);

            if row.len() > width {
                while width > 0 {
                    if row.is_char_boundary(width) {
                        row.truncate(width);

                        break;
                    }
                    width -= 1;
                }
            }
            println!("{}\r", row);
        }

        // println!(
        //     "{}\r",
        //     format_to_center(
        //         format!("{} editor -- version {}", "raw".cyan().bold(), VERSION),
        //         self.terminal.size()
        //     )
        // );
        // println!(
        //     "{}\r",
        //     format_to_center(
        //         format!("{}", "hello world".to_string().red()),
        //         self.terminal.size()
        //     )
        // );
    }

    // prompt user.
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, Error>
    where
        C: FnMut(&mut Self, KeyEvent, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::prompt(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let event = Terminal::read_event().unwrap();

            if let Event::Key(key) = event {
                match (key.code, key.modifiers) {
                    (KeyCode::Backspace, _) => result.truncate(result.len().saturating_sub(1)),
                    (KeyCode::Enter, _) => break,

                    // over
                    (KeyCode::Esc, _) | (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        result.truncate(0);
                        break;
                    }

                    (KeyCode::Char(c), _) => {
                        if !c.is_control() {
                            result.push(c);
                        }
                    }
                    _ => (),
                }
                callback(self, key, &result)
            }
        }
        self.status_message = StatusMessage::default();
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

    fn default() -> Self {
        StatusMessage::from(String::new())
    }

    fn error(message: String, err: Error) -> Self {
        StatusMessage::new(
            "ERROR".red().to_string(),
            format!("{}. Cause is {}", message, err),
        )
    }

    fn warn(message: String) -> Self {
        StatusMessage::new("WARNING".yellow().to_string(), message)
    }

    fn info(message: String) -> Self {
        StatusMessage::new("INFO".green().to_string(), message)
    }

    fn prompt(message: String) -> Self {
        StatusMessage::new("PROMPT ".cyan().to_string(), message)
    }

    fn error_raw(message: &str, err: Error) -> Self {
        StatusMessage::error(message.to_string(), err)
    }

    fn info_raw(message: &str) -> Self {
        StatusMessage::info(message.to_string())
    }

    fn new(prefix: String, suffix: String) -> Self {
        StatusMessage::from(format!("{} {}", prefix, suffix))
    }
}

/// format to center
fn format_to_center(mut str: String, size: &Size) -> String {
    let width = size.width as usize;
    let len = str.len();
    #[allow(clippy::integer_arithmetic, clippy::integer_division)]
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