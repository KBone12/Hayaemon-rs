extern crate pancurses;

use std::cell::RefCell;
use std::io::{self, Write};

mod bass;
use bass::{Bass, Mode};
use bass::device::Device;
use bass::effect::Tempo;
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

    let mut window = Window::new();
    window.show();
}
