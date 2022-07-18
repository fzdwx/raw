use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    source: String,
    len: usize,
}

impl Row {
    pub fn new(source: String) -> Self {
        Self {
            len: source.len(),
            source,
        }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.source.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        for (_, grapheme) in self.source[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                if c == '\t' {
                    result.push(' ');
                } else {
                    result.push(c);
                }
            }
        }
        result
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        Row::new(line.to_string())
    }
}

impl From<String> for Row {
    fn from(line: String) -> Self {
        Row::new(line)
    }
}