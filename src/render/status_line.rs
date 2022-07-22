use crate::app::AppCtx;
use crate::render::document::Document;
use crate::render::extend::RectEx;
use crate::render::Render;
use crate::{DEFAULT_FILENAME, DEFAULT_FILETYPE};
use std::fmt::format;
use std::fs::FileType;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

/// status line render.
pub struct StatusLine {
    filename: String,
    filetype: String,
    bg: Style,
    fg: Style,
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

    fn render_position(&self, buf: &mut Buffer, area: Rect, ctx: AppCtx) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        Paragraph::new(format!(
            "screen {}:{}",
            ctx.screen_size.0, ctx.screen_size.1
        ))
        .style(self.fg)
        .alignment(Alignment::Left)
        .render(chunks[0], buf);

        Paragraph::new(format!("doc size {}:{}", ctx.doc_size.1, ctx.doc_size.0))
            .style(self.fg)
            .alignment(Alignment::Center)
            .render(chunks[1], buf);

        Paragraph::new(format!("relative {}:{}", ctx.relative.y, ctx.relative.x))
            .style(self.fg)
            .alignment(Alignment::Center)
            .render(chunks[2], buf);

        Paragraph::new(format!("actual {}:{}", ctx.actual.y, ctx.actual.x))
            .style(self.fg)
            .alignment(Alignment::Right)
            .render(chunks[3], buf);
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

    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect) {
        self.render_bg(buf, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area);

        self.render_filename(buf, chunks[0]);
        self.render_position(buf, chunks[1], ctx);
        self.render_filetype(buf, chunks[2]);
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            filename: DEFAULT_FILENAME.to_string(),
            filetype: DEFAULT_FILETYPE.to_string(),
            bg: Style::default().bg(Color::Rgb(124, 252, 200)), // .bg(Color::Rgb(201, 123, 193)),
            fg: Style::default()
                .fg(Color::Rgb(30, 30, 46))
                .add_modifier(Modifier::BOLD), //.fg(Color::Rgb(30, 30, 46)),
        }
    }
}