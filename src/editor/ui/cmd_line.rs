use crate::{
    Terminal,
    editor::{Line, UI},
    prelude::{Size, TerminalCoordinate},
};

const DISABLE_PROMPT: &str = "退出命令模式!";
const SAVE_PROMPT: &str = "保存文件!";
const FIND_PROMPT: &str = "查找: ";

// 命令行，用于输入命令，显示提示信息
pub struct CmdLine {
    size: Size,
    // 命令的提示消息
    prompt_msg: String,
    // 命令输入区域
    input: Line,
    // 光标在命令行的位置
    caret: TerminalCoordinate,
}

impl CmdLine {
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// 设置退出命令模式的提示消息
    pub fn set_prompt_for_disable(&mut self) {
        self.set_prompt_msg(DISABLE_PROMPT);
    }

    /// 设置保存文件时的提示消息
    pub fn set_prompt_for_save(&mut self) {
        self.set_prompt_msg(SAVE_PROMPT);
    }

    /// 设置查找模式的提示消息
    pub fn set_prompt_for_find(&mut self) {
        self.set_prompt_msg(FIND_PROMPT);
    }

    /// 设置提示消息，并自动更新光标的位置
    fn set_prompt_msg(&mut self, msg: &str) {
        self.prompt_msg = msg.to_string();
        self.caret = TerminalCoordinate {
            row: Terminal::size().height.saturating_sub(1),
            col: self.prompt_msg.len(),
        };
    }

    pub fn input_string(&self) -> String {
        self.input.to_string()
    }

    pub fn set_caret(&mut self, coordinate: TerminalCoordinate) {
        self.caret = coordinate;
    }

    /// 获取光标在命令行中的位置
    pub fn caret(&self) -> &TerminalCoordinate {
        &self.caret
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
        let prompt_msg = "提示消息".to_string();
        let input = Line::default();
        let caret = TerminalCoordinate {
            row: Terminal::size().height.saturating_sub(1),
            col: prompt_msg.len(),
        };

        Self {
            size: Size::default(),
            prompt_msg,
            input,
            caret,
        }
    }
}
