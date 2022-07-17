use std::fs;
use std::io::Error;
use std::io::Write;

use crate::editor::SearchDirection;
use crate::filetype::FileType;
use crate::row::Row;
use crate::terminal::Position;

/// the document
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    dirty: bool,
    file_type: FileType,
    pub filename: Option<String>,
}

impl Document {
    /// open document.
    pub fn open(filename: &str) -> Result<Document, Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        let mut rows = Vec::new();

        for line in contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
            dirty: false,
            file_type,
        })
    }

    /// just set filename
    pub fn with_file_name(filename: &str) -> Document {
        Self {
            rows: Vec::new(),
            filename: Some(filename.to_string()),
            dirty: false,
            file_type: FileType::from(filename),
        }
    }

    /// from string
    pub fn with_string(str: String) -> Document {
        let mut rows = Vec::new();
        for line in str.lines() {
            rows.push(Row::from(line))
        }
        Self {
            rows,
            dirty: false,
            file_type: FileType::default(),
            filename: None,
        }
    }

    /// save file to disk
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            self.file_type = FileType::from(filename);

            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            // reset dirty
            self.dirty = false;
        }

        Ok(())
    }

    /// highlight current document
    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;
        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };

        for row in &mut self.rows[..until] {
            start_with_comment = row.highlight(
                self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }

    /// insert char to position
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }

        self.dirty = true;
        if c == '\n' {
            self.insert_new_line(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
        self.unhighlight_rows(at.y);
    }

    /// delete char from position
    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
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
            row.delete(at.x);
        }

        self.unhighlight_rows(at.y);
    }

    /// find str position
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }

                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }

        None
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    /// get row by index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// get current document file type
    pub fn file_type(&self) -> String {
        self.file_type.name()
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
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }
}