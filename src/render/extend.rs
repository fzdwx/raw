use ropey::RopeSlice;
use tui::layout::Rect;

pub trait RectEx {
    fn to_document(self) -> Rect;
    fn to_status_line(self) -> Rect;

    fn height_sub(self, amount: u16) -> Rect;
}

pub trait RopeSliceEx<'a> {
    fn get_string(&self) -> String;
}

impl<'a> RopeSliceEx<'a> for RopeSlice<'a> {
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}

impl RectEx for Rect {
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