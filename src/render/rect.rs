use tui::layout::Rect;

pub trait Sub {
    fn to_document(self) -> Rect;
    fn to_status_line(self) -> Rect;

    fn height_sub(self, amount: u16) -> Rect;
}

impl Sub for Rect {
    fn to_document(self) -> Rect {
        self.height_sub(1)
    }

    fn to_status_line(self) -> Rect {
        Rect {
            y: self.height - 1,
            height: 1,
            ..self
        }
    }

    fn height_sub(self, amount: u16) -> Rect {
        Rect {
            height: self.height.saturating_sub(amount),
            ..self
        }
    }
}