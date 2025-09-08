mod editor;
mod view;
mod buffer;

use editor::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1);

    let mut editor = Editor::new(filename).unwrap();
    editor.run().unwrap();
}
