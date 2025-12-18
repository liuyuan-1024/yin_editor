use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{fs::File, io::Write};

use crate::{
    editor::{Editor, command::Cmd},
    terminal::Terminal,
};

pub enum System {
    Resize,
    Quit,
    Save,
    Dismiss,
}

impl System {
    /// 更新所有组件的size
    fn resize(editor: &mut Editor) {
        editor.resize_edit_area();
        editor.resize_status_bar();
    }

    fn quit(editor: &mut Editor) {
        Terminal::clear_screen();
        editor.set_is_quit(true);
    }

    /// 保存文件，若是文件存在就创建文件后再保存
    fn save(editor: &mut Editor) {
        let file_info = editor.get_file_info();
        let file_path = file_info.get_path();

        let mut file = match File::create(file_path) {
            Ok(f) => f, // 创建成功，获取文件句柄
            Err(e) => {
                eprintln!("创建文件失败: {}", e); // 打印错误信息
                return;
            }
        };

        let edit_area = editor.get_mut_edit_area();
        for line in edit_area.get_lines() {
            if let Err(e) = writeln!(file, "{line}") {
                eprintln!("写入文件失败: {}", e);
                return; // 写入失败时退出
            }
        }
        edit_area.set_is_modified(false);

        editor.update_status();
    }
}

impl Cmd for System {
    fn execute(self, editor: &mut Editor) {
        match self {
            System::Resize => Self::resize(editor),
            System::Quit => Self::quit(editor),
            System::Save => Self::save(editor),
            System::Dismiss => println!("还未实现"),
        }
    }
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Char('s') => Ok(Self::Save),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
