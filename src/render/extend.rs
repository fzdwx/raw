use ropey::RopeSlice;
use tui::layout::Rect;

pub trait RectEx {
    fn to_document(self, should_render_message_bar: bool) -> Rect;
    fn to_status_line(self) -> Rect;
    fn to_message_bar(self) -> Rect;

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
    fn to_document(self, should_render_message_bar: bool) -> Rect {
        let mut amount = 1;

        if should_render_message_bar {
            amount += 1;
        }
        self.height_sub(amount)
    }

    fn to_status_line(self) -> Rect {
        Rect {
            y: self.height - 1,
            height: 1,
            ..self
        }
    }

    fn to_message_bar(self) -> Rect {
        Rect {
            y: self.height - 2,
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
