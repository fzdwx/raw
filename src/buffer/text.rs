use crate::buffer::Buffered;
use crate::row::Row;
use std::fs;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;
use tui::Frame;

pub struct TextBuffer {
    rows: Vec<Row>,
    filename: String,
}

impl TextBuffer {
    pub fn open(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).map_or("".to_string(), |s| s);
        let mut rows = Vec::new();

        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Self {
            rows,
            filename: filename.to_string(),
        }
    }
}

impl Buffered for TextBuffer {
    fn name(&self) -> String {
        self.filename.clone()
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        todo!()
    }
}

impl Widget for &TextBuffer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}

pub struct TextBufferContainer {
    texts: Vec<TextBuffer>,
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
    pub fn add(&mut self, text: TextBuffer) {
        let text_empty = text.is_empty();
        self.texts.push(text);
        self.update_empty(text_empty)
    }

    /// remove current text.
    pub fn remove_current(&mut self) -> TextBuffer {
        self.remove(self.index)
    }

    /// remove text by index.
    pub fn remove(&mut self, index: usize) -> TextBuffer {
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
    pub fn get(&self, index: usize) -> Option<&TextBuffer> {
        self.texts.get(index)
    }

    /// get current text.
    pub fn current(&self) -> Option<&TextBuffer> {
        self.get(self.index)
    }

    /// get container size.
    pub fn size(&self) -> usize {
        self.texts.len()
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