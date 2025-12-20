use crate::{Terminal, editor::ui::UI, prelude::Size};

// 命令行，用于输入命令，显示提示信息
pub struct CmdLine {
    size: Size,
    // 命令的提示消息
    prompt_msg: String,
}

impl CmdLine {
    pub fn size(&self) -> &Size {
        &self.size
    }
}

impl UI for CmdLine {
    /// CmdLine 的高度固定为 1 行
    fn resize(&mut self, size: Size) {
        let width = size.width;
        let height = 1;
        self.size = Size { width, height };
    }

    fn draw(&mut self, start_row: usize) {
        // 文档状态区域的结束行（此行不绘制任何东西）
        let end_row = start_row.saturating_add(self.size.height);

        // 绘制可视区域内容
        for current_row in start_row..end_row {
            Terminal::print_inverted_row(current_row, &self.prompt_msg);
        }
    }
}

impl Default for CmdLine {
    fn default() -> Self {
        Self {
            size: Size::default(),
            prompt_msg: "提示消息".to_string(),
        }
    }
}
