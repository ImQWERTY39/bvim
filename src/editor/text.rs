use std::fs::File;

pub(super) struct TextBuffer {
    buffer: Vec<char>,
}

impl TextBuffer {
    pub(super) fn read(fr: &mut File) -> Self {
        todo!()
    }
}
