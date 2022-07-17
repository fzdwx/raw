use crate::filetype::Name::{GoLang, Java, RUST};

pub struct FileType {
    name: String,
    highlight_opts: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

pub enum Name {
    RUST,
    GoLang,
    Java,
    Other,
}

impl Name {
    pub fn name(&self) -> String {
        match self {
            RUST => "Rust".to_string(),
            GoLang => "Golang".to_string(),
            Java => "Java".to_string(),
            Name::Other => "No FileType".to_string(),
        }
    }
}

impl FileType {
    /// create [FileType] by filename
    pub fn from(filename: &str) -> Self {
        if filename.is_empty() {
            return Self::default();
        }

        if filename.ends_with(".rs") {
            return Self {
                name: RUST.name(),
                highlight_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        } else if filename.ends_with(".go") {
            return Self {
                name: GoLang.name(),
                highlight_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    primary_keywords: vec![],
                    secondary_keywords: vec![],
                },
            };
        } else if filename.ends_with(".java") {
            return Self {
                name: Java.name(),
                highlight_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    primary_keywords: vec![],
                    secondary_keywords: vec![],
                },
            };
        } else {
            Self::default()
        }
    }

    /// get highlighting options
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.highlight_opts
    }

    /// get fileType name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl HighlightingOptions {
    /// get
    pub fn numbers(&self) -> bool {
        self.numbers
    }
    /// get
    pub fn strings(&self) -> bool {
        self.strings
    }
    /// get
    pub fn characters(&self) -> bool {
        self.characters
    }
    /// get
    pub fn comments(&self) -> bool {
        self.comments
    }
    /// get
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }
    /// get
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: Name::Other.name(),
            highlight_opts: HighlightingOptions::default(),
        }
    }
}