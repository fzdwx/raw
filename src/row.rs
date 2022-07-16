use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    source: String,
    len: usize,
}

impl Row {
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len {
            self.source.push(c);
        } else {
            let mut result: String = self.source[..].graphemes(true).take(at).collect();
            let remainder: String = self.source[..].graphemes(true).take(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self.source = result
        }

        self.update_len();
    }

    /// refresh row length
    pub fn update_len(&mut self) {
        self.len = self.source[..].graphemes(true).count()
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
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        let mut row = Self {
            source: String::from(line),
            len: 0,
        };

        row.update_len();

        row
    }
}