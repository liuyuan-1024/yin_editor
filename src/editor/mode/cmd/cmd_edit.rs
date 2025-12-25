use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{
        Cell, Editor,
        mode::{Execute, cmd::CmdMove},
    },
    prelude::DocumentCoordinate,
};

pub enum CmdEdit {
    Enter,
    Insert(Cell),
    Delete,
    Backspace,
}

impl CmdEdit {
    /// 执行命令
    fn enter(editor: &mut Editor) {}

    /// 在当前光标位置插入一个图元，并向右移动光标
    fn insert(cell: Cell, editor: &mut Editor) {
        let cmd_line = editor.mut_cmd_line();
        let DocumentCoordinate { cell_idx, .. } = *cmd_line.caret();
        cmd_line.mut_input().insert_cell(cell, cell_idx);
        CmdMove::Right.execute(editor);
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

    /// 删除当前光标位置的前一个图元，并向左移动光标
    fn backspace(editor: &mut Editor) {
        let cmd_line = editor.mut_cmd_line();
        let DocumentCoordinate { cell_idx, .. } = *cmd_line.caret();

        if cell_idx > 0 {
            let input = cmd_line.mut_input();
            input.delete_cell(cell_idx.saturating_sub(1));
            CmdMove::Left.execute(editor);
        }
    }
}

impl Execute for CmdEdit {
    fn execute(self, editor: &mut Editor) {
        match self {
            // Enter、Insert会移动光标，进而触发状态栏的更新
            Self::Enter => Self::enter(editor),
            Self::Insert(cell) => Self::insert(cell, editor),
            Self::Delete => Self::delete(editor),
            Self::Backspace => Self::backspace(editor),
        }
    }
}

impl TryFrom<KeyEvent> for CmdEdit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::Enter),
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
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}
