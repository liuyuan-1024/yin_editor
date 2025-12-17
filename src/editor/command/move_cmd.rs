use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{Editor, UI, command::Cmd, ui::EditArea},
    prelude::DocumentCoordinate,
    terminal::Terminal,
};

pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
}

/// 移动的规则：移动函数会计算出光标的“目标位置”，然后将目标位置传递给“校验函数”；
/// 校验函数会将目标位置调整为合理的位置
impl Move {
    // 光标向上移动
    fn caret_up(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: line_idx.saturating_sub(1),
                cell_idx: cell_idx,
            },
        );
    }

    // 光标向下移动
    fn caret_down(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: line_idx.saturating_add(1),
                cell_idx: cell_idx,
            },
        );
    }

    // 光标向左移动
    fn caret_left(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        if cell_idx > 0 {
            Self::move_caret_validly(
                edit_area,
                DocumentCoordinate {
                    line_idx,
                    cell_idx: cell_idx.saturating_sub(1),
                },
            );
        } else if line_idx > 0 {
            Self::caret_up(edit_area);
            Self::caret_end(edit_area);
        }
    }

    // 光标向右移动
    fn caret_right(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        let cell_count = edit_area.get_line_cell_count(line_idx);

        if cell_idx < cell_count {
            Self::move_caret_validly(
                edit_area,
                DocumentCoordinate {
                    line_idx,
                    cell_idx: cell_idx.saturating_add(1),
                },
            );
        } else if cell_idx == cell_count && line_idx < edit_area.lines_len().saturating_sub(1) {
            Self::caret_down(edit_area);
            Self::caret_home(edit_area);
        }
    }

    // 光标移动到行首
    fn caret_home(edit_area: &mut EditArea) {
        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: edit_area.caret().line_idx,
                cell_idx: 0,
            },
        );
    }

    // 光标移动到行尾
    fn caret_end(edit_area: &mut EditArea) {
        let line_idx = edit_area.caret().line_idx;
        let new_cell_idx = edit_area.get_line_cell_count(line_idx);

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx,
                cell_idx: new_cell_idx,
            },
        );
    }

    // 光标向上移动一页
    fn caret_page_up(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        let new_line_idx = line_idx.saturating_sub(Terminal::size().height);

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: new_line_idx,
                cell_idx: cell_idx,
            },
        );
    }

    // 光标向下移动一页
    fn caret_page_down(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        let new_line_idx = line_idx.saturating_add(Terminal::size().height);

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: new_line_idx,
                cell_idx: cell_idx,
            },
        );
    }

    /// 光标移动到指定位置，自动调整光标位置到最近的、合法的位置
    fn move_caret_validly(edit_area: &mut EditArea, target: DocumentCoordinate) {
        let lines_count = edit_area.lines_len();
        // 计算有效的目标位置
        let valid_line_idx = if lines_count == 0 {
            0 // 空文档时固定为0
        } else {
            target.line_idx.min(lines_count - 1)
        };
        let valid_cell_idx = target
            .cell_idx
            .min(edit_area.get_line_cell_count(valid_line_idx));

        let valid_document_coor = DocumentCoordinate {
            line_idx: valid_line_idx,
            cell_idx: valid_cell_idx,
        };

        // 设置光标位置
        edit_area.set_caret(valid_document_coor);
        // 相应地移动终端的光标，让用户可见
        Terminal::move_caret(edit_area.caret_to_terminal());
        // 正确移动光标后，滚动文本到编辑区域
        Self::scroll_text(edit_area);
    }

    /// 滚动文本到编辑区域，若滚动偏移量发生改变则重绘编辑区域
    fn scroll_text(edit_area: &mut EditArea) {
        let row_changed = Self::scroll_vertically(edit_area);
        let col_changed = Self::scroll_horizontally(edit_area);
        if col_changed || row_changed {
            edit_area.draw();
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
        let caret_col = edit_area.get_line_cell_width_until(caret_line, caret_cell);

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

impl Cmd for Move {
    fn execute(self, editor: &mut Editor) {
        let edit_area = editor.get_mut_edit_area();

        match self {
            Self::Up => Self::caret_up(edit_area),
            Self::Down => Self::caret_down(edit_area),
            Self::Left => Self::caret_left(edit_area),
            Self::Right => Self::caret_right(edit_area),
            Self::Home => Self::caret_home(edit_area),
            Self::End => Self::caret_end(edit_area),
            Self::PageUp => Self::caret_page_up(edit_area),
            Self::PageDown => Self::caret_page_down(edit_area),
        }
    }
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Up => Ok(Self::Up),
                KeyCode::Down => Ok(Self::Down),
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Right => Ok(Self::Right),
                KeyCode::Home => Ok(Self::Home),
                KeyCode::End => Ok(Self::End),
                KeyCode::PageUp => Ok(Self::PageUp),
                KeyCode::PageDown => Ok(Self::PageDown),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
