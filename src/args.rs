use std::env::args;

#[derive(Default)]
pub struct Args {
    pub filenames: Vec<String>,
}

impl Args {
    pub fn load() -> Self {
        let mut default = Self::default();
        let mut args = args().peekable().skip(1);

        while let Some(arg) = args.next() {
            default.filenames.push(arg)
        }

        default
    }
}