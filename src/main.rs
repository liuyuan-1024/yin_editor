use crate::editor::Editor;

mod editor;
mod file;
mod prelude;
mod terminal;

fn main() {
    Editor::new().run();
}
