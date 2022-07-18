use raw::filetype::FileTypeChecker;

#[test]
fn test_is_xxx_file() {
    let f1 = "qwe.rs";
    let f2 = "qwe.Rs";
    let f3 = "qwe.RS";
    let f4 = "qwe.rS";

    assert_eq!(f1.is_xxx_file("rs"), true);
    assert_eq!(f2.is_xxx_file("rs"), true);
    assert_eq!(f3.is_xxx_file("rs"), true);
    assert_eq!(f4.is_xxx_file("rs"), true);
    assert_eq!(f4.is_xxx_file("go"), false);
}