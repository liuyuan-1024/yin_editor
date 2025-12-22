use crossterm::event::Event;

mod edit;
mod execute;
mod system;
pub use edit::{Edit, EditMove};
pub use execute::Execute;
pub use system::System;

use crate::editor::Editor;

// 一个 Command 可以是一个 Move/Edit/System 命令。
pub enum Cmd {
    EditMove(EditMove),
    Edit(Edit),
    System(System),
}

impl Execute for Cmd {
    fn execute(self, editor: &mut Editor) {
        match self {
            Cmd::EditMove(cmd) => cmd.execute(editor),
            Cmd::Edit(cmd) => cmd.execute(editor),
            Cmd::System(cmd) => cmd.execute(editor),
        }
    }
}

impl TryFrom<Event> for Cmd {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Self::Edit)
                .or_else(|_| EditMove::try_from(key_event).map(Self::EditMove))
                .or_else(|_| System::try_from(key_event).map(Self::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
