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

    ///
    /// render
    ///
    /// # Arguments
    ///
    /// * `buf`: buffer to hold content
    /// * `area`: area that can be drawn
    ///
    /// # Examples
    ///
    /// ```
    ///use tui::buffer::Buffer;
    /// use tui::layout::Rect;
    /// use tui::style::Style;
    /// use raw::app::AppCtx;
    ///
    /// fn render(&self, ctx: AppCtx,buf: &mut Buffer, area: Rect) {
    ///        if self.is_empty() {
    ///            return;
    ///        }
    ///
    ///        let mut x = 0;
    ///        for line in self.content.lines() {
    ///            if x >= area.height {
    ///                return;
    ///            };
    ///
    ///            let string = format!("{}", line.slice(..).as_str().unwrap());
    ///            buf.set_string(0, x, string, Style::default());
    ///            x += 1;
    ///        }
    ///    }
    /// ```
    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect);
}
