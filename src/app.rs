use std::error;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

pub struct App {
    running: bool,
}

impl App {
    pub fn run(&self) {
        while self.running {}
    }
}

impl Default for App {
    fn default() -> Self {
        Self { running: false }
    }
}