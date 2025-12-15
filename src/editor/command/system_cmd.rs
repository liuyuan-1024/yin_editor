use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    editor::{Editor, command::Cmd},
    prelude::Size,
    terminal::Terminal,
};

pub enum System {
    Resize(Size),
    Quit,
    Save,
    Dismiss,
}

impl System {
    /// 更新所有组件的size
    fn resize(size: Size, editor: &mut Editor) {
        editor.resize_all(size);
    }

    fn quit(editor: &mut Editor) {
        Terminal::clear_screen();
        editor.set_is_quit(true);
    }
}

impl Cmd for System {
    fn execute(self, editor: &mut Editor) {
        match self {
            System::Resize(size) => Self::resize(size, editor),
            System::Quit => Self::quit(editor),
            System::Save => println!("还未实现"),
            System::Dismiss => println!("还未实现"),
        }
    }
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Char('s') => Ok(Self::Save),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
