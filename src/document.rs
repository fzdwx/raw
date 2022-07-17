use std::fs;
use std::io::{Error, Write};

use crate::row::Row;
use crate::terminal::Position;

/// the document
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
    dirty: bool,
}

impl Document {
    /// open document.
    pub fn open(filename: &str) -> Result<Document, Error> {
        let contents = fs::read_to_string(filename)?;

        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
            dirty: false,
        })
    }

    pub fn of(filename: &str) -> Document {
        Self {
            rows: Vec::new(),
            filename: Some(filename.to_string()),
            dirty: false,
        }
    }

    /// save file to disk
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            // reset dirty
            self.dirty = false;
        }

        Ok(())
    }

    /// insert char to position
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }

        self.dirty = true;
        if c == '\n' {
            self.insert_new_line(at);
            return;
        };

        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }

    /// delete char from position
    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        };

        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.concat(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x)
        }
    }

    /// get row by index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
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
        if at.y > self.rows.len() {
            return;
        }

        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }

        // cut somewhere in a row
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[at.y].split(at.x);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }
}