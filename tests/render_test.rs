use raw::render::document::Document;
use raw::render::switcher::DocumentSwitcher;
use raw::render::Render;

#[test]
fn test_license() {
    let license = Document::open("./scr/../LICENSE").unwrap();
    println!("size: {}", license.line(21).len_bytes());
    println!("content: {}", license.line(21));

    println!("==========");

    println!("size: {}", license.line(20).len_bytes());
    println!("content: {}", license.line(20));
}

#[test]
fn test_doc() {
    let banner = Document::open("./src/banner").unwrap();
    println!("{:?}", banner.content.line(1));
}

#[test]
fn test_render() {
    let render = Document::open("./src/render.rs").unwrap();
    for line in render.content.lines() {
        print!("{}", format!("{}", line));
    }
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
    container.add(Document::open("./src/").unwrap());

    assert_eq!(container.size(), 2);

    assert_eq!(container.current().unwrap().name(), "./src/banner");
    container.remove_current();

    assert_eq!(container.current().unwrap().name(), "./src/");
    container.remove_current();

    assert_eq!(container.is_empty(), true);
    assert_eq!(container.size(), 0);
}

#[test]
fn test_move() {
    let mut container = DocumentSwitcher::default();
    container.add(Document::open("./src/banner").unwrap());
    container.add(Document::open("./src/").unwrap());

    container.prev();

    assert_eq!(container.current().unwrap().name(), "./src/");
    assert_eq!(container.name(), "./src/");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
    assert_eq!(container.name(), "./src/banner");
    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/");
    assert_eq!(container.name(), "./src/");

    container.reset();
    assert_eq!(container.current().unwrap().name(), "./src/banner");

    container.next();
    assert_eq!(container.current().unwrap().name(), "./src/");
    container.prev();
    assert_eq!(container.current().unwrap().name(), "./src/banner");
}