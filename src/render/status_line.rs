use crate::render::document::Document;
use crate::render::rect::Sub;
use crate::render::Render;
use crate::{DEFAULT_FILENAME, DEFAULT_FILETYPE};
use std::fmt::format;
use std::fs::FileType;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};

/// status line render.
pub struct StatusLine {
    filename: String,
    filetype: String,
    bg: Style,
    fg: Style,
    // todo position
}

impl StatusLine {
    pub fn refresh(&mut self, name: String, file_type: String) {
        self.filename = name;
        self.filetype = file_type;
    }

    fn render_bg(&self, buf: &mut Buffer, area: Rect) {
        buf.set_style(area, self.bg);
    }

    fn render_filename(&self, buf: &mut Buffer, area: Rect) {
        Paragraph::new(format!(" 📝 {}", self.filename))
            .style(self.fg)
            .alignment(Alignment::Left)
            .render(area, buf);
    }

    fn render_filetype(&self, buf: &mut Buffer, area: Rect) {
        Paragraph::new(self.filetype.as_str())
            .style(self.fg)
            .alignment(Alignment::Right)
            .render(area, buf);
    }
}

impl Render for StatusLine {
    fn name(&self) -> String {
        "status line".to_string()
    }

    fn render(&mut self, buf: &mut Buffer, area: Rect) {
        self.render_bg(buf, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(30),
            ])
            .split(area);

        self.render_filename(buf, chunks[0]);
        self.render_filetype(buf, chunks[2]);
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            filename: DEFAULT_FILENAME.to_string(),
            filetype: DEFAULT_FILETYPE.to_string(),
            bg: Style::default().bg(Color::Gray), // .bg(Color::Rgb(201, 123, 193)),
            fg: Style::default().fg(Color::Rgb(30, 30, 46)), //.fg(Color::Rgb(30, 30, 46)),
        }
    }
}