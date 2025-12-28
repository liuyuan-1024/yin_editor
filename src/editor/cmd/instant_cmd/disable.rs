use crossterm::event::{KeyCode, KeyEvent};

use crate::{Editor, Terminal, editor::cmd::Execute};

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
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
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

        // 设置命令行提示词
        editor.mut_cmd_line().set_prompt_for_disable();

        // 将光标移动到编辑区域的原来位置
        let caret = editor.edit_area().caret_to_terminal();
        Terminal::move_caret(caret);
    }
}
