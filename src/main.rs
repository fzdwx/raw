mod editor;
mod terminal;

use crate::editor::Editor;


fn main() {
    Editor::default().run();
}