use std::fs;

use crate::{
    editor::{Line, ui::UI},
    prelude::{CellIdx, DocumentCoordinate, LineIdx, RowIdx, Size, TerminalCoordinate},
    terminal::Terminal,
};

/// 编辑区
pub struct EditArea {
    // 编辑区域的尺寸
    size: Size,
    // 文档内容
    lines: Vec<Line>,
    // 文档是否被修改过
    is_modified: bool,
    // 光标在文档中的位置
    caret: DocumentCoordinate,
    // 滚动偏移量
    scroll_offset: TerminalCoordinate,
}

impl EditArea {
    /// 加载文档
    pub fn load(&mut self, file_name: &str) {
        let contents = fs::read_to_string(file_name).unwrap_or_default();

        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Line::from(line));
        }
        lines.push(Line::from(""));

        let dirty = false;

        let caret = DocumentCoordinate {
            line_idx: 0,
            cell_idx: 0,
        };

        self.lines = lines;
        self.is_modified = dirty;
        self.caret = caret;
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    /// 获取可变的行列表引用
    pub fn get_mut_lines(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }

    /// 获取行列表的引用
    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }

    /// 获取行列表的长度
    pub fn get_lines_count(&self) -> usize {
        self.lines.len()
    }

    /// 获取指定行的图元数量
    pub fn get_line_cell_count(&self, line_idx: LineIdx) -> usize {
        self.lines.get(line_idx).map_or(0, Line::get_cells_count)
    }

    /// 获取指定行的 [行首，指定索引位置) 的所有图元占据的终端列宽
    pub fn get_line_cell_width_until(&self, line_idx: LineIdx, cell_idx: CellIdx) -> usize {
        self.lines
            .get(line_idx)
            .map_or(0, |line| line.get_cell_width_until(cell_idx))
    }

    /// 获取光标所在的那一行的引用
    pub fn get_line_on_caret(&self) -> Option<&Line> {
        self.lines.get(self.caret.line_idx)
    }

    /// 获取光标所在的那一行的可变引用
    pub fn get_mut_line_on_caret(&mut self) -> Option<&mut Line> {
        self.lines.get_mut(self.caret.line_idx)
    }

    pub fn set_is_modified(&mut self, is_dirty: bool) {
        self.is_modified = is_dirty;
    }

    pub fn get_is_modified(&self) -> bool {
        self.is_modified
    }

    /// 获取光标在文档中的位置
    pub fn get_caret(&self) -> &DocumentCoordinate {
        &self.caret
    }

    pub fn set_caret(&mut self, position: DocumentCoordinate) {
        self.caret = position;
    }

    /// 光标的文档坐标转为终端坐标
    pub fn caret_to_terminal(&self) -> TerminalCoordinate {
        // 光标的文档位置
        let caret_line_idx = self.caret.line_idx;
        let caret_cell_idx = self.caret.cell_idx;
        // 光标在终端中的位置
        let caret_row_idx = caret_line_idx;
        let caret_col_idx = self.get_line_cell_width_until(caret_line_idx, caret_cell_idx);

        // 返回，光标在终端中相对的位置，即肉眼看到的位置
        TerminalCoordinate {
            row: caret_row_idx.saturating_sub(self.scroll_offset.row),
            col: caret_col_idx.saturating_sub(self.scroll_offset.col),
        }
    }

    /// 移除指定行，并返回指定行，返回一定不会为空
    pub fn remove_line(&mut self, line_idx: usize) -> Line {
        if line_idx >= self.lines.len() {
            return Line::default();
        }
        return self.lines.remove(line_idx);
    }

    pub fn get_scroll_offset(&self) -> &TerminalCoordinate {
        &self.scroll_offset
    }

    pub fn get_mut_scroll_offset(&mut self) -> &mut TerminalCoordinate {
        &mut self.scroll_offset
    }
}

impl UI for EditArea {
    fn resize(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, start_row: RowIdx) {
        if !self.lines.is_empty() {
            // 编辑区域的结束行
            let end_row = start_row.saturating_add(self.size.height);

            // 绘制编辑区域
            for current_row in start_row..end_row {
                let line_idx = current_row
                    .saturating_sub(start_row)
                    .saturating_add(self.scroll_offset.row);

                Terminal::print_row(
                    current_row,
                    &self.lines.get(line_idx).map_or("~".to_string(), |line| {
                        line.get_visible_substr(
                            self.scroll_offset.col,
                            self.scroll_offset.col.saturating_add(self.size.width),
                        )
                    }),
                );
            }
        }
    }
}

impl Default for EditArea {
    fn default() -> Self {
        Self {
            size: Size::default(),
            lines: Vec::new(),
            is_modified: false,
            caret: DocumentCoordinate::default(),
            scroll_offset: TerminalCoordinate::default(),
        }
    }
}
