use std::io::{Write, stdout};

use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{
        Attribute::{Reset, Reverse},
        Print,
    },
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode, size,
    },
};

use crate::{
    prelude::{RowIdx, Size, TerminalCoordinate},
    terminal,
};

/// 我们使用crossterm库来实现终端的控制功能
pub struct Terminal {
    title: String,
}

impl Terminal {
    /// 获取终端的实时尺寸
    pub fn size() -> Size {
        let (width_u16, height_u16) = terminal::size().unwrap_or((0, 0));
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Size { width, height }
    }

    /// 设置终端标题
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        Self::queue_command(SetTitle(title))
    }

    /// 初始化终端
    pub fn initialize() {
        enable_raw_mode(); // 1. 进入原始模式（禁用终端默认行为）
        Self::enter_alternate_screen(); // 2. 进入备用屏幕（独立缓冲区，不干扰原终端）
        Self::disable_line_wrap(); // 3. 禁用自动换行（编辑器自己控制换行）
        Self::clear_screen(); // 4. 清空备用屏幕（初始化显示）
        Self::execute(); // 5. 执行所有命令（刷新缓冲区）
    }

    /// 终止终端（恢复默认状态）
    pub fn terminate() {
        // 1. 在备用屏幕内恢复终端属性（避免影响原始终端）
        Self::enable_line_wrap(); // 恢复自动换行
        Self::show_caret(); // 恢复光标显示
        Self::clear_screen(); // 清空备用屏幕（可选，避免残留）
        Self::execute(); // 执行备用屏幕内的清理命令

        // 2. 离开备用屏幕（回到原始终端）
        Self::leave_alternate_screen();
        Self::execute(); // 确保离开操作生效

        // 3. 最后退出原始模式（必须处理错误）
        disable_raw_mode();
    }

    /// 进入备用屏幕
    fn enter_alternate_screen() {
        Self::queue_command(EnterAlternateScreen);
    }

    /// 离开备用屏幕
    fn leave_alternate_screen() {
        Self::queue_command(LeaveAlternateScreen);
    }

    /// 清理整个屏幕
    pub fn clear_screen() {
        Self::queue_command(Clear(ClearType::All));
    }

    /// 清理当前行
    pub fn clear_line() {
        Self::queue_command(Clear(ClearType::CurrentLine));
    }

    /// 隐藏光标
    pub fn hide_caret() {
        Self::queue_command(Hide);
    }

    /// 显示光标
    pub fn show_caret() {
        Self::queue_command(Show);
    }

    /// 移动光标到指定位置
    pub fn move_caret(position: TerminalCoordinate) {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16));
    }

    /// 开启终端自动换行
    pub fn enable_line_wrap() {
        Self::queue_command(EnableLineWrap);
    }

    /// 关闭终端自动换行
    pub fn disable_line_wrap() {
        Self::queue_command(DisableLineWrap);
    }

    /// 在终端上打印一行字符串
    pub fn print_row(row: RowIdx, line_text: &str) {
        Self::move_caret(TerminalCoordinate { col: 0, row });
        Self::clear_line();
        Self::print(line_text);
    }

    /// 在终端上打印一行字符串，并反转颜色
    pub fn print_inverted_row(row: RowIdx, line_text: &str) {
        let width = Self::size().width;
        Self::print_row(row, &format!("{Reverse}{line_text:width$.width$}{Reset}"))
    }

    /// 在终端上打印字符串
    fn print(string: &str) {
        Self::queue_command(Print(string));
    }

    /// 立即执行命令
    pub fn execute() {
        let _ = stdout().flush();
    }

    /// queue!：将命令暂存到缓冲区，延迟执行，直到手动调用 flush() 才会一次性发送所有排队的命令
    fn queue_command(command: impl Command) {
        let _ = queue!(stdout(), command);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            title: String::new(),
        }
    }
}
