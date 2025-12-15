mod edit_area;
pub use edit_area::EditArea;

use crate::prelude::{RowIdx, Size};

/// 所有UI组件都必须实现 UI trait，不然无法渲染该组件
/// UI组件应该有的字段：size
pub trait UI {
    /// 更改UI组件的大小
    fn resize(&mut self, size: Size);

    /// 从可视区域的指定行开始绘制UI组件
    fn draw(&mut self, start_row: RowIdx);
}
