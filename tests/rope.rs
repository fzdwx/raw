use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use raw::extension::rope::RopeSliceEx;
use raw::render::document::Document;

#[test]
fn test_len_word_boundary(){
    let doc = Document::open("LICENSE").unwrap();
    let slice = doc.content.line(21);
    println!("{}", slice.len_chars());
    println!("{}", slice.len_word_boundary());
    
    
    let x = slice.char(1);
    println!("{}", x);
    println!("{:?}", x.width());
    
    println!("{:?}", slice)
}

#[test]
fn test_2(){
    let doc = Document::open("LICENSE").unwrap();
    let slice = doc.content.line(21);
    let line = slice.to_line();
    println!("{}", line.offset_mapping[0]);
    println!("str:{},width:{}", line.str_list[0],line.str_list[0].width());
    println!("{}", line.offset_mapping[1]);
    println!("str:{},width:{}", line.str_list[1],line.str_list[1].width());
    println!("{}", line.offset_mapping[2]);
    println!("str:{},width:{}", line.str_list[2],line.str_list[2].width());
}