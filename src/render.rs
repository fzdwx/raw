use crate::app::AppCtx;
use tui::buffer::Buffer;
use tui::layout::Rect;

pub mod banner;
pub mod document;
pub mod message;
pub mod status_line;
pub mod switcher;

pub trait Render {
    fn name(&self) -> String;
    
    /// * `buf`: buffer to hold content
    /// * `area`: area that can be drawn
    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect);
}
