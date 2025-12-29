use crossterm::event::{KeyCode, KeyEvent};

use super::super::Execute;
use crate::Editor;

/// ESC：退出命令模式以及命令编辑模式，返回至文本编辑模式
#[derive(PartialEq, Eq)]
pub enum Disable {
    Esc,
}

impl TryFrom<KeyEvent> for Disable {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if code == KeyCode::Esc {
            Ok(Self::Esc)
        } else {
            Err(format!("取消命令不支持：{modifiers:?} + {code:?}"))
        }
    }
}

impl Execute for Disable {
    /// 关闭当前命令模式
    fn execute(self, editor: &mut Editor) {
        if editor.delay_cmd.is_none() {
            return;
        }

        editor.set_delay_cmd(Option::None);
    }
}
