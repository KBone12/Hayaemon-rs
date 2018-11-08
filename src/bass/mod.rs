use std::os::raw::c_void;
use std::ptr;

pub mod device;
use self::device::Device;
pub mod music;

mod error;
use self::error::ErrorType;

#[link(name = "bass")]
extern "C" {
    fn BASS_Init(device: i32, frequency: u32, flags: u32, window: *mut c_void, class_id: *mut c_void) -> bool;
    fn BASS_ErrorGetCode() -> i32;
    fn BASS_Free() -> bool;
}

#[allow(dead_code)]
pub enum Mode {
    None,
    Sound8Bits,
    Monoral,
    Audio3D,
    Sound16Bits,
    Latency,
    CPSpeakers,
    Speakers,
    NoSpeaker,
    DMIX,
    Frequency,
    Stereo,
    HOG,
    AudioTrack,
    DirectSound,
}

impl Mode {
    pub fn to_value(modes: &[Mode]) -> u32 {
        let mut result = 0;
        modes.iter().for_each(|mode| match mode {
            Mode::None => result |= 0,
            Mode::Sound8Bits => result |= 1,
            Mode::Monoral => result |= 2,
            Mode::Audio3D => result |= 4,
            Mode::Sound16Bits => result |= 8,
            Mode::Latency => result |= 0x100,
            Mode::CPSpeakers => result |= 0x400,
            Mode::Speakers => result |= 0x800,
            Mode::NoSpeaker => result |= 0x1000,
            Mode::DMIX => result |= 0x2000,
            Mode::Frequency => result |= 0x4000,
            Mode::Stereo => result |= 0x8000,
            Mode::HOG => result |= 0x10000,
            Mode::AudioTrack => result |= 0x20000,
            Mode::DirectSound => result |= 0x40000,
        });
        result
    }
}

pub struct Bass {
}

impl Bass {
    pub fn new(device: Device, frequency: u32, modes: &[Mode]) -> Self {
        if unsafe { !BASS_Init(device.get_id() as i32, frequency, Mode::to_value(modes), ptr::null_mut(), ptr::null_mut()) } {
            panic!(format!("Initializing error: {}",
                    ErrorType::from_value(unsafe { BASS_ErrorGetCode() })
                                .into_iter()
                                .map(|error| format!("{:?}", error))
                                .collect::<Vec<_>>()
                                .join(", ")
            ));
        }

        Self {
        }
    }
}

impl Drop for Bass {
    fn drop(&mut self) {
        unsafe { BASS_Free(); }
    }
}
