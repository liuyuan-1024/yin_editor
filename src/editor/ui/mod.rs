mod cmd_line;
mod edit_area;
mod status_bar;
pub use cmd_line::CmdLine;
pub use edit_area::EditArea;
pub use status_bar::StatusBar;

use crate::prelude::Size;

/// 所有UI组件都必须实现 UI trait，不然无法渲染该组件
/// UI组件应该有的字段：size
pub trait UI {
    /// 更改UI组件的大小
    fn resize(&mut self, size: Size);

    /// 绘制UI组件
    fn draw(&mut self, start_row: usize);
}
