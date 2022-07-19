use std::fs;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::Widget;
use tui::Frame;

use crate::buffer::Buffered;
use crate::row::Row;

/// only load banner.
pub struct Banner {
    rows: Vec<Row>,
    name: String,
    max_size: usize,
}

impl Banner {
    pub fn default() -> Self {
        let contents = String::from_utf8(include_bytes!("../banner").to_vec()).unwrap();

        let mut rows = Vec::new();
        let mut max_size: usize = 0;
        for line in contents.lines() {
            if line.len() > max_size {
                max_size = line.len();
            }

            rows.push(Row::from(line))
        }

        Self {
            rows,
            name: "banner".to_string(),
            max_size,
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}

impl Buffered for Banner {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let vec = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.size());

        /// see impl Widget for &BannerDocument#render#render
        frame.render_widget(self, vec[1]);
    }
}

impl Widget for &Banner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut width = area.width as usize;
        let start = area.x as usize;
        let end = width.saturating_add(area.x as usize);
        let padding = width.saturating_sub(self.max_size) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1) as usize);
        let y = area.y;

        let len = self.rows.len();
        for i in 0..len {
            if let Some(row) = self.row(i) {
                let mut row = row.render(start, end);
                row = format!("{}{}", spaces, row);

                if row.len() > width {
                    while width > 0 {
                        if row.is_char_boundary(width) {
                            row.truncate(width);
                            break;
                        }
                        width -= 1;
                    }
                }
                buf.set_string(area.left(), (i as u16) + y, row.as_str(), Style::default());
            }
        }
    }
}