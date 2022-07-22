use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

pub trait RopeSliceEx<'a> {
    /// repo slice to string
    fn get_string(&self) -> String;

    fn to_line(&self) -> Line;
}

impl<'a> RopeSliceEx<'a> for RopeSlice<'a> {
    fn get_string(&self) -> String {
        format!("{}", self)
    }

    fn to_line(&self) -> Line {
        let mut width = 1;
        let mut str_list = Vec::new();
        let mut width_mapping = Vec::new();
        for str in self.get_string().graphemes(true) {
            str_list.push(str.to_string());
            width_mapping.push(str.len());
            width += 1;
        }

        Line {
            width_mapping,
            str_list,
            width,
        }
    }
}

pub struct Line {
    width_mapping: Vec<usize>,
    str_list: Vec<String>,
    width: usize,
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
