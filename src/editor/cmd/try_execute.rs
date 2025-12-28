use crossterm::event::KeyEvent;

use super::Execute;
use crate::Editor;

pub trait TryExecute {
    /// 通用事件处理逻辑：尝试转换为目标类型并执行
    fn try_execute<T>(key_event: KeyEvent, editor: &mut Editor) -> bool
    where
        T: TryFrom<KeyEvent, Error = String> + Execute,
    {
        if let Ok(command) = T::try_from(key_event) {
            command.execute(editor);
            true
        } else {
            false
        }
    }
}
