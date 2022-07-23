use ropey::{Rope, RopeSlice};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::app::{AppCtx, AppResult};
use crate::extension::rope::RopeSliceEx;
use crate::render::Render;
use crate::{DEFAULT_FILENAME, DEFAULT_FILETYPE};

/// the document
#[derive(Debug)]
pub struct Document {
    pub content: Rope,
    name: String,
    filetype: String,
}

impl Render for Document {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect) {
        if self.is_empty() {
            return;
        }

        for (x, line) in self
            .content
            .lines()
            .skip(ctx.cal_offset_y())
            .take(area.height as usize)
            .enumerate()
        {
            if x >= area.height as usize {
                return;
            };

            // todo 是不是太暴力了.
            buf.set_string(0, x as u16, line.to_line().to_string(), Style::default());
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
        if index >= self.len() {
            return 0;
        }

        let slice = self.content.line(index);

        let line_len = slice.len_word_boundary();

        // 如果这一行没有数据，或者没有换行,直接返回usize
        let lines_count = slice.len_lines();
        if line_len == 0 || lines_count == 0 || lines_count == 1 {
            return line_len;
        }

        line_len - 1
    }

    /// 获取当前行的长度,使用width获取
    /// 1  => 1
    /// 你 => 2
    pub fn line_width(&self, index: usize) -> usize {
        if index >= self.len() {
            return 0;
        }

        let slice = self.content.line(index);
        let mut raw_width = 0;
        for str in slice.get_string().graphemes(true) {
            raw_width += str.width();
        }
        raw_width
    }

    /// get line by index.
    pub fn line(&self, index: usize) -> RopeSlice {
        if let Some(line) = self.content.get_line(index) {
            line
        } else {
            RopeSlice::from("")
        }
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