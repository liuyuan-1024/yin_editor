use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{
        Cell, Editor,
        cmd::{
            DelayCmd, TryExecute,
            delay_cmd::{CmdCaretMove, Find},
        },
    },
    prelude::DocumentCoordinate,
};

/// 命令行中的命令的编辑指令
pub enum CmdEdit {
    Insert(Cell),
    Backspace,
    Delete,
    Confirm,
}

impl CmdEdit {
    /// 在当前光标位置插入一个图元，并向右移动光标
    fn insert(cell: Cell, editor: &mut Editor) {
        let cmd_line = editor.mut_cmd_line();
        let DocumentCoordinate { cell_idx, .. } = *cmd_line.caret();
        cmd_line.mut_input().insert_cell(cell, cell_idx);
        CmdCaretMove::Right.execute(editor);
    }

    /// 删除当前光标位置的前一个图元，并向左移动光标
    fn backspace(editor: &mut Editor) {
        let cmd_line = editor.mut_cmd_line();
        let DocumentCoordinate { cell_idx, .. } = *cmd_line.caret();

        if cell_idx > 0 {
            let input = cmd_line.mut_input();
            input.delete_cell(cell_idx.saturating_sub(1));
            CmdCaretMove::Left.execute(editor);
        }
    }

    /// 删除当前光标位置的一个图元，不移动光标
    fn delete(editor: &mut Editor) {
        let cmd_line = editor.mut_cmd_line();
        let DocumentCoordinate { cell_idx, .. } = *cmd_line.caret();

        let input = cmd_line.mut_input();

        if cell_idx < input.cells_count() {
            // 移除当前光标位置的图元
            input.delete_cell(cell_idx);
        }
    }

    /// 执行当前命令的确认函数
    fn confirm(editor: &mut Editor) {
        let (delay_cmd, ..) = editor.delay_cmd.as_ref().unwrap();

        match delay_cmd {
            DelayCmd::Find => Find::Confirm.execute(editor),
        }
    }
}

/// 尝试将 KeyEvent 转换为 CmdEdit 指令
impl TryFrom<KeyEvent> for CmdEdit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (KeyCode::Char(char), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                let cell = Cell::char_to_cell(char);
                Ok(Self::Insert(cell))
            }
            (KeyCode::Tab, KeyModifiers::NONE) => {
                let cell = Cell::char_to_cell('\t');
                Ok(Self::Insert(cell))
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::Backspace),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::Confirm),
            _ => Err(format!(
                "命令编辑不支持：{:?} + {:?}",
                event.modifiers, event.code
            )),
        }
    }
}

impl TryExecute for CmdEdit {
    fn execute(self, editor: &mut Editor) {
        match self {
            Self::Insert(cell) => Self::insert(cell, editor),
            Self::Backspace => Self::backspace(editor),
            Self::Delete => Self::delete(editor),
            Self::Confirm => Self::confirm(editor),
        }
    }
}
