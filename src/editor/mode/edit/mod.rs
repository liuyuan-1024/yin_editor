use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

mod edit_move;
pub use edit_move::EditMove;

use crate::{
    editor::{Cell, Editor, mode::Execute},
    prelude::DocumentCoordinate,
};

pub enum Edit {
    Enter,
    Insert(Cell),
    Delete,
    Backspace,
}

impl Edit {
    /// 根据当前光标位置，截断当前行，行的后一部分作为新行内容插入到下一行，并向下移动光标
    fn enter(editor: &mut Editor) {
        let edit_area = editor.mut_edit_area();
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        if let Some(line) = edit_area.mut_line_on_caret() {
            let (head, tail) = line.split(cell_idx);
            let lines = edit_area.mut_lines();
            lines.remove(line_idx);
            lines.insert(line_idx, head);
            lines.insert(line_idx.saturating_add(1), tail);
            edit_area.set_is_modified(true);
            EditMove::Down.execute(editor);
            EditMove::Home.execute(editor);
        }
    }

    /// 在当前光标位置插入一个图元，并向右移动光标
    fn insert(cell: Cell, editor: &mut Editor) {
        let edit_area = editor.mut_edit_area();
        let DocumentCoordinate { cell_idx, .. } = *edit_area.caret();

        if let Some(line) = edit_area.mut_line_on_caret() {
            line.insert_cell(cell, cell_idx);
            edit_area.set_is_modified(true);
            EditMove::Right.execute(editor);
        }
    }

    /// 删除当前光标位置的一个图元，不移动光标
    fn delete(editor: &mut Editor) {
        let edit_area = editor.mut_edit_area();
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        // 边界情况：行尾：按下 delete 需要移除并合并下一行，如何没有下一行就无操作。
        // edit_area.remove_line() 一定会返回一个 Line，虽然可能line中没有图元，但可以统一边界操作
        let cell_count = edit_area
            .line_on_caret()
            .map_or(0, |line| line.cells_count());

        if cell_idx == cell_count {
            // 移除并获取下一行
            let next_line = edit_area.remove_line(line_idx.saturating_add(1));
            // 合并两行
            if let Some(line) = edit_area.mut_line_on_caret() {
                line.merge(next_line);
                edit_area.set_is_modified(true);
            }
        } else if cell_idx < cell_count {
            // 一般情况
            // 移除当前光标位置的图元
            if let Some(line) = edit_area.mut_line_on_caret() {
                line.delete_cell(cell_idx);
                edit_area.set_is_modified(true);
            }
        }
    }

    /// 删除当前光标位置的前一个图元，并向左移动光标
    fn backspace(editor: &mut Editor) {
        let edit_area = editor.mut_edit_area();
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        // 光标在首行的行首
        if line_idx == 0 && cell_idx == 0 {
            return;
        }

        // 其他边界情况：
        // 行首：移动光标到上一行的行尾，并合并当前行
        // 两种边界情况可以合并处理：移动光标到上一行的行尾，然后执行 delete
        if line_idx == edit_area.lines_len() || cell_idx == 0 {
            EditMove::Up.execute(editor);
            EditMove::End.execute(editor);
            Self::delete(editor);
        } else {
            // 一般情况
            // 移除当前光标位置的图元
            if let Some(line) = edit_area.mut_line_on_caret() {
                line.delete_cell(cell_idx.saturating_sub(1));
                edit_area.set_is_modified(true);
                EditMove::Left.execute(editor);
            }
        }
    }
}

impl Execute for Edit {
    fn execute(self, editor: &mut Editor) {
        match self {
            // Enter、Insert会移动光标，进而触发状态栏的更新
            Edit::Enter => Self::enter(editor),
            Edit::Insert(cell) => Self::insert(cell, editor),
            Edit::Delete => {
                Self::delete(editor);
                editor.update_status();
            }
            Edit::Backspace => {
                Self::backspace(editor);
                editor.update_status();
            }
        }
    }
}

impl TryFrom<KeyEvent> for Edit {
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
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::Enter),
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::Backspace),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}
