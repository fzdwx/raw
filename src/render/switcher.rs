use ropey::RopeSlice;

use tui::buffer::Buffer;
use tui::layout::Rect;

use crate::app::AppCtx;
use crate::extension::rect::RectEx;
use crate::extension::rope::{Line, RopeSliceEx};
use crate::render::document::Document;
use crate::render::message::MessageBar;
use crate::render::status_line::StatusLine;
use crate::render::Render;

pub struct DocumentSwitcher {
    documents: Vec<Document>,
    index: usize,
    empty: bool,
    status_line: StatusLine,
    message_bar: MessageBar,
}

impl Render for DocumentSwitcher {
    fn name(&self) -> String {
        self.current().unwrap().name()
    }

    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect) {
        let should_render_message_bar = self.message_bar.should_render();

        self.current_mut()
            .unwrap()
            .render(ctx, buf, area.to_document(should_render_message_bar));

        let current = self.current().unwrap();
        self.status_line.refresh(current.name(), current.filetype());
        self.status_line.render(ctx, buf, area.to_status_line());
        self.message_bar.render(ctx, buf, area.to_message_bar());
    }
}

impl DocumentSwitcher {
    pub fn default() -> Self {
        Self {
            documents: Vec::new(),
            index: 0,
            empty: true,
            status_line: StatusLine::default(),
            message_bar: MessageBar::default(),
        }
    }

    /// check
    pub fn is_empty(&self) -> bool {
        self.empty
    }

    /// a
    pub fn get_bottom_height(&self) -> usize {
        // 索引从0开始   +1
        // status_line +1
        let mut bottom_height: usize = 2; //

        if self.message_bar.should_render() {
            // should render message +1
            bottom_height = bottom_height.saturating_add(1);
        }

        bottom_height
    }

    /// get current document (row.width,doc.len)
    pub fn current_doc_size(&self, row: usize) -> (usize, usize) {
        match self.current() {
            None => (0, 0),
            Some(doc) => (doc.line_width(row), doc.len()),
        }
    }

    /// 获取当前document的高度
    pub fn current_doc_height(&self) -> usize {
        match self.current() {
            None => 0,
            Some(doc) => doc.len(),
        }
    }

    /// 获取指定行并转换为line
    pub fn current_doc_row_to_line(&self, index: usize) -> Line {
        match self.current() {
            None => Line::default(),
            Some(doc) => doc.line(index).to_line(),
        }
    }

    /// 获取指定行
    pub fn current_doc_row(&self, index: usize) -> RopeSlice {
        match self.current() {
            None => RopeSlice::from(""),
            Some(doc) => doc.line(index),
        }
    }

    /// 获取指定行的长度，根据字素簇边界分割
    pub fn get_row_width_split_by_word_boundary(&self, row: usize) -> usize {
        match self.current() {
            None => (0),
            Some(doc) => (doc.line_width(row)),
        }
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
        if self.empty {
            return;
        }

        self.index = (self.index + 1) % self.documents.len();
    }

    /// move to prev text.
    pub fn prev(&mut self) {
        if self.empty {
            return;
        }

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