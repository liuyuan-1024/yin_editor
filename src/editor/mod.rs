mod cell;
mod command;
mod line;
mod ui;

pub use cell::Cell;
pub use command::Command;
pub use line::Line;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::env;

use crate::{
    editor::{
        command::Cmd,
        ui::{EditArea, UI},
    },
    file::FileInfo,
    prelude::Size,
    terminal::Terminal,
};

/// 编辑器
pub struct Editor {
    terminal: Terminal,
    // 文件信息
    file_info: FileInfo,
    // 编辑区
    edit_area: EditArea,
    // 是否退出编辑器
    is_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        // 先将终端准备好
        Terminal::initialize();

        let mut editor = Self::default();

        // 获取命令行参数
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.file_info = FileInfo::from(file_name);
            // 有对应文件，就加载文件
            editor.edit_area.load(file_name);
            // 设置终端标题
            editor.terminal.set_title(file_name);
        }

        editor.resize_all(Terminal::get_size());

        editor
    }

    pub fn run(&mut self) {
        Terminal::initialize();

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
        let terminal_size = Terminal::get_size();

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

    /// 更新所有组件的尺寸，绘制所有组件
    pub fn resize_all(&mut self, size: Size) {
        self.edit_area.resize(size);
        // self.status_bar.resize(size);
        // self.command_line_bar.resize(size);
        self.draw_all();
    }

    /// 绘制所有组件
    pub fn draw_all(&mut self) {
        self.edit_area.draw(0);
        // self.status_bar.draw(self.size.height.saturating_sub(2));
        // self.command_line_bar.draw(self.size.height);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            terminal: Terminal::default(),
            file_info: FileInfo::default(),
            edit_area: EditArea::default(),
            is_quit: false,
        }
    }
}
