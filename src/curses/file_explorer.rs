use std::path::{Path, PathBuf};

use pancurses::{self, Input, Window};

pub struct FileExplorer {
    input_line: Window,
    files_pane: Window,
}

impl FileExplorer {
    pub fn new(window: &Window) -> Self {
        let input_line = window.derwin(1, 0, 0, 0).unwrap();
        let files_pane = window.derwin(0, 0, 1, 0).unwrap();
        Self {
            input_line,
            files_pane,
        }
    }

    pub fn get_file(&self) -> PathBuf {
        pancurses::curs_set(1);
        self.input_line.nodelay(false);
        self.input_line.keypad(true);
        self.files_pane.clear();
        self.files_pane.refresh();

        let mut buffer = String::new();
        loop {
            match self.input_line.getch().unwrap() {
                Input::Character('\n') => break,
                Input::Character(c) => {
                    if c.is_control() {
                        match c as u8 {
                            127 | 0x08_u8 => {      // Backspace key
                                buffer.pop();
                                let (y, x) = self.input_line.get_cur_yx();
                                self.input_line.mv(y, x - 1);
                                self.input_line.delch();
                                self.input_line.refresh();
                            },
                            0x1b_u8 => break,       // Escape key
                            _ => {}
                        }
                    } else {
                        buffer.push(c);
                        self.input_line.addch(c);
                        self.input_line.refresh();
                    }
                }
                _ => {}
            }
        }

        Path::new(&buffer).to_path_buf()
    }
}
