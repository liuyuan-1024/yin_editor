//! 编辑器模式：编辑模式、命令模式
//! 每种模式下，编辑器会有不同的逻辑

use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

mod cmd;
mod edit;
mod execute;
use cmd::{Cmd, CmdEdit, CmdMove};
use edit::Edit;
use edit::EditMove;
pub use execute::Execute;

use crate::Editor;

pub enum Mode {}

impl Mode {
    pub fn key_event_handler(key_event: KeyEvent, editor: &mut Editor) {
        if key_event.kind == KeyEventKind::Press {
            let _ = !Self::execute_in_edit_mode(key_event, editor)
                && Self::execute_in_cmd_mode(key_event, editor);
        }
    }

    /// 处理编辑模式下的事件
    fn execute_in_edit_mode(key_event: KeyEvent, editor: &mut Editor) -> bool {
        Self::try_execute::<Edit>(key_event, editor)
            || Self::try_execute::<EditMove>(key_event, editor)
    }

    /// 处理命令模式下的事件
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
