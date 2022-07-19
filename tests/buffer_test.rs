use raw::buffer::text::{TextBuffer, TextBufferContainer};
use raw::buffer::Buffered;
use std::fs;

#[test]
fn test_empty_default() {
    let container = TextBufferContainer::default();
    assert_eq!(container.is_empty(), true)
}

#[test]
fn test_empty_add() {
    let mut container = TextBufferContainer::default();
    let test_1 = TextBuffer::open("qwe");

    container.add(test_1);

    assert_eq!(container.is_empty(), true)
}

#[test]
fn test_empty_add_text() {
    let mut container = TextBufferContainer::default();
    let test_1 = TextBuffer::open("./src/banner");
    container.add(test_1);

    assert_eq!(container.is_empty(), false)
}

#[test]
fn test_remove_and_add_and_size() {
    let mut container = TextBufferContainer::default();
    container.add(TextBuffer::open("./src/banner"));
    container.add(TextBuffer::open("./src/row.rs"));

    assert_eq!(container.size(), 2);

    assert_eq!(container.current().unwrap().name(), "./src/banner");
    container.remove_current();

    assert_eq!(container.current().unwrap().name(), "./src/row.rs");
    container.remove_current();

    assert_eq!(container.is_empty(), true);
    assert_eq!(container.size(), 0);
}

#[test]
fn test_move() {
    let mut container = TextBufferContainer::default();
    container.add(TextBuffer::open("./src/banner"));
    container.add(TextBuffer::open("./src/row.rs"));

    container.prev();

    assert_eq!(container.current().unwrap().name(), "./src/row.rs");
    assert_eq!(container.name(), "./src/row.rs");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
    assert_eq!(container.name(), "./src/banner");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/row.rs");
    assert_eq!(container.name(), "./src/row.rs");

    container.reset();
    assert_eq!(container.current().unwrap().name(), "./src/banner");

    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/row.rs");
    container.prev();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
}