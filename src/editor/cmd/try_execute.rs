use crossterm::event::KeyEvent;

use crate::Editor;

pub trait TryExecute {
    /// 通用事件处理逻辑：尝试转换为目标类型并执行
    fn try_execute<T>(key_event: KeyEvent, editor: &mut Editor) -> bool
    where
        T: TryFrom<KeyEvent, Error = String> + TryExecute,
    {
        if let Ok(t) = T::try_from(key_event) {
            t.execute(editor);
            true
        } else {
            false
        }
    }

    /// 执行指令
    fn execute(self, editor: &mut Editor);
}
