use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub enum FileType {
    Rust,
    Text,
    #[default]
    Unsupport,
}

impl Display for FileType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Rust => write!(formatter, "Rust"),
            Self::Text => write!(formatter, "Text"),
            Self::Unsupport => write!(formatter, "Unsupport"),
        }
    }
}
