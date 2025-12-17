mod editor;
mod file;
mod prelude;
mod terminal;
pub use terminal::Terminal;

use crate::editor::Editor;

fn main() {
    Editor::new().run();
}
