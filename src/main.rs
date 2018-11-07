extern crate pancurses;

fn main() {
    let _ = pancurses::initscr();
    pancurses::endwin();
}
