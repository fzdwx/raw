pub struct Row {
    string: String,
}

impl Row {
    pub(crate) fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start = std::cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        Self {
            string: String::from(line),
        }
    }
}