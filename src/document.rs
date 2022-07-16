use crate::row::Row;

/// the document
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    /// open document.
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("hello world"));
        rows.push(Row::from("hello world"));
        rows.push(Row::from("hello world"));
        Self { rows }
    }

    /// get row by index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// the document is empty?
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}