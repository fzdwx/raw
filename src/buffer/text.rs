use crate::app::{App, AppContext};
use crate::buffer::Buffered;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;

pub struct TextDocument {}

impl Buffered for TextDocument {
    fn name(&self) -> String {
        todo!()
    }

    fn draw(&self, app: &mut AppContext, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        todo!()
    }
}