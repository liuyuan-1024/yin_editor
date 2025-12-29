use crate::editor::Editor;

/// 延时命令的必有特性：每一个延时命令都必须包含此特性，否则无法正常执行。
/// 延时命令必须经过两或三个阶段：“开启” -> “编辑并确认” -> “其他”。
pub trait DelayCmdTrait {
    /// 开启指定延时命令
    fn enable(self, editor: &mut Editor);

    /// 确认延时命令编辑完毕，可以执行
    fn confirm(self, editor: &mut Editor);
}
