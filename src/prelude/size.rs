pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
