pub(super) struct CursorPosition {
    x: u32,
    y: u32,
}

impl CursorPosition {
    pub(super) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
