#[derive(Clone)]
pub struct FileType {
    name: String,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: "normal".to_string(),
        }
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}