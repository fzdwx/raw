use ropey::RopeSlice;
use tui::layout::Rect;

/// copy from (ropey author)[https://github.com/cessen/led/blob/master/src/graphemes.rs]
pub mod graphemes;
/// the extension for [`tui::layout::Rect`]
pub mod rect;
/// the extension for [`ropey::RopeSlice`]
pub mod rope;

mod tests {
    use crate::extension::rope::RopeSliceEx;
    use crate::render::document::Document;
    use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete, UnicodeSegmentation};
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn test_boundary() {
        let test = Document::open("./src/test").unwrap();
        let slice = test.line(0);

        let byte_idx = slice.char_to_byte(0);
        let (mut chunk, mut chunk_byte_idx, mut chunk_char_idx, _) = slice.chunk_at_byte(byte_idx);
        let mut cursor = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);
        match cursor.next_boundary(chunk, chunk_byte_idx) {
            Ok(Some(x)) => {}
            Err(e) => {}
            Err(e) => {}
        };
    }

    #[test]
    fn test_line_width() {
        let test = Document::open("./src/test").unwrap();
        let slice = test.line(0);

        // let chars = slice.as_str().unwrap().chars().skip(1).;

        let line = slice.to_line();
        println!("{}", slice.len_word_boundary());
        println!("{}", slice.slice(0..1)); // 1
        println!("{}", slice.slice(1..2)); // 好
        println!("{}", slice.slice(2..3)); // 👩
        println!("{}", slice.slice(3..4)); // 🔬
        println!("{}", slice.slice(4..7));

        println!("{}", line.width_mapping.get(0).unwrap());
        println!("{}", line.str_list.get(0).unwrap());
        println!("{}", line.width_mapping.get(1).unwrap());
        println!("{}", line.str_list.get(1).unwrap());
        println!("{}", line.width_mapping.get(2).unwrap());
        println!("{}", line.str_list.get(2).unwrap());
        println!("{}", line.width_mapping.get(3).unwrap());
        println!("{}", line.str_list.get(3).unwrap());
        println!("{}", line.width_mapping.get(4).unwrap());
        println!("{}", line.str_list.get(4).unwrap());
        println!("{}", line.to_string());
        // println!("{}", line.width);
        // println!(
        //     "{}",
        //     slice.slice(line.get_next_width(0)..line.get_next_width(1))
        // );
        // println!(
        //     "{}",
        //     slice.slice(line.get_next_width(1)..line.get_next_width(2))
        // );
        // println!("{}", line.get_next_width(2))
    }
}