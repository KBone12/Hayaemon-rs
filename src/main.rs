use std::io::{self, Write};
use std::os::raw::c_void;
#[cfg(unix)] use std::os::unix::ffi::OsStrExt;
#[cfg(windows)] use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::thread;
use std::time::Duration;

mod bass;
use bass::{Bass, Mode};
use bass::device::Device;

#[link(name = "bass")]
extern "C" {
    fn BASS_StreamCreateFile(from_memory: bool, location: *mut c_void, offset: u64, length: u64, flags: u32) -> u32;
    fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32);
}

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

    unsafe {
        print!("Put the path to the sound file > ");
        io::stdout().flush().ok();
        tmp.clear();
        io::stdin().read_line(&mut tmp).ok();
        #[cfg(windows)] let mut path = Path::new(&tmp.trim()).as_os_str().encode_wide().collect::<Vec<_>>();
        #[cfg(unix)] let mut path = Path::new(&tmp.trim()).as_os_str().as_bytes().to_vec();
        path.push(0);   // Add '\0' to the last
        let stream_handle = BASS_StreamCreateFile(false, path.as_mut_slice().as_mut_ptr() as *mut c_void, 0, 0, 0);

        BASS_ChannelPlay(stream_handle, false);
        while BASS_ChannelIsActive(stream_handle) != 0 {
            thread::sleep(Duration::from_millis(10));
        }

        BASS_StreamFree(stream_handle);
    }
}
