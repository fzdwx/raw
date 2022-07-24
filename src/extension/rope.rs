use ropey::iter::Chunks;
use ropey::str_utils::byte_to_char_idx;
use ropey::RopeSlice;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

pub trait RopeSliceEx<'a> {
    /// repo slice to string
    fn get_string(&self) -> String;

    fn to_line(&self) -> Line;

    /// O(n) get len
    fn len_word_boundary(&self) -> usize;
}

#[derive(Default, Clone)]
pub struct Line {
    pub offset_mapping: Vec<usize>,
    pub str_list: Vec<String>,
    pub width: usize,
}

impl Line {
    /// 获取x在当前行中的开头的offset
    pub fn get_offset(&self, x: usize) -> usize {
        if let Some(offset) = self.offset_mapping.get(x) {
            *offset
        } else {
            // 当做最后一个字符处理
            if let Some(str) = self.str_list.get(x.saturating_sub(1)) {
                str.width() + self.get_offset(x.saturating_sub(1))
            } else {
                0
            }
        }
    }

    pub fn render(&self, offset: usize) -> String {
        if self.str_list.is_empty() {
            return "".to_string();
        }

        let mut result = String::new();
        for string in self.str_list.iter().skip(offset) {
            result.push_str(string);
        }
        result
    }
}

impl<'a> RopeSliceEx<'a> for RopeSlice<'a> {
    fn get_string(&self) -> String {
        format!("{}", self)
    }

    fn to_line(&self) -> Line {
        let mut width = 0;
        let mut str_list = Vec::new();
        let mut offset_mapping = Vec::new();
        for str in self.get_string().graphemes(true) {
            str_list.push(str.to_string());
            let raw_len = str.width();
            offset_mapping.push(width);
            width += raw_len;
        }

        Line {
            offset_mapping,
            str_list,
            width,
        }
    }

    /// 获取当前line 的长度，根据字素簇边界分割
    /// 1  => 1
    /// 你 => 1
    fn len_word_boundary(&self) -> usize {
        let mut width = 0;
        for _ in self.get_string().graphemes(true) {
            width += 1;
        }

        width
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        if self.str_list.is_empty() {
            return "".to_string();
        }

        let mut result = String::new();
        for string in self.str_list.iter() {
            result.push_str(string);
        }
        result
    }
}