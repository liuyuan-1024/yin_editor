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
    terminal: Terminal,
    // 文件信息
    file_info: FileInfo,
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
        if let Some(file_name) = args.get(1) {
            editor.terminal.set_title(file_name);
            editor.edit_area.load(file_name);
            editor.status_bar.update_status(
                Size {
                    width: Terminal::size().width,
                    height: 2,
                },
                FileInfo::from(file_name),
                editor.edit_area.lines_len(),
                editor.edit_area.is_modified(),
                editor.edit_area.caret().clone(),
            );
        }

        // 调整组件尺寸
        editor.resize_all(Terminal::size());

        editor
    }

    pub fn run(&mut self) {
        // Terminal::initialize();

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
        let width = size.width;
        let edit_area_size = Size {
            width,
            height: size.height - self.status_bar.size().height,
        };

        self.edit_area.resize(edit_area_size);
        self.status_bar.resize(size);
        // self.command_line_bar.resize(size);
        self.draw_all();
    }

    /// 绘制所有组件
    pub fn draw_all(&mut self) {
        self.edit_area.draw(0);
        self.status_bar.draw(self.edit_area.size().height);
        // self.command_line_bar.draw(self.size.height);
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
