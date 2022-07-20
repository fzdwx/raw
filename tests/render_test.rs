use raw::render::text::{Document, DocumentSwitcher};

#[test]
fn test_doc() {
    let banner = Document::open("./src/banner").unwrap();
    println!("{:?}", banner.content.lines());
}

#[test]
fn test_empty_default() {
    let container = DocumentSwitcher::default();
    assert_eq!(container.is_empty(), true)
}

#[test]
fn test_empty_add() {
    let mut container = DocumentSwitcher::default();

    container.add(Document::default());

    assert_eq!(container.is_empty(), true)
}

#[test]
fn test_empty_add_text() {
    let mut container = DocumentSwitcher::default();
    let test_1 = Document::open("./src/banner");
    container.add(test_1.unwrap());

    assert_eq!(container.is_empty(), false)
}

#[test]
fn test_remove_and_add_and_size() {
    let mut container = DocumentSwitcher::default();
    container.add(Document::open("./src/banner").unwrap());
    container.add(Document::open("./src/screen.rs").unwrap());

    assert_eq!(container.size(), 2);

    assert_eq!(container.current().unwrap().name(), "./src/banner");
    container.remove_current();

    assert_eq!(container.current().unwrap().name(), "./src/screen.rs");
    container.remove_current();

    assert_eq!(container.is_empty(), true);
    assert_eq!(container.size(), 0);
}

#[test]
fn test_move() {
    let mut container = DocumentSwitcher::default();
    container.add(Document::open("./src/banner").unwrap());
    container.add(Document::open("./src/screen.rs").unwrap());

    container.prev();

    assert_eq!(container.current().unwrap().name(), "./src/screen.rs");
    assert_eq!(container.name(), "./src/screen.rs");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
    assert_eq!(container.name(), "./src/banner");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/screen.rs");
    assert_eq!(container.name(), "./src/screen.rs");

    container.reset();
    assert_eq!(container.current().unwrap().name(), "./src/banner");

    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/screen.rs");
    container.prev();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
}