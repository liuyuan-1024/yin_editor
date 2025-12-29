use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    Editor,
    editor::{
        cmd::{
            TryExecute,
            delay_cmd::{DelayCmd, DelayCmdTrait},
            text_cmd::TextScroll,
        },
        ui::EditArea,
    },
    prelude::DocumentCoordinate,
};

/// CRTL + F：查找目标字符串
#[derive(PartialEq, Eq)]
pub enum Find {
    Enable,
    Confirm,
    Up,
    Down,
    Left,
    Right,
}

impl Find {
    /// 查找上一个结果
    fn find_prev(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: line_idx.saturating_sub(1),
                cell_idx: cell_idx,
            },
        );
    }

    /// 查找下一个结果
    fn find_next(edit_area: &mut EditArea) {
        let DocumentCoordinate { line_idx, cell_idx } = *edit_area.caret();

        Self::move_caret_validly(
            edit_area,
            DocumentCoordinate {
                line_idx: line_idx.saturating_add(1),
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
            target.line_idx.min(lines_count.saturating_sub(1))
        };
        let valid_cell_idx = target
            .cell_idx
            .min(edit_area.line_cell_count(valid_line_idx));

        let valid_document_coor = DocumentCoordinate {
            line_idx: valid_line_idx,
            cell_idx: valid_cell_idx,
        };

        // 设置光标位置
        edit_area.set_caret(valid_document_coor);
        // 正确移动光标后，滚动文本到编辑区域
        TextScroll::scroll_text(edit_area);
    }
}

impl TryFrom<KeyEvent> for Find {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('f') {
            Ok(Self::Enable)
        } else if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Enter => Ok(Self::Confirm),
                KeyCode::Up => Ok(Self::Up),
                KeyCode::Down => Ok(Self::Down),
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Right => Ok(Self::Right),
                _ => Err(format!("文本光标移动不支持：{code:?}")),
            }
        } else {
            Err(format!("查找命令不支持：{modifiers:?} + {code:?}"))
        }
    }
}

impl DelayCmdTrait for Find {
    fn enable(self, editor: &mut Editor) {
        editor.enable_delay_cmd(DelayCmd::Find);
        // 修改命令行的提示词
        editor.mut_cmd_line().set_prompt_for_find();
    }

    fn confirm(self, editor: &mut Editor) {
        editor.confirm_delay_cmd();
    }
}

impl TryExecute for Find {
    fn execute(self, editor: &mut Editor) {
        match self {
            Self::Enable => self.enable(editor),
            Self::Confirm => self.confirm(editor),
            Self::Up | Self::Left => Self::find_prev(editor.mut_edit_area()),
            Self::Down | Self::Right => Self::find_next(editor.mut_edit_area()),
        }
    }
}
