use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    source: String,
    len: usize,
}

impl Row {
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
            source: splitted_row,
            len: splitted_length,
        }
    }

    /// find query in current row?
    pub fn find(&self, query: &str) -> Option<usize> {
        let matching_byte_index = self.source.find(query);
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                self.source[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    // 如果不加query.len 就是跳到第一个单词前面,
                    // 加了就是跳到最后
                    return Some(grapheme_index + query.len());
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

    /// render row start to end
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.source.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();

        for grapheme in self.source[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        Self {
            source: String::from(line),
            len: line.graphemes(true).count(),
        }
    }
}