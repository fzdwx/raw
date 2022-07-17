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

use crate::editor::Editor;

mod document;
mod editor;
mod row;
mod terminal;

fn main() {
    Editor::default().run();
}