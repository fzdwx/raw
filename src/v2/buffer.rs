pub mod banner;
pub mod statusline;
pub mod text;

use std::io::Stdout;

use crate::buffer::banner::Banner;
use crate::buffer::statusline::StatusLine;
use crate::buffer::text::{Text, TextBufferContainer};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget;
use tui::Frame;

pub enum Buffer {
    Banner(Banner),
    Text(Text),
    TextBufferContainer(TextBufferContainer),
    StatusLine(StatusLine),
}

pub trait Buffered {
    fn name(&self) -> String;
    fn is_empty(&self) -> bool;
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>);
}

pub fn get_text_and_status_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(99), Constraint::Percentage(1)])
        .split(area)
}