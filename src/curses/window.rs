use std::rc::Rc;

use pancurses::{self, Input};

use curses::file_explorer::FileExplorer;
use bass::music::{Music, State};

pub struct Window {
    window: pancurses::Window,
    music: Option<Rc<Music>>,
}

impl Window {
    pub fn new() -> Self {
        let window = pancurses::initscr();

        pancurses::set_title("Hayaemon-rs");
        pancurses::cbreak();
        pancurses::curs_set(0);
        pancurses::noecho();

        window.nodelay(true);

        Self {
            window,
            music: None,
        }
    }

    pub fn show(&self) {
        let mut should_close = false;
        while !should_close {
            let (height, width) = self.window.get_max_yx();
            let text = "Press the space key to toggle the music playing/stopped.";

            self.window.mvaddstr(height / 2, (width - text.len() as i32) / 2, text);
            self.window.refresh();

            while let Some(input) = self.window.getch() {
                match input {
                    Input::Character('q') => should_close = true,
                    Input::Character(' ') => {
                        if let Some(ref music) = self.music {
                            if music.get_state() == State::Playing {
                                music.pause();
                            } else {
                                music.play(true);
                            }
                        }
                    },
                    Input::Character('F') => {
                        self.window.mv(0, 0);
                        self.window.clrtoeol();
                        let path = FileExplorer::new(&self.window).get_file();
                        self.window.mvaddstr(0, 0, path.to_string_lossy());
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn set_music(&mut self, music: Rc<Music>) {
        self.music = Some(music);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
