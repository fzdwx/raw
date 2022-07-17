use crossterm::style::{Color, SetForegroundColor};
use unicode_segmentation::UnicodeSegmentation;

use crate::editor::SearchDirection;
use crate::filetype::HighlightingOptions;
use crate::highlighting;
use crate::highlighting::Type;

#[derive(Default)]
pub struct Row {
    source: String,
    len: usize,
    highlighting: Vec<Type>,
    pub is_highlighted: bool,
}

impl Row {
    /// render row start to end
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.source.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlight = &Type::None;

        for (index, grapheme) in self.source[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlight_type = self.highlighting.get(index).unwrap_or(&Type::None);

                if current_highlight != highlight_type {
                    current_highlight = highlight_type;
                    result.push_str(
                        format!("{}", SetForegroundColor(highlight_type.to_color())).as_str(),
                    );
                };

                if c == '\t' {
                    result.push(' ');
                } else {
                    result.push(c);
                }
            }
        }
        result.push_str(format!("{}", SetForegroundColor(Color::Reset)).as_str());

        result
    }

    /// highlight current row
    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.source.chars().collect();
        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == Type::MultilineComment
                    && self.source.len() > 1
                    && self.source[self.source.len() - 2..] == *"*/"
                {
                    return true;
                };
            }
            return false;
        }

        self.highlighting = Vec::new();
        let mut index = 0;

        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.source.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(Type::MultilineComment);
            }
            index = closing_index;
        }

        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, opts, *c, &chars) {
                in_ml_comment = true;
                continue;
            }

            in_ml_comment = false;
            if self.highlight_char(&mut index, opts, *c, &chars)
                || self.highlight_comment(&mut index, opts, *c, &chars)
                || self.highlight_primary_keywords(&mut index, opts, &chars)
                || self.highlight_secondary_keywords(&mut index, opts, &chars)
                || self.highlight_string(&mut index, opts, *c, &chars)
                || self.highlight_number(&mut index, opts, *c, &chars)
            {
                continue;
            }

            self.highlighting.push(Type::None);
            index += 1;
        }

        self.highlight_match(word);

        if in_ml_comment && &self.source[self.source.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
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
        self.is_highlighted = false;
        Self {
            highlighting: Vec::new(),
            source: splitted_row,
            len: splitted_length,
            is_highlighted: false,
        }
    }

    /// find query in current row?
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        let sub_string: String = self.source[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        let matching_byte_index = if direction == SearchDirection::Forward {
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

    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.characters() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };

                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlighting.push(highlighting::Type::Character);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                        *index += 1;
                    }
                    return true;
                }
            }
        };
        false
    }
    #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.multiline_comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.source[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(Type::MultilineComment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        false
    }
    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.strings() && c == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *index += 1;

                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.highlighting.push(highlighting::Type::String);
            *index += 1;

            return true;
        };

        false
    }
    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
                let prev_char = chars[*index - 1];
                if !is_separator(prev_char) {
                    return false;
                }
            }

            loop {
                self.highlighting.push(highlighting::Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        };
        false
    }
    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(index, chars, opts.primary_keywords(), Type::PrimaryKeywords)
    }
    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            Type::SecondaryKeywords,
        )
    }
    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: Type,
    ) -> bool {
        if *index > 0 {
            #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }

        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars[*index + word.len()];
                if !is_separator(next_char) {
                    continue;
                }
            }

            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    /// highlight str
    fn highlight_str(
        &mut self,
        index: &mut usize,
        str: &str,
        chars: &[char],
        hl_type: Type,
    ) -> bool {
        if str.is_empty() {
            return false;
        };

        for (keyword_idx, c) in str.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(keyword_idx)) {
                if *next_char != c {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..str.len() {
            self.highlighting.push(hl_type);
            *index += 1;
        }

        true
    }
    // highlight match word
    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }

            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    for i in search_match..next_index {
                        self.highlighting[i] = Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        Self {
            source: String::from(line),
            len: line.graphemes(true).count(),
            highlighting: Vec::new(),
            is_highlighted: false,
        }
    }
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}