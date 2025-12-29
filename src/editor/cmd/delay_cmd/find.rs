use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    Editor,
    editor::cmd::{
        Execute,
        delay_cmd::{DelayCmd, DelayCmdTrait},
    },
};

/// CRTL + F：查找目标字符串
#[derive(PartialEq, Eq)]
pub enum Find {
    Enable,
    Confirm,
}

impl TryFrom<KeyEvent> for Find {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('f') {
            Ok(Self::Enable)
        } else if modifiers == KeyModifiers::NONE && code == KeyCode::Enter {
            Ok(Self::Confirm)
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

impl Execute for Find {
    fn execute(self, editor: &mut Editor) {
        match self {
            Self::Enable => self.enable(editor),
            Self::Confirm => self.confirm(editor),
        }
    }
}
