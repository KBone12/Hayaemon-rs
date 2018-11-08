use pancurses::{self, Input};

pub struct Window {
    window: pancurses::Window,
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
        }
    }

    pub fn show(&self) {
        let mut should_close = false;
        while !should_close {
            let (height, width) = self.window.get_max_yx();
            let text = "Play Pane";

            self.window.mvaddstr(height / 2, (width - text.len() as i32) / 2, text);
            self.window.refresh();

            while let Some(input) = self.window.getch() {
                match input {
                    Input::Character('q') => should_close = true,
                    _ => {}
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
