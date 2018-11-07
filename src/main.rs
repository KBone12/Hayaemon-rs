use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::os::raw::{c_char, c_void};
#[cfg(unix)] use std::os::unix::ffi::OsStrExt;
#[cfg(windows)] use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum BassDeviceInfoFlag {
    Enabled,
    Default,
    Initialized,
    Loopback,
    Network,
    Speakers,
    Line,
    Headphones,
    Microphone,
    Headset,
    Handset,
    Digital,
    SPDIF,
    HDMI,
    DisplayPort,
}

impl BassDeviceInfoFlag {
    pub fn from(value: u32) -> Vec<Self> {
        let mut result = Vec::new();
        if value & 1 == 1 {
            result.push(BassDeviceInfoFlag::Enabled);
        }
        if value & 2 == 2 {
            result.push(BassDeviceInfoFlag::Default);
        }
        if value & 4 == 4 {
            result.push(BassDeviceInfoFlag::Initialized);
        }
        if value & 8 == 8 {
            result.push(BassDeviceInfoFlag::Loopback);
        }
        if value & 0x01000000 == 0x01000000 {
            result.push(BassDeviceInfoFlag::Network);
        }
        if value & 0x02000000 == 0x02000000 {
            result.push(BassDeviceInfoFlag::Speakers);
        }
        if value & 0x03000000 == 0x03000000 {
            result.push(BassDeviceInfoFlag::Line);
        }
        if value & 0x04000000 == 0x04000000 {
            result.push(BassDeviceInfoFlag::Headphones);
        }
        if value & 0x05000000 == 0x05000000 {
            result.push(BassDeviceInfoFlag::Microphone);
        }
        if value & 0x06000000 == 0x06000000 {
            result.push(BassDeviceInfoFlag::Headset);
        }
        if value & 0x07000000 == 0x07000000 {
            result.push(BassDeviceInfoFlag::Handset);
        }
        if value & 0x08000000 == 0x08000000 {
            result.push(BassDeviceInfoFlag::Digital);
        }
        if value & 0x09000000 == 0x09000000 {
            result.push(BassDeviceInfoFlag::SPDIF);
        }
        if value & 0x0a000000 == 0x0a000000 {
            result.push(BassDeviceInfoFlag::HDMI);
        }
        if value & 0x40000000 == 0x40000000 {
            result.push(BassDeviceInfoFlag::DisplayPort);
        }

        result
    }
}

#[repr(C)]
#[derive(Debug)]
struct BassDeviceInfo {
    name: *mut c_char,
    driver: *mut c_char,
    flags: u32,
}

impl BassDeviceInfo {
    pub fn new() -> Self {
        Self {
            name: &mut 0,
            driver: &mut 0,
            flags: 0,
        }
    }
}

impl Display for BassDeviceInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "BassDeviceInfo {{ name: {}, driver: {}, flags: {} }}",
               unsafe { CStr::from_ptr(self.name).to_string_lossy() },
               unsafe { CStr::from_ptr(self.driver).to_string_lossy() },
               BassDeviceInfoFlag::from(self.flags).into_iter().map(|f| format!("{:?}", f)).collect::<Vec<_>>().join(", "))
    }
}

#[link(name = "bass")]
extern "C" {
    fn BASS_GetDeviceInfo(device: u32, info: *mut BassDeviceInfo) -> bool;
    fn BASS_Init(device: i32, frequency: u32, flags: u32, window: *mut c_void, class_id: *mut c_void) -> bool;
    fn BASS_StreamCreateFile(from_memory: bool, location: *mut c_void, offset: u64, length: u64, flags: u32) -> u32;
    fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32);
    fn BASS_Free() -> bool;
}

fn main() {
    unsafe {
        let mut device_number = 1;
        let mut device_info = BassDeviceInfo::new();
        while BASS_GetDeviceInfo(device_number, &mut device_info) {
            println!("device={}: {}", device_number, device_info);
            device_number += 1;
        }
        print!("Choose device > ");
        io::stdout().flush().ok();
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).ok();
        device_number = tmp.trim().parse().ok().unwrap();

        BASS_Init(device_number as i32, 44100, 0, ptr::null_mut(), ptr::null_mut());

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
        BASS_Free();
    }
}
