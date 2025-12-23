use crossterm::event::{self, Event};

mod cell;
mod line;
mod mode;
mod ui;
pub use cell::Cell;
pub use line::Line;

use crate::{
    editor::{
        mode::Mode,
        ui::{CmdLine, EditArea, StatusBar, UI},
    },
    file::FileInfo,
    prelude::Size,
    terminal::Terminal,
};

/// 编辑器
pub struct Editor {
    // 文件信息
    file_info: FileInfo,
    // 终端
    terminal: Terminal,
    // 编辑区
    edit_area: EditArea,
    // 状态栏
    status_bar: StatusBar,
    // 命令行
    cmd_line: CmdLine,
    // 是否退出编辑器
    is_quit: bool,
}

impl Editor {
    pub fn new(file_path: &str) -> Self {
        let mut editor = Editor::default();

        // 初始化文件信息
        editor.file_info = FileInfo::from(file_path);
        // 初始化终端标题
        editor.terminal.set_title(editor.file_info.get_name());
        // 初始化编辑区文档
        editor.edit_area.load(&editor.file_info.get_path_str());
        // 初始化状态栏
        editor.status_bar.update_status(
            editor.file_info.clone(),
            editor.edit_area.lines_len(),
            editor.edit_area.is_modified(),
            editor.edit_area.caret().clone(),
        );

        // 调整组件尺寸，
        editor.resize_all();

        // 初始化终端
        Terminal::initialize();

        editor
    }

    pub fn run(&mut self) {
        loop {
            if self.is_quit {
                Terminal::terminate();
                break;
            }

            // 每当匹配一个命令后，都会循环到此处，触发刷新屏幕函数
            self.refresh_screen();

            match event::read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        let _ = err;
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let terminal_size = Terminal::size();

        if terminal_size.height == 0 || terminal_size.width == 0 {
            return;
        }

        Terminal::hide_caret();

        // 绘制所有组件
        self.draw_all();

        Terminal::move_caret(self.edit_area.caret_to_terminal());
        Terminal::show_caret();

        Terminal::execute();
    }

    fn evaluate_event(&mut self, event: Event) {
        if matches!(&event, Event::Resize(_, _)) {
            self.resize_all();
            return;
        }

        if let Event::Key(key_event) = event {
            Mode::key_event_handler(key_event, self);
        }
    }

    pub fn get_file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn get_edit_area(&self) -> &EditArea {
        &self.edit_area
    }

    pub fn get_mut_edit_area(&mut self) -> &mut EditArea {
        &mut self.edit_area
    }

    pub fn set_is_quit(&mut self, is_quit: bool) {
        self.is_quit = is_quit;
    }

    pub fn update_status(&mut self) {
        let file_info = self.get_file_info().clone();

        let edit_area = self.get_edit_area();
        let total_lens = edit_area.lines_len();
        let is_modified = edit_area.is_modified();
        let caret = edit_area.caret().clone();

        self.status_bar
            .update_status(file_info, total_lens, is_modified, caret);
    }

    /// 更新所有组件尺寸，并重绘组件
    /// 先调整状态栏和命令行的尺寸，再调整编辑区尺寸，编辑区尺寸依赖于前两者尺寸
    pub fn resize_all(&mut self) {
        self.resize_status_bar();
        self.resize_cmd_line();
        self.resize_edit_area();
    }

    fn resize_edit_area(&mut self) {
        let size = Size {
            width: Terminal::size().width,
            height: Terminal::size()
                .height
                .saturating_sub(self.status_bar.size().height)
                .saturating_sub(self.cmd_line.size().height),
        };
        self.edit_area.resize(size);
    }

    fn resize_status_bar(&mut self) {
        let size = Size {
            width: Terminal::size().width,
            height: 0,
        };
        self.status_bar.resize(size);
    }

    fn resize_cmd_line(&mut self) {
        let size = Size {
            width: Terminal::size().width,
            height: 0,
        };
        self.cmd_line.resize(size);
    }

    pub fn draw_all(&mut self) {
        self.draw_edit_area();
        self.draw_status_bar();
        self.draw_cmd_line();
    }

    fn draw_edit_area(&mut self) {
        self.edit_area.draw(0);
    }

    fn draw_status_bar(&mut self) {
        self.status_bar.draw(self.edit_area.size().height);
    }

    fn draw_cmd_line(&mut self) {
        self.cmd_line.draw(Terminal::size().height);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            terminal: Terminal::default(),
            edit_area: EditArea::default(),
            file_info: FileInfo::default(),
            status_bar: StatusBar::default(),
            cmd_line: CmdLine::default(),
            is_quit: false,
        }
    }
}
