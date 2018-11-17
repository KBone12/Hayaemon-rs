use std::cell::RefCell;
use std::rc::Rc;

use pancurses::{self, Input};

use bass::music::{Music, State};

pub struct Window {
    window: pancurses::Window,
    music: Option<Rc<RefCell<Music>>>,
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
                            if music.borrow().get_state() == State::Playing {
                                music.borrow().pause();
                            } else {
                                music.borrow().play(true);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn set_music(&mut self, music: Rc<RefCell<Music>>) {
        self.music = Some(music);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
