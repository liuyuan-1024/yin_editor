mod edit_cmd;
mod move_cmd;
mod system_cmd;

use crossterm::event::Event;
pub use edit_cmd::Edit;
pub use move_cmd::Move;
pub use system_cmd::System::{self, Resize};

use crate::editor::Editor;

pub trait Cmd {
    /// 执行指令
    fn execute(self, editor: &mut Editor);
}

// 一个 Command 可以是一个 Move/Edit/System 命令。
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl Cmd for Command {
    fn execute(self, editor: &mut Editor) {
        match self {
            Command::Move(cmd) => cmd.execute(editor),
            Command::Edit(cmd) => cmd.execute(editor),
            Command::System(cmd) => cmd.execute(editor),
        }
    }
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            // 转换优先级：Edit > Move > System
            Event::Key(key_event) => Move::try_from(key_event)
                .map(Self::Move)
                .or_else(|_| Edit::try_from(key_event).map(Self::Edit))
                .or_else(|_| System::try_from(key_event).map(Self::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            Event::Resize(_, _) => Ok(Self::System(Resize)),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
