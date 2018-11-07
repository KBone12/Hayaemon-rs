extern crate pancurses;
use pancurses::Input;

fn main() {
    let window = pancurses::initscr();

    pancurses::set_title("Hayaemon-rs");
    pancurses::cbreak();
    pancurses::curs_set(0);
    pancurses::noecho();

    window.nodelay(true);

    let mut should_close = false;
    while !should_close {
        let (height, width) = window.get_max_yx();
        let text = "Play Pane";

        window.mvaddstr(height / 2, (width - text.len() as i32) / 2, text);
        window.refresh();

        while let Some(input) = window.getch() {
            match input {
                Input::Character('q') => should_close = true,
                _ => {}
            }
        }
    }
    pancurses::endwin();
}
