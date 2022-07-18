#![warn(clippy::all)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
extern crate core;

use raw::app::App;
use raw::terminal::Terminal;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use tui::widgets::{Block, Borders};

fn main() {
    App::new().run()
}