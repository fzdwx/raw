use crate::buffer::text::Text;
use crate::tui::Tui;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn moved(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, text: Option<&Text>) {
        let (x, y) = self.adapter(frame, text);
        frame.set_cursor(x, y);
        Tui::move_to(x, y);
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn add_x(&mut self, amount: usize) {
        self.x = self.x.saturating_add(amount);
    }

    pub fn add_y(&mut self, amount: usize) {
        self.y = self.y.saturating_add(amount);
    }

    pub fn sub_x(&mut self, amount: usize) {
        self.x = self.x.saturating_sub(amount);
    }

    pub fn sub_y(&mut self, amount: usize) {
        self.y = self.y.saturating_sub(amount);
    }

    // pub fn move_to(&mut self, x: usize, y: usize, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    //     self.set(x, y);
    //     self.moved(frame);
    // }

    fn adapter(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        text: Option<&Text>,
    ) -> (u16, u16) {
        if self.y < 0 {
            self.y = 0
        }

        let Rect { x, y, .. } = frame.size();

        let mut x = if self.x != 0 && x != 0 {
            self.x % (x as usize)
        } else {
            self.x
        };

        let mut y = if self.y != 0 && y != 0 {
            self.y % (y as usize)
        } else {
            self.y
        };

        match text {
            None => {}
            Some(text) => {
                let text_len = text.len();
                if y > text_len {
                    y = text_len
                }

                let row_len = if let Some(row) = text.row(y) {
                    row.len()
                } else {
                    0
                };

                if x > row_len {
                    x = row_len
                }
            }
        }

        (x as u16, y as u16)
    }
}