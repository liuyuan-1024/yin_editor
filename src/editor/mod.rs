mod cell;
mod command;
mod line;
mod ui;

use std::env;

pub use cell::Cell;
pub use command::Command;
pub use line::Line;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

use crate::{
    editor::{
        command::Cmd,
        ui::{EditArea, StatusBar, UI},
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
    // 是否退出编辑器
    is_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        // 初始化终端
        Terminal::initialize();

        let mut editor = Editor::default();

        // 处理命令行参数（加载文件）
        let args: Vec<String> = env::args().collect();
        let file_info = match args.get(1) {
            Some(file_path) => FileInfo::from(file_path),
            None => FileInfo::default(),
        };

        // 初始化文件信息
        editor.file_info = file_info;
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

        // 调整组件尺寸
        editor.resize_edit_area();
        editor.resize_status_bar();

        editor
    }

    pub fn run(&mut self) {
        loop {
            if self.is_quit {
                Terminal::terminate();
                break;
            }

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
        self.draw_edit_area();
        self.draw_status_bar();

        Terminal::move_caret(self.edit_area.caret_to_terminal());
        Terminal::show_caret();

        Terminal::execute();
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_process {
            return;
        }

        if let Ok(command) = Command::try_from(event) {
            command.execute(self);
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

    pub fn resize_edit_area(&mut self) {
        let size = Size {
            width: Terminal::size().width,
            height: Terminal::size().height - self.status_bar.size().height,
        };
        self.edit_area.resize(size);
    }

    pub fn resize_status_bar(&mut self) {
        let size = Size {
            width: Terminal::size().width,
            height: 0,
        };
        self.status_bar.resize(size);
    }

    pub fn draw_edit_area(&mut self) {
        self.edit_area.draw(0);
    }

    pub fn draw_status_bar(&mut self) {
        self.status_bar.draw(self.edit_area.size().height);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            terminal: Terminal::default(),
            edit_area: EditArea::default(),
            file_info: FileInfo::default(),
            status_bar: StatusBar::default(),
            is_quit: false,
        }
    }
}
