use std::fmt;

use crate::{
    editor::Cell,
    prelude::{CellIdx, ColIdx},
};

/// 终端在渲染文档时，是以 Line 为单位的
pub struct Line {
    cells: Vec<Cell>,
}

impl Line {
    pub fn from(str: &str) -> Self {
        let cells = Cell::str_to_cells(str);

        let mut string = String::new();
        for cell in &cells {
            string.push_str(cell.to_string());
        }

        Self { cells }
    }

    /// 获取可见子串
    pub fn get_visible_substr(&self, start: ColIdx, end: ColIdx) -> String {
        let mut result = String::new();

        if start >= end || start > self.get_cell_width_until(self.get_cells_count()) {
            return result;
        }

        let start = start.max(0);
        let end = end.min(self.get_cell_width_until(self.get_cells_count()));

        let mut cumulative_width: usize = 0;
        for cell in &self.cells {
            let cell_width = cell.get_cell_width();
            let cell_start = cumulative_width;
            let cell_end = cumulative_width.saturating_add(cell_width);

            // 调整包含条件：只要单元格与可视区域有交集就包含
            if cell_start < end && cell_end > start {
                result.push_str(cell.to_string());
            }
            cumulative_width = cell_end;

            // 超出可视区域后提前退出
            if cumulative_width >= end {
                break;
            }
        }

        result
    }

    /// 计算 [行首，指定索引) 的所有图元，占据的终端列宽
    pub fn get_cell_width_until(&self, cell_idx: CellIdx) -> usize {
        // 保证指定索引不会大于行的图元个数
        let cell_idx = cell_idx.min(self.cells.len());

        self.cells[..cell_idx]
            .iter()
            .map(|cell| cell.get_cell_width())
            .sum()
    }

    /// 一行中所有图元的数量
    pub fn get_cells_count(&self) -> usize {
        self.cells.len()
    }

    /// 插入一个图元到行中指定位置
    pub fn insert_cell(&mut self, cell: Cell, cell_idx: CellIdx) {
        self.cells.insert(cell_idx, cell);
    }

    /// 删除一个行中指定位置的图元
    pub fn delete_cell(&mut self, cell_idx: CellIdx) {
        self.cells.remove(cell_idx);
    }

    /// 从指定图元索引位置，将行拆分为两个图元向量
    pub fn split(&mut self, at: CellIdx) -> (Line, Line) {
        let head = Line {
            cells: self.cells[..at].to_vec(),
        };
        let tail = Line {
            cells: self.cells[at..].to_vec(),
        };
        (head, tail)
    }

    /// 合并两个行，将其他行的图元插入到当前行的末尾
    pub fn merge(&mut self, other: Line) {
        self.cells.extend(other.cells);
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let string = self
            .cells
            .iter()
            .map(|cell| cell.to_grapheme().to_string())
            .collect::<String>();

        write!(formatter, "{}", string)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self { cells: Vec::new() }
    }
}
