use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{Editor, mode::Execute, ui::EditArea},
    prelude::DocumentCoordinate,
    terminal::Terminal,
};

/// 定义了命令行中如何移动光标
pub enum CmdMove {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}

impl CmdMove {
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

    /// 光标移动到指定位置，自动调整光标位置到最近的、合法的位置
    fn move_caret_validly(edit_area: &mut EditArea, target: DocumentCoordinate) {
        let lines_count = edit_area.lines_len();
        // 计算有效的目标位置
        let valid_line_idx = if lines_count == 0 {
            0 // 空文档时固定为0
        } else {
            target.line_idx.min(lines_count.saturating_sub(1))
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
        // Self::scroll_text(edit_area);
    }
}

impl Execute for CmdMove {
    fn execute(self, editor: &mut Editor) {
        {
            let edit_area = editor.get_mut_edit_area();

            match self {
                Self::Up => Self::caret_up(edit_area),
                Self::Down => Self::caret_down(edit_area),
                Self::Left => Self::caret_left(edit_area),
                Self::Right => Self::caret_right(edit_area),
                Self::Home => Self::caret_home(edit_area),
                Self::End => Self::caret_end(edit_area),
            }
        }

        // 更新状态栏
        editor.update_status();
    }
}

impl TryFrom<KeyEvent> for CmdMove {
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
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
