use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::os::raw::{c_char, c_void};
use std::ptr;

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
        BASS_Free();
    }
}
