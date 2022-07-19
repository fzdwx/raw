pub mod banner;
pub mod text;

use std::io::Stdout;

use crate::buffer::banner::BannerBuffer;
use crate::buffer::text::TextBuffer;
use tui::backend::CrosstermBackend;
use tui::Frame;

pub enum Buffer {
    Banner(BannerBuffer),
    Text(TextBuffer),
}

pub trait Buffered {
    fn name(&self) -> String;
    fn is_empty(&self) -> bool;
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>);
}