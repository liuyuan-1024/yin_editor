use crossterm::event::{KeyEvent, KeyModifiers};

mod cmd_caret_move;
mod cmd_edit;
mod delay_cmd_trait;
mod find;
use cmd_caret_move::CmdCaretMove;
use cmd_edit::CmdEdit;
use delay_cmd_trait::DelayCmdTrait;
pub use find::Find;

use crate::{Editor, editor::cmd::TryExecute};

/// 延时命令：“开启” -> “编辑并确认” -> “其他”。
/// 按下对应快捷键开启指定命令模式，进行命令编辑，enter 键确认命令，个别延时命令有 “其他” 这一步，例如查找命令。
pub enum DelayCmd {
    Find,
}

impl DelayCmd {
    /// 延时命令处理器
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) -> bool {
        if editor.delay_cmd.is_some() {
            // 编辑器处于编辑或执行延时命令中
            let (.., flag) = editor.delay_cmd.as_ref().unwrap();

            if *flag {
                // 处于执行延时命令中（确认执行后）
                return Self::after_enter(key_event, editor);
            } else {
                // 处于编辑延时命令中（包含了确认执行命令 enter 键）
                return Self::edit(key_event, editor);
            }
        } else if key_event.modifiers == KeyModifiers::CONTROL {
            return Self::entry(key_event, editor);
        }

        false
    }

    /// 尝试进入命令模式
    fn entry(key_event: KeyEvent, editor: &mut Editor) -> bool {
        return Self::try_execute::<Find>(key_event, editor);
    }

    fn edit(key_event: KeyEvent, editor: &mut Editor) -> bool {
        return Self::try_execute::<CmdEdit>(key_event, editor)
            || Self::try_execute::<CmdCaretMove>(key_event, editor);
    }

    /// 确认执行后，根据不同的命令模式尝试执行对应操作
    fn after_enter(key_event: KeyEvent, editor: &mut Editor) -> bool {
        let (delay_cmd, ..) = editor.delay_cmd.as_ref().unwrap();

        match delay_cmd {
            Self::Find => {
                return Self::try_execute::<Find>(key_event, editor);
            }
        }
    }
}

impl TryExecute for DelayCmd {}
