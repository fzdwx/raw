use std::borrow::Borrow;

use anyhow::Context;
use ropey::{Rope, RopeBuilder, RopeSlice};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Span;

use crate::app::AppResult;
use crate::render::extend::RopeSliceEx;
use crate::render::switcher::DocumentSwitcher;
use crate::render::Render;
use crate::{DEFAULT_FILENAME, DEFAULT_FILETYPE};

/// the document
pub struct Document {
    pub content: Rope,
    name: String,
    filetype: String,
}

impl Render for Document {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn render(&mut self, buf: &mut Buffer, area: Rect) {
        if self.is_empty() {
            return;
        }

        let mut x = 0;
        for line in self.content.lines() {
            if x >= area.height {
                return;
            };

            // todo 是不是太暴力了.
            buf.set_string(0, x, line.get_string(), Style::default());
            x += 1;
        }
    }
}

impl Document {
    pub fn from(content: Rope, filepath: &str) -> Self {
        Self {
            content,
            name: filepath.to_string(),
            filetype: DEFAULT_FILETYPE.to_string(),
        }
    }

    pub fn open(filepath: &str) -> AppResult<Self> {
        let reader = std::fs::File::open(filepath)?;

        let content = Rope::from_reader(reader)?;

        Ok(Document::from(content, filepath))
    }

    pub fn default() -> Self {
        Self::from(Default::default(), DEFAULT_FILENAME)
    }

    /// get content.
    pub fn content(&self) -> &Rope {
        &self.content
    }

    /// How many lines are in the current document
    pub fn len(&self) -> usize {
        self.content.len_lines()
    }

    /// get line len.
    pub fn line_len(&self, index: usize) -> usize {
        if index > self.len() {
            return 0;
        }

        // todo 只是简单的-2(因为有/r/n)
        self.content.line(index).len_bytes() - 2
    }

    /// get line by index.
    pub fn line(&self, index: usize) -> RopeSlice {
        self.content.line(index)
    }

    pub fn is_empty(&self) -> bool {
        self.content.len_bytes() == 0
    }

    pub fn filetype(&self) -> String {
        self.filetype.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::render::document::Document;

    #[test]
    fn test_line_len() {
        // std::fs::File::open("./src/render/document.rs")?;
        let doc = Document::open("./src/render/document.rs").unwrap();
        println!("{}", doc.line(0));
        println!("{}", doc.line_len(0));
    }
}