mod disable;
mod quit;
mod save;
use crossterm::event::KeyEvent;
use disable::Disable;
use quit::Quit;
use save::Save;

use crate::{Editor, editor::cmd::TryExecute};

/// 即时命令：无需进入指定的命令模式，在任何模式下，只要按下快捷键，即刻生效。
pub enum InstantCmd {}

impl InstantCmd {
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) -> bool {
        // 只要处于命令模式，无论是命令的编辑还是执行，都需要先尝试执行以下命令
        Self::try_execute::<Save>(key_event, editor)
            || Self::try_execute::<Quit>(key_event, editor)
            || Self::try_execute::<Disable>(key_event, editor)
    }
}

impl TryExecute for InstantCmd {}
