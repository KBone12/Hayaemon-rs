extern crate pancurses;

mod window;
use window::Window;

fn main() {
    let window = Window::new();
    window.show();
}
