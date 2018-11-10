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
        let mut result = Path::new(".");
        loop {
            match self.input_line.getch().unwrap() {
                Input::Character('\n') => {
                    let target = Path::new(&buffer);
                    if target.is_file() {
                        result = target;
                    }
                    break;
                },
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

            self.files_pane.clear();
            let target = Path::new(&buffer);
            if let Some(parent) = target.parent() {
                if let Ok(dir) = parent.read_dir() {
                    dir.filter_map(|entry| entry.ok())
                        .map(|entry| entry.path())
                        .filter(|path| path.to_string_lossy().starts_with(&buffer))
                        .for_each(|path| {
                            if path.is_dir() {
                                self.files_pane.addstr(format!("{}/\n", path.display()));
                            } else {
                                self.files_pane.addstr(format!("{}\n", path.display()));
                            }
                        });
                }
            }
            if let Ok(dir) = target.read_dir() {
                dir.filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .for_each(|path| {
                        if path.is_dir() {
                            self.files_pane.addstr(format!("{}/\n", path.display()));
                        } else {
                            self.files_pane.addstr(format!("{}\n", path.display()));
                        }
                    });
            }
            self.files_pane.refresh();
            self.input_line.mv(0, self.input_line.get_cur_x());
        }

        self.input_line.clear();
        self.input_line.refresh();
        self.files_pane.clear();
        self.files_pane.refresh();
        result.to_path_buf()
    }
}
