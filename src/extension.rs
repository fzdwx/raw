use ropey::RopeSlice;
use tui::layout::Rect;

/// the extension for [`tui::layout::Rect`]
pub mod rect;
/// the extension for [`ropey::RopeSlice`]
pub mod rope;

mod tests {
    use crate::extension::rope::RopeSliceEx;
    use crate::render::document::Document;
    use unicode_segmentation::UnicodeSegmentation;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn test_line_width() {
        let test = Document::open("./src/test").unwrap();
        let slice = test.line(0);

        let line = slice.to_line();
        println!("{}", line.to_string());
        // println!("{}", graphemes.next().unwrap());
        // println!("{}", graphemes.next().unwrap());
        // println!("{}", graphemes.next().unwrap());
        // println!("{}", graphemes.next().unwrap());
        // println!("{}", graphemes.next().unwrap());
        // println!("{}", string);
    }
}
