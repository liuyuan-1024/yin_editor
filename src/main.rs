mod editor;
mod file;
mod prelude;
mod terminal;
use std::env;

pub use terminal::Terminal;

use crate::editor::Editor;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件路径参数
    if let Some(file_path) = args.get(1) {
        Editor::new(file_path).run();
    };
}
