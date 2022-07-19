use crossterm::cursor::position;
use crossterm::style::Stylize;
use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::Widget;
use tui::Frame;

use crate::buffer::text::Text;
use crate::buffer::{get_text_and_status_layout, Buffered};
use crate::filetype::FileType;

pub struct StatusLine {
    filename: String,
    filetype: FileType,
    bg: Style,
    fg: Style,
}

impl StatusLine {
    pub fn refresh(&mut self, text: Option<&Text>) {
        match text {
            None => {}
            Some(text) => {
                self.filename = text.name();
                self.filetype = text.filetype();
            }
        }
    }

    fn get_layout(&self, area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
            ])
            .split(area)
    }

    fn render_bg(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.bg);
    }

    pub fn render_filename(&self, area: Rect, buf: &mut Buffer) {
        let span = Span::styled(self.filename.as_str(), self.fg);
        buf.set_span(area.x, area.y, &span, span.width() as u16);
    }

    pub fn render_position(&self, area: Rect, buf: &mut Buffer) {
        let (x, y) = position().unwrap();
        let position = format!("{}:{}", y, x).to_string();
        let span = Span::styled(position, self.fg);
        buf.set_span(area.right(), area.y, &span, span.width() as u16);
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            filename: "".to_string(),
            filetype: FileType::default(),
            bg: Style::default().bg(Color::Rgb(201, 123, 193)),
            fg: Style::default().fg(Color::Rgb(30, 30, 46)),
        }
    }
}

impl Widget for &StatusLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_bg(area, buf);

        let layout = self.get_layout(area);

        self.render_filename(layout[0], buf);
        self.render_position(layout[1], buf);

        // buf.set_string(
        //     area.x,
        //     area.y,
        //     format!(
        //         "{}          {:?}  {} ({},{})",
        //         self.filename,
        //         area,
        //         self.filetype.to_string(),
        //         y,
        //         x,
        //     ),
        //     Style::default(),
        // );
    }
}

impl Buffered for StatusLine {
    fn name(&self) -> String {
        "status_line".to_string()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        frame.render_widget(self, get_text_and_status_layout(frame.size())[1]);
    }
}