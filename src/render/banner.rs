use crate::app::AppCtx;
use crate::extension::rope::RopeSliceEx;
use crate::render::Render;
use ropey::Rope;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Paragraph, Widget, Wrap};

/// banner (welcome message)
pub struct Banner {
    content: Rope,
}

impl Default for Banner {
    fn default() -> Self {
        let content = Rope::from(String::from_utf8(include_bytes!("../banner").to_vec()).unwrap());
        Self { content }
    }
}

impl Render for Banner {
    fn name(&self) -> String {
        "banner".to_string()
    }

    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .direction(Direction::Vertical)
            .split(area);

        let chunk = chunks[1];
        let contents: Vec<Paragraph> = self
            .content
            .lines()
            .map(|line| Paragraph::new(line.get_string()).alignment(Alignment::Center))
            .collect();

        let mut y = chunk.y;
        for p in contents {
            if y >= chunk.height {
                // check for overflow
                return;
            }

            p.render(
                Rect {
                    y,
                    height: 1,
                    ..chunk
                },
                buf,
            );
            y += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::extension::rope::RopeSliceEx;
    use crate::render::banner::Banner;
    use tui::layout::Alignment;
    use tui::widgets::Paragraph;

    #[test]
    fn test_1() {
        let banner = Banner::default();

        let contents: Vec<Paragraph> = banner
            .content
            .lines()
            .map(|line| Paragraph::new(line.get_string()).alignment(Alignment::Center))
            .collect();

        println!("{}", contents.len());
        println!("{:?}", contents)
    }
}
