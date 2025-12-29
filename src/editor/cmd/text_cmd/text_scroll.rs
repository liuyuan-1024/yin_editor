use crate::editor::ui::{EditArea, UI};

pub struct TextScroll {}

/// 文本跟随光标进行滚动
impl TextScroll {
    /// 滚动文本到编辑区域，若滚动偏移量发生改变则重绘编辑区域
    pub fn scroll_text(edit_area: &mut EditArea) {
        let row_changed = Self::scroll_vertically(edit_area);
        let col_changed = Self::scroll_horizontally(edit_area);
        if col_changed || row_changed {
            // todo 想办法不使用 edit_area.draw(0)，而是使用 editor.draw_edit_area()
            edit_area.draw(0);
        }
    }

    /// 纵向滚动文本，滚动到指定行
    fn scroll_vertically(edit_area: &mut EditArea) -> bool {
        let height = edit_area.size().height;
        let caret_line = edit_area.caret().line_idx;
        let offset = edit_area.mut_scroll_offset();

        // 可视区域范围：[offset.row, offset.row + height - 1]
        let start = offset.row;
        let end = offset.row.saturating_add(height).saturating_sub(1);

        // 判断目标行是否超出当前可视区域，决定是否更新偏移量
        let offset_changed = if caret_line < start {
            // 情况1：目标行在当前编辑区域第一行的**上方**，则可视区域上移到目标行
            offset.row = caret_line;
            true
        } else if caret_line > end {
            // 编辑区域最后一行的索引（左闭右开，所以实际最后一行是 offset.row + height - 1）
            // 情况2：目标行在当前可视区域最后一行的**下方**，则可视区域下移到目标行
            // 计算新偏移量：目标行 - 可视高度 + 1（让目标行显示在可视区域的最后一行）
            offset.row = caret_line.saturating_sub(height).saturating_add(1);
            true
        } else {
            // 情况3：目标行在当前可视区域内（无需调整偏移量）
            false
        };

        offset_changed
    }

    /// 横向滚动文本，滚动到指定列
    fn scroll_horizontally(edit_area: &mut EditArea) -> bool {
        let width = edit_area.size().width;
        let caret_line = edit_area.caret().line_idx;
        let caret_cell = edit_area.caret().cell_idx;
        // 光标在文档中的绝对列宽
        let caret_col = edit_area.line_cell_width_until(caret_line, caret_cell);

        let offset = edit_area.mut_scroll_offset();

        // 可视区域范围：[offset.col, offset.col + width - 1]
        let start = offset.col;
        let end = offset.col.saturating_add(width).saturating_sub(1);

        // 判断目标行是否超出当前可视区域，决定是否更新偏移量
        let offset_changed = if caret_col < start {
            // 情况1：目标行在当前可视区域第一行的**上方**，则可视区域上移到目标行
            offset.col = caret_col;
            true
        } else if caret_col >= end {
            // 情况2：目标行在当前可视区域最后一行的**下方**，则可视区域下移到目标行
            // 计算新偏移量：目标行 - 可视高度 + 1（让目标行显示在可视区域的最后一行）
            offset.col = caret_col.saturating_sub(width).saturating_add(1);
            true
        } else {
            // 情况3：目标行在当前可视区域内（无需调整偏移量）
            false
        };

        offset_changed
    }
}
