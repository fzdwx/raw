use std::fs;

use crate::row::Row;
use crate::terminal::Position;

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

    /// insert char to position
    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' {
            self.insert_new_line(at);
            return;
        };

        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    /// delete char from position
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        };

        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.concat(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x)
        }
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

    /// insert new line to position
    fn insert_new_line(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }

        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        // cut somewhere in a row
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }
}