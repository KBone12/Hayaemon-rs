use pancurses::Input;

use bass::effect::Tempo;
use bass::music::State;
use curses::window::Window;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PaneType {
    Play,
    Effect,
}

pub trait Pane {
    fn draw(&self, window: &Window);
    fn input(&mut self, window: &mut Window, input: &Input);
    fn next(&mut self) -> PaneType;
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
        window.get_handle().mvaddstr(1, (width - 17) / 2, "E: Effect setting");
    }

    fn input(&mut self, _window: &mut Window, input: &Input) {
        if input == &Input::Character('E') {
            self.next = PaneType::Effect;
        }
    }

    fn next(&mut self) -> PaneType {
        let tmp = self.next;
        self.next = PaneType::Play;
        tmp
    }
}

pub struct EffectPane {
    next: PaneType,
    speed: f32,
}

impl EffectPane {
    pub fn new() -> Self {
        Self {
            next: PaneType::Effect,
            speed: 0.0
        }
    }
}

impl Pane for EffectPane {
    fn draw(&self, window: &Window) {
        let (height, width) = window.get_handle().get_max_yx();
        let texts = vec![
            format!("Speed: {:.0}%", self.speed + 100.0),
            "k: Speed UP".to_string(),
            "j: Speed DOWN".to_string(),
            "P: Playlist".to_string(),
        ];
        texts.iter().enumerate().for_each(|(index, text)| {
            window.get_handle().mvaddstr(height / 2 + index as i32 - texts.len() as i32, (width - text.len() as i32) / 2, text);
        });
    }

    fn input(&mut self, window: &mut Window, input: &Input) {
        match input {
            Input::Character('k') => {
                self.speed += 1.00;
                if let Some(music) = window.get_music() {
                    music.apply_effect(&Tempo::new(self.speed));
                }
            },
            Input::Character('j') => {
                self.speed -= 1.00;
                if let Some(music) = window.get_music() {
                    music.apply_effect(&Tempo::new(self.speed));
                }
            },
            Input::Character('P') => self.next = PaneType::Play,
            _ => {}
        };
    }

    fn next(&mut self) -> PaneType {
        let tmp = self.next;
        self.next = PaneType::Effect;
        tmp
    }
}
