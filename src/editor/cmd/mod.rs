use crossterm::event::{KeyEvent, KeyEventKind};

mod delay_cmd;
mod execute;
mod instant_cmd;
mod text_cmd;
mod try_execute;
pub use delay_cmd::DelayCmd;
use execute::Execute;
use instant_cmd::InstantCmd;
use text_cmd::TextCmd;
use try_execute::TryExecute;

use crate::Editor;

pub enum Cmd {}

impl Cmd {
    /// 命令处理器，
    pub fn handler(key_event: KeyEvent, editor: &mut Editor) {
        if key_event.kind == KeyEventKind::Press {
            let _ = InstantCmd::handler(key_event, editor)
                || TextCmd::handler(key_event, editor)
                || DelayCmd::handler(key_event, editor);
        }
    }
}
