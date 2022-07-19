pub mod banner;
pub mod statusline;
pub mod text;

use std::io::Stdout;

use crate::buffer::banner::Banner;
use crate::buffer::text::Text;
use tui::backend::CrosstermBackend;
use tui::Frame;

pub enum Buffer {
    Banner(Banner),
    Text(Text),
}

pub trait Buffered {
    fn name(&self) -> String;
    fn is_empty(&self) -> bool;
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>);
}