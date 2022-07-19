pub mod banner;
pub mod text;

use std::io::Stdout;

use crate::app::{App, AppContext};
use crate::buffer::banner::BannerDocument;
use crate::buffer::text::TextDocument;
use tui::backend::CrosstermBackend;
use tui::Frame;

pub enum Buffer {
    Banner(BannerDocument),
    Text(TextDocument),
}

pub trait Buffered {
    fn name(&self) -> String;
    fn draw(&self, app: &mut AppContext, frame: &mut Frame<CrosstermBackend<Stdout>>);
}