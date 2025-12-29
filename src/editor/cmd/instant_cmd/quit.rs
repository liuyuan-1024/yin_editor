use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{Editor, Terminal, editor::cmd::TryExecute};

/// CRTL + Q：退出编辑器
#[derive(PartialEq, Eq)]
pub enum Quit {
    CrtlQ,
}

impl TryFrom<KeyEvent> for Quit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q') {
            Ok(Self::CrtlQ)
        } else {
            Err(format!("退出命令不支持：{modifiers:?} + {code:?}"))
        }
    }
}

impl TryExecute for Quit {
    /// 清理屏幕并退出
    fn execute(self, editor: &mut Editor) {
        Terminal::clear_screen();
        editor.set_is_quit(true);
    }
}
