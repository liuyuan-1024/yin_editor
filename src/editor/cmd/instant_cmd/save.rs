use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fs::File;
use std::io::Write;

use crate::Editor;
use crate::editor::cmd::Execute;

/// CRTL + S：保存文件，将文本写入硬盘
#[derive(PartialEq, Eq)]
pub enum Save {
    CrtlS,
}

impl TryFrom<KeyEvent> for Save {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('s') {
            Ok(Self::CrtlS)
        } else {
            Err(format!("保存命令不支持：{modifiers:?} + {code:?}"))
        }
    }
}

impl Execute for Save {
    /// 保存文件，若是文件不存在就创建文件后再保存
    fn execute(self, editor: &mut Editor) {
        let file_info = editor.file_info();
        let file_path = file_info.get_path();

        let mut file = match File::create(file_path) {
            Ok(f) => f, // 创建成功，获取文件句柄
            Err(e) => {
                panic!("创建文件失败: {e:?}");
            }
        };

        let edit_area = editor.mut_edit_area();
        for line in edit_area.lines() {
            if let Err(e) = writeln!(file, "{line}") {
                panic!("写入文件失败: {e:?}");
            }
        }
        edit_area.set_is_modified(false);

        editor.update_status();

        editor.mut_cmd_line().set_prompt_for_save();
    }
}
