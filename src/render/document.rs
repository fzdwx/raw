use anyhow::Context;
use ropey::{Rope, RopeBuilder};
use std::borrow::Borrow;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::Span;

use crate::app::AppResult;
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
            let string = format!("{}", line);
            buf.set_string(0, x, string, Style::default());
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

    pub fn is_empty(&self) -> bool {
        self.content.len_bytes() == 0
    }

    pub fn filetype(&self) -> String {
        self.filetype.clone()
    }
}