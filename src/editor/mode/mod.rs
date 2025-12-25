use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

mod cmd;
mod edit;
mod execute;
pub use cmd::FindContext;
use cmd::{Cmd, CmdEdit, CmdMove};
use crossterm::event::KeyModifiers;
use edit::Edit;
use edit::EditMove;
pub use execute::Execute;

use crate::Editor;

/// 编辑器模式：编辑模式、命令行模式
/// 每种模式下，编辑器会有不同的逻辑
pub enum Mode {
    EditMode,
    CmdLineMode,
}

impl Mode {
    pub fn key_event_handler(key_event: KeyEvent, editor: &mut Editor) {
        if key_event.kind == KeyEventKind::Press {
            return;
        }

        // 此时，编辑器可能处于编辑模式，也可能处于命令行模式
        match editor.mode() {
            Mode::EditMode => {
                // 当编辑器处于编辑模式时，要依据当前的KeyEvent来判断，用户是否要进入命令行模式
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::execute_in_cmd_mode(key_event, editor);
                } else {
                    Self::execute_in_edit_mode(key_event, editor);
                }
            }
            Mode::CmdLineMode => {
                // 当编辑器处于命令行模式时，直接执行命令行模式下的指令
                Self::execute_in_cmd_mode(key_event, editor);
            }
        }
    }

    /// 处理编辑模式下的事件
    fn execute_in_edit_mode(key_event: KeyEvent, editor: &mut Editor) -> bool {
        Self::try_execute::<Edit>(key_event, editor)
            || Self::try_execute::<EditMove>(key_event, editor)
    }

    /// 处理命令行模式下的事件
    fn execute_in_cmd_mode(key_event: KeyEvent, editor: &mut Editor) -> bool {
        Self::try_execute::<Cmd>(key_event, editor)
            || Self::try_execute::<CmdEdit>(key_event, editor)
            || Self::try_execute::<CmdMove>(key_event, editor)
    }

    /// 通用事件处理逻辑：尝试转换为目标类型并执行
    fn try_execute<T>(key_event: KeyEvent, editor: &mut Editor) -> bool
    where
        T: TryFrom<KeyEvent, Error = String> + Execute,
    {
        if let Ok(cmd) = T::try_from(key_event) {
            cmd.execute(editor);
            true
        } else {
            false
        }
    }
}
