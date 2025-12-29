use crate::{
    Terminal,
    editor::{Line, UI},
    prelude::{DocumentCoordinate, Size, TerminalCoordinate},
};

const SAVE_PROMPT: &str = "保存文件!";
const FIND_PROMPT: &str = "查找：";

// 命令行，用于输入命令，显示提示信息
pub struct CmdLine {
    size: Size,
    // 命令的提示消息
    prompt_msg: Line,
    // 命令输入区域
    input: Line,
    // 光标在命令行的位置，虽然是文档位置，但这是为了方便计算光标在图元间的移动和转为终端位置
    // line_idx：代表了终端可视区域的最后一行（命令行所在的区域）
    // cell_idx：代表了光标在命令行的输入区域的图元索引
    caret: DocumentCoordinate,
}

impl CmdLine {
    pub fn size(&self) -> &Size {
        &self.size
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
        self.prompt_msg = Line::from(msg);
        self.caret = DocumentCoordinate {
            line_idx: Terminal::size().height.saturating_sub(1),
            cell_idx: self.prompt_msg.width(),
        };
    }

    pub fn input(&self) -> &Line {
        &self.input
    }

    pub fn mut_input(&mut self) -> &mut Line {
        &mut self.input
    }

    pub fn set_caret(&mut self, coordinate: DocumentCoordinate) {
        self.caret = coordinate;
    }

    /// 获取光标在命令行中的位置
    pub fn caret(&self) -> &DocumentCoordinate {
        &self.caret
    }

    /// 光标的文档坐标转为终端坐标
    pub fn caret_to_terminal(&self) -> TerminalCoordinate {
        // 光标在命令行中的文档位置
        let caret_cell_idx = self.caret.cell_idx;
        // 将光标在命令行中的位置转为终端位置
        let caret_row_idx = self.caret.line_idx;
        let caret_col_idx = self
            .prompt_msg
            .width()
            .saturating_add(self.input.width_until(caret_cell_idx));

        // 返回，光标在终端中相对的位置，即肉眼看到的位置
        TerminalCoordinate {
            row: caret_row_idx,
            col: caret_col_idx,
        }
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
            let str = format!("{}{}", self.prompt_msg.to_string(), self.input.to_string());

            Terminal::print_inverted_row(current_row, &str);
        }
    }
}

impl Default for CmdLine {
    fn default() -> Self {
        let prompt_msg = Line::from("提示消息");
        let input = Line::default();
        let caret = DocumentCoordinate {
            line_idx: Terminal::size().height.saturating_sub(1),
            cell_idx: prompt_msg.width_until(prompt_msg.cells_count()),
        };

        Self {
            size: Size::default(),
            prompt_msg,
            input,
            caret,
        }
    }
}
