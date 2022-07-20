use anyhow::Context;
use ropey::{Rope, RopeBuilder};

use crate::app::AppResult;
use crate::render::Render;

/// the document
pub struct Document {
    pub content: Rope,
    name: String,
}

pub struct DocumentSwitcher {
    documents: Vec<Document>,
    index: usize,
    empty: bool,
}

impl Render for Document {
    fn draw() {
        todo!()
    }
}

impl Document {
    pub fn from(content: Rope, filepath: &str) -> Self {
        Self {
            content,
            name: filepath.to_string(),
        }
    }

    pub fn open(filepath: &str) -> AppResult<Self> {
        let reader = std::fs::File::open(filepath)?;

        let content = Rope::from_reader(reader)?;

        Ok(Document::from(content, filepath))
    }

    pub fn default() -> Self {
        Self {
            content: Default::default(),
            name: "".to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.len_bytes() == 0
    }

    pub fn name(&self) -> String {
        self.name.clone()
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