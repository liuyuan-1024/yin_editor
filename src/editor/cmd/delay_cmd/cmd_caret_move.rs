use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{Editor, cmd::Execute, ui::CmdLine},
    prelude::DocumentCoordinate,
    terminal::Terminal,
};

/// 命令行中的光标的移动指令
pub enum CmdCaretMove {
    Left,
    Right,
    Home,
    End,
}

impl CmdCaretMove {
    // 光标向左移动
    fn caret_left(cmd_line: &mut CmdLine) {
        let DocumentCoordinate { line_idx, cell_idx } = *cmd_line.caret();

        if cell_idx > 0 {
            Self::move_caret_validly(
                cmd_line,
                DocumentCoordinate {
                    line_idx,
                    cell_idx: cell_idx.saturating_sub(1),
                },
            );
        }
    }

    // 光标向右移动
    fn caret_right(cmd_line: &mut CmdLine) {
        let DocumentCoordinate { line_idx, cell_idx } = *cmd_line.caret();

        if cell_idx < cmd_line.input().cells_count() {
            Self::move_caret_validly(
                cmd_line,
                DocumentCoordinate {
                    line_idx,
                    cell_idx: cell_idx.saturating_add(1),
                },
            );
        }
    }

    // 光标移动到行首
    fn caret_home(cmd_line: &mut CmdLine) {
        Self::move_caret_validly(
            cmd_line,
            DocumentCoordinate {
                line_idx: cmd_line.caret().line_idx,
                cell_idx: 0,
            },
        );
    }

    // 光标移动到行尾
    fn caret_end(cmd_line: &mut CmdLine) {
        let row = cmd_line.caret().line_idx;
        let new_cell_idx = cmd_line.input().cells_count();

        Self::move_caret_validly(
            cmd_line,
            DocumentCoordinate {
                line_idx: row,
                cell_idx: new_cell_idx,
            },
        );
    }

    /// 光标移动到指定位置，自动调整光标位置到最近的、合法的位置
    fn move_caret_validly(cmd_line: &mut CmdLine, target: DocumentCoordinate) {
        let valid_row = Terminal::size().height.saturating_sub(1);
        let valid_cell_idx = target.cell_idx.min(cmd_line.input().width());

        // 设置光标位置
        cmd_line.set_caret(DocumentCoordinate {
            line_idx: valid_row,
            cell_idx: valid_cell_idx,
        });
    }
}

impl TryFrom<KeyEvent> for CmdCaretMove {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Right => Ok(Self::Right),
                KeyCode::Home => Ok(Self::Home),
                KeyCode::End => Ok(Self::End),
                _ => Err(format!("命令行光标移动不支持：{code:?}")),
            }
        } else {
            Err(format!("命令行光标移动不支持：{modifiers:?} + {code:?}"))
        }
    }
}

impl Execute for CmdCaretMove {
    fn execute(self, editor: &mut Editor) {
        {
            let cmd_line = editor.mut_cmd_line();

            match self {
                Self::Left => Self::caret_left(cmd_line),
                Self::Right => Self::caret_right(cmd_line),
                Self::Home => Self::caret_home(cmd_line),
                Self::End => Self::caret_end(cmd_line),
            }
        }
    }
}
