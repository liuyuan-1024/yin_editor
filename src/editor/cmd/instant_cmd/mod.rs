mod disable;
mod quit;
mod save;
use crossterm::event::KeyEvent;
use disable::Disable;
use quit::Quit;
use save::Save;

use crate::{Editor, editor::cmd::TryExecute};

/// 即时命令：在任何情况下，只要按下快捷键，即刻生效。
pub enum InstantCmd {}

impl InstantCmd {
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) -> bool {
        return Self::try_execute::<Save>(key_event, editor)
            || Self::try_execute::<Quit>(key_event, editor)
            || Self::try_execute::<Disable>(key_event, editor);
    }
}

impl TryExecute for InstantCmd {
    fn execute(self, _: &mut Editor) {}
}
