use crossterm::event::Event;

mod edit;
mod execute;
mod system;
pub use edit::{Edit, EditMove};
pub use execute::Execute;
pub use system::System::{self, Resize};

use crate::editor::Editor;

// 一个 Command 可以是一个 Move/Edit/System 命令。
pub enum Cmd {
    Move(EditMove),
    Edit(Edit),
    System(System),
}

impl Execute for Cmd {
    fn execute(self, editor: &mut Editor) {
        match self {
            Cmd::Move(cmd) => cmd.execute(editor),
            Cmd::Edit(cmd) => cmd.execute(editor),
            Cmd::System(cmd) => cmd.execute(editor),
        }
    }
}

impl TryFrom<Event> for Cmd {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            // 转换优先级：Edit > Move > System
            Event::Key(key_event) => EditMove::try_from(key_event)
                .map(Self::Move)
                .or_else(|_| Edit::try_from(key_event).map(Self::Edit))
                .or_else(|_| System::try_from(key_event).map(Self::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            Event::Resize(_, _) => Ok(Self::System(Resize)),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
