use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
struct DeviceInfo {
    name: *mut c_char,
    driver: *mut c_char,
    flags: u32,
}

#[link(name = "bass")]
extern "C" {
    fn BASS_GetDeviceInfo(device: u32, info: *mut DeviceInfo) -> bool;
}

#[derive(Debug)]
enum DeviceType {
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

impl DeviceType {
    pub fn from(value: u32) -> Vec<Self> {
        let mut result = Vec::new();

        if value & 1 == 1 { result.push(DeviceType::Enabled); }
        if value & 2 == 2 { result.push(DeviceType::Default); }
        if value & 4 == 4 { result.push(DeviceType::Initialized); }
        if value & 8 == 8 { result.push(DeviceType::Loopback); }
        if value & 0x01000000 == 0x01000000 { result.push(DeviceType::Network); }
        if value & 0x02000000 == 0x02000000 { result.push(DeviceType::Speakers); }
        if value & 0x03000000 == 0x03000000 { result.push(DeviceType::Line); }
        if value & 0x04000000 == 0x04000000 { result.push(DeviceType::Headphones); }
        if value & 0x05000000 == 0x05000000 { result.push(DeviceType::Microphone); }
        if value & 0x06000000 == 0x06000000 { result.push(DeviceType::Headset); }
        if value & 0x07000000 == 0x07000000 { result.push(DeviceType::Handset); }
        if value & 0x08000000 == 0x08000000 { result.push(DeviceType::Digital); }
        if value & 0x09000000 == 0x09000000 { result.push(DeviceType::SPDIF); }
        if value & 0x0a000000 == 0x0a000000 { result.push(DeviceType::HDMI); }
        if value & 0x40000000 == 0x40000000 { result.push(DeviceType::DisplayPort); }

        result
    }
}

pub struct Device {
    id: u32,
    info: DeviceInfo,
}

impl Device {
    pub fn new(id: u32) -> Option<Self> {
        let mut info = DeviceInfo {
            name: ptr::null_mut(),
            driver: ptr::null_mut(),
            flags: 0,
        };
        if unsafe { !BASS_GetDeviceInfo(id, &mut info) } {
            None
        } else {
            Some(Self { id, info, })
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Audio device: id={}, name={}, driver={}, flags={}",
               self.id,
               unsafe { CStr::from_ptr(self.info.name).to_string_lossy() },
               unsafe { CStr::from_ptr(self.info.driver).to_string_lossy() },
               DeviceType::from(self.info.flags).into_iter().map(|f| format!("{:?}", f)).collect::<Vec<_>>().join(", "))
    }
}
