use crossterm::event::KeyEvent;

mod text_caret_move;
mod text_edit;
mod text_scroll;
use text_caret_move::TextCaretMove;
use text_edit::TextEdit;
pub use text_scroll::TextScroll;

use crate::{Editor, editor::cmd::TryExecute};

/// 文本命令：负责执行文本编辑和文本光标移动
pub struct TextCmd {}

impl TextCmd {
    /// 文本命令处理器
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) -> bool {
        if editor.delay_cmd.is_none() {
            // 编辑器处于**文本编辑**中
            return Self::try_execute::<TextEdit>(key_event, editor)
                || Self::try_execute::<TextCaretMove>(key_event, editor);
        }

        false
    }
}

impl TryExecute for TextCmd {
    fn execute(self, _: &mut Editor) {}
}
