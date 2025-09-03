mod editor;
mod view;
mod buffer;

use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.start().unwrap();
}
