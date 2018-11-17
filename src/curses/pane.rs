use pancurses::{self, Input};

use bass::music::State;
use curses::window::Window;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PaneType {
    Play,
    Effect,
}

pub trait Pane {
    fn draw(&self, window: &Window);
    fn input(&mut self, window: &pancurses::Window, input: &Input);
    fn next(&self) -> PaneType;
}

pub struct PlaylistPane {
    next: PaneType,
}

impl PlaylistPane {
    pub fn new() -> Self {
        Self {
            next: PaneType::Play
        }
    }
}

impl Pane for PlaylistPane {
    fn draw(&self, window: &Window) {
        let (_, width) = window.get_handle().get_max_yx();
        if let Some(music) = window.get_music() {
            let text = format!("{}", if music.get_state() == State::Playing { "Playing!" } else { "Not playing" });
            window.get_handle().mvaddstr(0, (width - text.len() as i32) / 2, text);
        }
    }

    fn input(&mut self, window: &pancurses::Window, input: &Input) {
    }

    fn next(&self) -> PaneType {
        self.next
    }
}

pub struct EffectPane {
    next: PaneType,
}

impl EffectPane {
    pub fn new() -> Self {
        Self {
            next: PaneType::Effect
        }
    }
}

impl Pane for EffectPane {
    fn draw(&self, window: &Window) {
    }

    fn input(&mut self, window: &pancurses::Window, input: &Input) {
    }

    fn next(&self) -> PaneType {
        self.next
    }
}
