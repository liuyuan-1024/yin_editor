use crossterm::event::{KeyEvent, KeyModifiers};

mod cmd_caret_move;
mod cmd_edit;
mod cmd_entry;
mod find;
use cmd_caret_move::CmdCaretMove;
use cmd_edit::CmdEdit;
use cmd_entry::CmdEntry;
use find::Find;

use crate::{Editor, editor::cmd::TryExecute};

/// 延时命令：需按下对应的快捷键进入指定的命令模式，进行命令编辑，按下 enter 键才会执行。
pub enum DelayCmd {
    Find(Find),
}

impl DelayCmd {
    /// 延时命令处理器
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) -> bool {
        if editor.delay_cmd.is_none() {
            if key_event.modifiers == KeyModifiers::CONTROL {
                return Self::try_execute::<Find>(key_event, editor);
            }
        }

        if editor.delay_cmd.is_some() {
            // 编辑器处于编辑或执行延时命令中
            let (delay_cmd, flag) = editor.delay_cmd.as_ref().unwrap();

            if *flag {
                // 处于编辑延时命令中
                return Self::try_execute::<CmdEdit>(key_event, editor)
                    || Self::try_execute::<CmdCaretMove>(key_event, editor);
            } else {
                // 处于执行延时命令中
                match delay_cmd {
                    Self::Find(_) => return Self::try_execute::<Find>(key_event, editor),
                }
            }
        }

        false
    }
}

impl TryExecute for DelayCmd {}
