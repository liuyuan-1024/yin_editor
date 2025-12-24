use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{fs::File, io::Write};

mod cmd_edit;
mod cmd_move;
mod find_context;
pub use cmd_edit::CmdEdit;
pub use cmd_move::CmdMove;
pub use find_context::FindContext;

use crate::{
    editor::{Editor, mode::Execute},
    terminal::Terminal,
};

pub enum Cmd {
    Quit,    // CRTL + Q
    Save,    // CRTL + S
    Disable, // ESC
    Find,    // CRTL + F
}

impl Cmd {
    /// 清理屏幕并退出
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
                panic!("创建文件失败: {e:?}");
            }
        };

        let edit_area = editor.get_mut_edit_area();
        for line in edit_area.lines() {
            if let Err(e) = writeln!(file, "{line}") {
                panic!("写入文件失败: {e:?}");
            }
        }
        edit_area.set_is_modified(false);

        editor.update_status();

        editor.mut_cmd_line().set_prompt_for_save();
    }

    /// 关闭命令行模式
    fn disable_cmd(editor: &mut Editor) {
        editor.disable_cmd_line();
        editor.mut_cmd_line().set_prompt_for_disable();

        // 将光标移动到编辑区域的原来位置
        let caret = editor.get_edit_area().caret_to_terminal();
        Terminal::move_caret(caret);
    }

    /// 开启查找模式
    fn enable_cmd_find(editor: &mut Editor) {
        editor.enable_cmd_line();

        let cmd = editor.mut_cmd_line();

        // 修改命令行的提示词
        cmd.set_prompt_for_find();
        // 将光标移动到命令行的输入框中
        Terminal::move_caret(cmd.caret().clone());
    }
}

impl Execute for Cmd {
    fn execute(self, editor: &mut Editor) {
        match self {
            Self::Quit => Self::quit(editor),
            Self::Save => Self::save(editor),
            Self::Disable => Self::disable_cmd(editor),
            Self::Find => Self::enable_cmd_find(editor),
        }
    }
}

impl TryFrom<KeyEvent> for Cmd {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Char('s') => Ok(Self::Save),
                KeyCode::Char('f') => Ok(Self::Find),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Disable)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
