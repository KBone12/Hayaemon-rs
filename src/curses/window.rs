use std::collections::HashMap;
use std::rc::Rc;

use pancurses::{self, Input};

use bass::music::{Music, State};
use curses::file_explorer::FileExplorer;
use curses::pane::{EffectPane, Pane, PaneType, PlaylistPane};

pub struct Window {
    window: pancurses::Window,
    panes: HashMap<PaneType, Box<Pane>>,
    current_pane: PaneType,
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

        let mut panes = HashMap::new();
        panes.insert(PaneType::Play, Box::new(PlaylistPane::new()) as Box<Pane>);
        panes.insert(PaneType::Effect, Box::new(EffectPane::new()) as Box<Pane>);

        Self {
            window,
            panes,
            current_pane: PaneType::Play,
            music: None,
        }
    }

    pub fn show(&mut self) {
        let mut should_close = false;
        while !should_close {
            self.window.clear();

            let (height, width) = self.window.get_max_yx();
            let texts = vec![
                "<Space>: Toggle the music playing/stopped",
                "'F': Choose the music file",
                "'q': Quit",
            ];
            texts.iter().enumerate().for_each(|(i, t)| {
                self.window.mvaddstr(height - (texts.len() - i) as i32, (width - t.len() as i32) / 2, t);
            });
            if let Some(ref pane) = self.panes.get(&self.current_pane) {
                pane.draw(&self);
            }

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
                        if path.exists() {
                            self.set_music(Rc::new(Music::from_file(&path)));
                        }
                    },
                    _ => {
                        if let Some(ref mut pane) = self.panes.get_mut(&self.current_pane) {
                            pane.input(&self.window, &input);
                        }
                    }
                }
            }

            if let Some(ref pane) = self.panes.get(&self.current_pane) {
                self.current_pane = pane.next();
            }
        }
    }

    pub fn get_handle(&self) -> &pancurses::Window {
        &self.window
    }

    pub fn get_music(&self) -> Option<Rc<Music>> {
        self.music.clone()
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
