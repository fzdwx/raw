use crossterm::style::{Color, SetForegroundColor};
use unicode_segmentation::UnicodeSegmentation;

use crate::editor::SearchDirection;
use crate::highlighting;

#[derive(Default)]
pub struct Row {
    source: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
}

impl Row {
    /// render row start to end
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.source.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlight = &highlighting::Type::None;

        for (index, grapheme) in self.source[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlight_type = self
                    .highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Type::None);

                if current_highlight != highlight_type {
                    current_highlight = highlight_type;
                    result.push_str(
                        format!("{}", SetForegroundColor(highlight_type.to_color())).as_str(),
                    );
                };

                if c == '\t' {
                    result.push(' ');
                } else {
                    result.push(c)
                }
            }
        }
        result.push_str(format!("{}", SetForegroundColor(Color::Reset)).as_str());

        result
    }

    /// highlight current row
    pub fn highlight(&mut self, word: Option<&str>) {
        let mut h = Vec::new();
        let chars: Vec<char> = self.source.chars().collect();
        let mut matches = Vec::new();
        let mut search_index = 0;

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::FORWARD) {
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut index = 0;
        let mut prev_is_separator = true; // 为了正确的显示数字(要求数字前面有一个分隔符)
        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word[..].graphemes(true) {
                        index += 1;
                        h.push(highlighting::Type::Match)
                    }
                    continue;
                }
            }

            let prev_highlight = if index > 0 {
                #[allow(clippy::integer_arithmetic)]
                h.get(index - 1).unwrap_or(&highlighting::Type::None)
            } else {
                &highlighting::Type::None
            };

            if (c.is_ascii_digit()
                // 前面是分隔符或前面是数字
                && (prev_is_separator || prev_highlight == &highlighting::Type::Number))
                // 支持小数
                || (c == &'.' && prev_highlight == &highlighting::Type::Number)
            {
                h.push(highlighting::Type::Number)
            } else {
                h.push(highlighting::Type::None)
            }

            prev_is_separator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
            index += 1;
        }

        self.highlighting = h;
    }

    /// insert char at target index
    pub fn insert(&mut self, at: usize, c: char) {
        // 在最后插入
        if at >= self.len {
            self.source.push(c);
            self.len += 1;
            return;
        }

        // 在中间某个地方插入
        let mut result: String = String::new();
        let mut length = 0;
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.source[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.source = result;
    }

    /// delete char from target index
    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        };

        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.source[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.source = result;
    }

    /// concat row
    pub fn concat(&mut self, other: &Row) {
        self.source = format!("{}{}", self.source, other.source);
        self.len += other.len;
    }

    /// split current row
    pub fn split(&mut self, at: usize) -> Row {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.source[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.source = row;
        self.len = length;
        Self {
            highlighting: Vec::new(),
            source: splitted_row,
            len: splitted_length,
        }
    }

    /// find query in current row?
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }

        let start = if direction == SearchDirection::FORWARD {
            at
        } else {
            0
        };

        let end = if direction == SearchDirection::FORWARD {
            self.len
        } else {
            at
        };

        let sub_string: String = self.source[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        let matching_byte_index = if direction == SearchDirection::FORWARD {
            sub_string.find(query)
        } else {
            sub_string.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                sub_string[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(grapheme_index + start);
                }
            }
        }

        None
    }
    /// to bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    /// the row length
    pub fn len(&self) -> usize {
        self.len
    }

    /// the row is empty?
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        Self {
            source: String::from(line),
            len: line.graphemes(true).count(),
            highlighting: Vec::new(),
        }
    }
}