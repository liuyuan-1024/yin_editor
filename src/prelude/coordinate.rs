//! 坐标

use crate::prelude::idx::{CellIdx, ColIdx, LineIdx, RowIdx};

/// 文档坐标，一般用来：
/// 标识光标在文档中的哪个位置
#[derive(Clone)]
pub struct DocumentCoordinate {
    pub line_idx: LineIdx,
    pub cell_idx: CellIdx,
}

impl Default for DocumentCoordinate {
    fn default() -> Self {
        Self {
            line_idx: 0,
            cell_idx: 0,
        }
    }
}

/// 终端坐标，一般用来：
/// 标识滚动偏移量
/// 标识光标在终端中的哪个位置
#[derive(Clone)]
pub struct TerminalCoordinate {
    pub row: RowIdx,
    pub col: ColIdx,
}

impl Default for TerminalCoordinate {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}
