mod document;
mod editor;
mod row;
mod terminal;

use crate::editor::Editor;

fn main() {
    Editor::default().run();
}