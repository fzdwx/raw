use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

pub trait RopeSliceEx<'a> {
    /// repo slice to string
    fn get_string(&self) -> String;

    fn to_line(&self) -> Line;

    /// O(n) get len
    fn len_word_boundary(&self) -> usize;
}

impl<'a> RopeSliceEx<'a> for RopeSlice<'a> {
    fn get_string(&self) -> String {
        format!("{}", self)
    }

    fn to_line(&self) -> Line {
        let mut width = 0;
        let mut raw_width = 0;
        let mut str_list = Vec::new();
        let mut width_mapping = Vec::new();
        for str in self.get_string().graphemes(true) {
            str_list.push(str.to_string());
            let raw_len = str.len();
            width_mapping.push(width);
            width += raw_len;
            raw_width += raw_len;
        }

        Line {
            width_mapping,
            str_list,
            width,
        }
    }

    fn len_word_boundary(&self) -> usize {
        let mut width = 0;
        for _ in self.get_string().graphemes(true) {
            width += 1;
        }

        width
    }
}

#[derive(Default)]
pub struct Line {
    pub width_mapping: Vec<usize>,
    pub str_list: Vec<String>,
    pub width: usize,
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