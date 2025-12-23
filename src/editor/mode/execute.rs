use crate::editor::Editor;

pub trait Execute {
    /// 执行指令
    fn execute(self, editor: &mut Editor);
}
