use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    Editor, Terminal,
    editor::cmd::{
        Execute,
        delay_cmd::{CmdEntry, DelayCmd},
    },
};

/// CRTL + F：查找目标字符串
#[derive(PartialEq, Eq)]
pub enum Find {
    Entry,
}

impl Find {
    /// 确认查找
    pub fn confirm(&self) {}
}

impl TryFrom<KeyEvent> for Find {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('f') {
            Ok(Self::Entry)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}

impl CmdEntry for Find {
    fn entry(self, editor: &mut Editor) {
        editor.set_delay_cmd(Some((DelayCmd::Find(Find::Entry), false)));

        let cmd = editor.mut_cmd_line();
        // 修改命令行的提示词
        cmd.set_prompt_for_find();
        // 将光标移动到命令行的输入框中
        Terminal::move_caret(cmd.caret_to_terminal());
    }
}

impl Execute for Find {
    fn execute(self, editor: &mut Editor) {
        match self {
            Self::Entry => self.entry(editor),
        }
    }
}
