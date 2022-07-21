use crate::render::document::Document;
use crate::render::extend::RectEx;
use crate::render::status_line::StatusLine;
use crate::render::Render;
use std::borrow::Borrow;
use tui::buffer::Buffer;
use tui::layout::Rect;

pub struct DocumentSwitcher {
    documents: Vec<Document>,
    index: usize,
    empty: bool,
    status_line: StatusLine,
}

impl Render for DocumentSwitcher {
    fn name(&self) -> String {
        self.current().unwrap().name()
    }

    fn render(&mut self, buf: &mut Buffer, area: Rect) {
        self.current_mut().unwrap().render(buf, area.to_document());

        let current = self.current().unwrap();
        self.status_line.refresh(current.name(), current.filetype());
        self.status_line.render(buf, area.to_status_line());
    }
}

impl DocumentSwitcher {
    pub fn default() -> Self {
        Self {
            documents: Vec::new(),
            index: 0,
            empty: true,
            status_line: StatusLine::default(),
        }
    }

    /// check
    pub fn is_empty(&self) -> bool {
        self.empty
    }

    /// add text to last.
    pub fn add(&mut self, doc: Document) {
        let doc_empty = doc.is_empty();
        self.documents.push(doc);
        self.update_empty(doc_empty)
    }

    /// remove current text.
    pub fn remove_current(&mut self) -> Document {
        self.remove(self.index)
    }

    /// remove text by index.
    pub fn remove(&mut self, index: usize) -> Document {
        let result = self.documents.remove(index);
        if self.documents.is_empty() {
            self.empty = true;
        }
        result
    }

    /// move to next text.
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.documents.len();
    }

    /// move to prev text.
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.documents.len() - 1;
        }
    }

    /// move index to origin.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// get text by index.
    ///
    /// only get,don't move to index.
    pub fn get(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)
    }

    /// get current text.
    pub fn current_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.index)
    }

    pub fn current(&self) -> Option<&Document> {
        self.documents.get(self.index)
    }

    /// get container size.
    pub fn size(&self) -> usize {
        self.documents.len()
    }

    /// load files
    pub fn load(&mut self, filenames: Vec<String>) {
        for filename in filenames {
            match Document::open(filename.as_str()) {
                Ok(doc) => self.add(doc),
                Err(_) => {
                    // todo file not found error,collect it,return to app
                }
            }
        }
    }

    fn update_empty(&mut self, doc_empty: bool) {
        if !self.empty {
            return;
        }

        if doc_empty {
            return;
        }

        self.empty = doc_empty;
    }
}