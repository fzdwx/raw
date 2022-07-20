use crate::buffer::{get_text_and_status_layout, Buffered};
use crate::filetype::FileType;
use crate::row::Row;
use crate::tui::Tui;
use std::fs;
use std::io::{Error, Stdout};
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;
use tui::Frame;

pub struct Text {
    rows: Vec<Row>,
    filename: String,
    filetype: FileType,
}

impl Text {
    pub fn open(filename: &str) -> Result<Text, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            filename: filename.to_string(),
            // todo 解析文件类型
            filetype: FileType::default(),
        })
    }

    /// get file type
    pub fn filetype(&self) -> FileType {
        self.filetype.clone()
    }

    /// get text length
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// get row by index.
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}

impl Buffered for Text {
    fn name(&self) -> String {
        self.filename.clone()
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        frame.render_widget(self, get_text_and_status_layout(frame.size())[0]);
    }
}

impl Widget for &Text {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let y = area.y as usize;
        for terminal_row in 0..area.height {
            if let Some(row) = self.rows.get(y.saturating_add(terminal_row as usize)) {
                let width = area.width as usize;
                let start = area.x as usize;
                let end = start + width as usize;

                let line = row.render(start, end);
                buf.set_string(
                    area.left(),
                    (terminal_row) + area.y,
                    line.as_str(),
                    Style::default(),
                );
            }
        }
    }
}

pub struct TextBufferContainer {
    texts: Vec<Text>,
    index: usize,
    empty: bool,
}

impl TextBufferContainer {
    pub fn default() -> Self {
        Self {
            texts: Vec::new(),
            index: 0,
            empty: true,
        }
    }

    /// add text to last.
    pub fn add(&mut self, text: Text) {
        let text_empty = text.is_empty();
        self.texts.push(text);
        self.update_empty(text_empty)
    }

    /// remove current text.
    pub fn remove_current(&mut self) -> Text {
        self.remove(self.index)
    }

    /// remove text by index.
    pub fn remove(&mut self, index: usize) -> Text {
        let result = self.texts.remove(index);
        if self.texts.is_empty() {
            self.empty = true;
        }
        result
    }

    /// move to next text.
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.texts.len();
    }

    /// move to prev text.
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.texts.len() - 1;
        }
    }

    /// move index to origin.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// get text by index.
    ///
    /// only get,don't move to index.
    pub fn get(&self, index: usize) -> Option<&Text> {
        self.texts.get(index)
    }

    /// get current text.
    pub fn current(&self) -> Option<&Text> {
        self.get(self.index)
    }

    /// get container size.
    pub fn size(&self) -> usize {
        self.texts.len()
    }

    /// load files
    pub fn load(&mut self, filenames: Vec<String>) {
        for filename in filenames {
            match Text::open(filename.as_str()) {
                Ok(text_buffer) => self.add(text_buffer),
                Err(_) => {

                    // todo file not found error,collect it,return to app
                }
            }
        }
    }

    fn update_empty(&mut self, text_empty: bool) {
        if !self.empty {
            return;
        }

        if text_empty {
            return;
        }

        self.empty = text_empty;
    }
}

impl Buffered for TextBufferContainer {
    fn name(&self) -> String {
        self.current().unwrap().name()
    }

    fn is_empty(&self) -> bool {
        self.empty
    }

    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        self.current().unwrap().draw(frame)
    }
}