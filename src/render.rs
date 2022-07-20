use tui::buffer::Buffer;
use tui::layout::Rect;

pub mod document;
pub mod rect;
pub mod status_line;
pub mod switcher;

pub trait Render {
    fn draw(&self, buf: &mut Buffer, area: Rect);
}