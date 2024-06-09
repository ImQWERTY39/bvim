use std::{fs::File, path::Path};

mod modes;
use crossterm::QueueableCommand;
use modes::{Modes, ViewModes};

mod cursor;
use cursor::CursorPosition;

mod text;
use text::TextBuffer;

pub struct Editor {
    mode: Modes,
    view_mode: ViewModes,
    cursor_pos: CursorPosition,
    file_descriptor: File,
    buffer: TextBuffer,
}

impl Editor {
    pub fn init(file_path: Option<String>) -> Self {
        match file_path {
            Some(i) => {
                let path = Path::new(&i);

                if !path.is_file() {
                    panic!("{:?} is not a file", path);
                }

                let mut fd = if !path.exists() {
                    File::create_new(path).unwrap()
                } else {
                    File::options().write(true).read(true).open(path).unwrap()
                };

                let buffer = TextBuffer::read(&mut fd);

                Self {
                    mode: Modes::Normal,
                    view_mode: ViewModes::Binary,
                    cursor_pos: CursorPosition::new(0, 0),
                    file_descriptor: fd,
                    buffer,
                }
            }
            None => todo!(),
        }
    }
}
