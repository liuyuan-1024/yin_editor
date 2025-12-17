use crate::{
    Terminal,
    editor::{FileInfo, UI},
    prelude::{DocumentCoordinate, Size},
};

pub struct StatusBar {
    size: Size,
    file_info: FileInfo,
    total_lines: usize,
    is_modified: bool,
    caret: DocumentCoordinate,
}

impl StatusBar {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn update_status(
        &mut self,
        file_info: FileInfo,
        total_lines: usize,
        is_modified: bool,
        caret: DocumentCoordinate,
    ) {
        self.file_info = file_info;
        self.total_lines = total_lines;
        self.is_modified = is_modified;
        self.caret = caret;
    }

    /// 更新状态栏信息
    /// 1. 文件路径
    /// 2. 文档名称 - 文档总行数 lines (是否修改了文档) 空白填充 光标位置行:列 | 文档类型
    /// 返回：更新后的状态信息
    fn get_status(&self) -> Vec<String> {
        let mut result = Vec::new();

        // 文档路径
        result.push(self.file_info.get_path_string());

        // 文档名称 - 文档总行数 lines (是否修改了文档) 空白填充 光标位置行:列 | 文档类型
        // 左：文件名.后缀 - 总行数 lines (是否修改了文件)
        let left = format!(
            "{} - {} lines {}",
            self.file_info.get_name(),
            self.total_lines,
            Self::is_modified_to_string(self.is_modified)
        );

        // 右：光标所在终端网格的“行:列” | 文件类型
        let right = format!(
            "{} | {}",
            Self::caret_to_string(&self.caret),
            self.file_info.get_file_type().to_string()
        );

        // 根据状态栏的宽度，在中间填充空白

        let middle_len = Terminal::size()
            .width
            .saturating_sub(left.len())
            .saturating_sub(right.len());
        let middle = " ".repeat(middle_len);

        result.push(format!("{left}{middle}{right}"));

        result
    }

    fn is_modified_to_string(is_modified: bool) -> String {
        if is_modified {
            "(modified)".to_ascii_lowercase()
        } else {
            String::new()
        }
    }

    // 光标位置转为字符串
    fn caret_to_string(caret: &DocumentCoordinate) -> String {
        format!(
            "{}:{}",
            caret.line_idx.saturating_add(1),
            caret.cell_idx.saturating_add(1)
        )
    }
}

impl UI for StatusBar {
    /// StatusBar的高度等于状态栏内容的行数
    fn resize(&mut self, size: Size) {
        let width = size.width;
        let height = self.get_status().len();
        self.size = Size { width, height };
    }

    fn draw(&mut self, start_row: usize) {
        // 绘制状态栏
        let status_strs = self.get_status();
        // 文档状态区域的结束行
        let end_row = start_row.saturating_add(status_strs.len());
        // 绘制可视区域内容
        for current_row in start_row..end_row {
            let idx = current_row.saturating_sub(start_row);
            Terminal::print_inverted_row(current_row, status_strs.get(idx).unwrap());
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            size: Size::default(),
            file_info: FileInfo::default(),
            total_lines: 0,
            is_modified: false,
            caret: DocumentCoordinate::default(),
        }
    }
}
