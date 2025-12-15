use crate::editor::Editor;

mod editor;
mod prelude;
mod terminal;

fn main() {
    Editor::new().run();
}
