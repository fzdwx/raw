use std::fs;

use crate::row::Row;

/// the document
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    /// open document.
    pub fn open(filename: &str) -> Result<Document, std::io::Error> {
        let contents = fs::read_to_string(filename)?;

        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
        })
    }

    /// get row by index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// the document is empty?
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// the document rows length.
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}