use crate::editor::Editor;

pub trait CmdEntry {
    /// 命令的入口函数：修改编辑器的 mode 和 cmd_mode，因为有些命令必须在指定的命令模式下才能触发。如 Find 等。
    /// 在任何模式下，只需按下快捷键即刻生效的命令，无需入口函数，如 Save、Quit 等。
    fn entry(self, editor: &mut Editor);
}
