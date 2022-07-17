use crate::filetype::Name::{GoLang, Java, RUST};

pub struct FileType {
    name: String,
    highlight_opts: HighlightingOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
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
                },
            };
        } else if filename.ends_with(".go") {
            return Self {
                name: GoLang.name(),
                highlight_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                },
            };
        } else if filename.ends_with(".java") {
            return Self {
                name: Java.name(),
                highlight_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                },
            };
        } else {
            Self::default()
        }
    }

    /// get highlighting options
    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.highlight_opts
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
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: Name::Other.name(),
            highlight_opts: HighlightingOptions::default(),
        }
    }
}