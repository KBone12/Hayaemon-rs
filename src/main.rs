extern crate pancurses;

use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;

mod bass;
use bass::{Bass, Mode};
use bass::device::Device;
use bass::music::Music;
mod curses;
use curses::window::Window;

fn main() {
    let mut device_number = 1;
    while let Some(device) = Device::new(device_number) {
        println!("{}", device);
        device_number += 1;
    }
    print!("Choose device > ");
    io::stdout().flush().ok();
    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).ok();
    device_number = tmp.trim().parse().ok().unwrap();

    let _bass = Bass::new(Device::new(device_number).unwrap_or(Device::new(0).unwrap()), 44100, &vec![Mode::None]);

    /*
    print!("Put the path to the sound file > ");
    io::stdout().flush().ok();
    tmp.clear();
    io::stdin().read_line(&mut tmp).ok();
    let music = Rc::new(Music::from_file(Path::new(&tmp.trim())));
    */

    let mut window = Window::new();
    // window.set_music(music);
    window.show();
}
