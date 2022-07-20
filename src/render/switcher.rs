use crate::render::document::Document;
use crate::render::rect::Sub;
use crate::render::Render;
use tui::buffer::Buffer;
use tui::layout::Rect;

pub struct DocumentSwitcher {
    documents: Vec<Document>,
    index: usize,
    empty: bool,
}

impl Render for DocumentSwitcher {
    fn draw(&self, buf: &mut Buffer, area: Rect) {
        let current = self.current().unwrap();

        current.draw(buf, area.to_text());
    }
}

impl DocumentSwitcher {
    pub fn default() -> Self {
        Self {
            documents: Vec::new(),
            index: 0,
            empty: true,
        }
    }

    pub fn name(&self) -> String {
        self.current().unwrap().name()
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
    pub fn current(&self) -> Option<&Document> {
        self.get(self.index)
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