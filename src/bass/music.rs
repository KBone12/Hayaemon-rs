use std::os::raw::c_void;
#[cfg(unix)] use std::os::unix::ffi::OsStrExt;
#[cfg(windows)] use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use bass::effect::{Effect, EffectType};

#[link(name = "bass")]
extern "C" {
    fn BASS_StreamCreateFile(from_memory: bool, location: *mut c_void, offset: u64, length: u64, flags: u32) -> u32;
    fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;
    fn BASS_ChannelPause(handle: u32) -> bool;
    fn BASS_ChannelStop(handle: u32) -> bool;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32);
}

#[derive(PartialEq)]
pub enum PlayingState {
    Stopped,
    Playing,
    Stalled,
    Paused,
}

pub struct Music {
    handle: u32,
}

impl Music {
    pub fn from_file(path: &Path) -> Self {
        #[cfg(windows)] let mut path = Path::new(path).as_os_str().encode_wide().collect::<Vec<_>>();
        #[cfg(unix)] let mut path = Path::new(path).as_os_str().as_bytes().to_vec();
        path.push(0);   // Add '\0' to the last
        let stream_handle = unsafe {
            // 0x200000: Decoded stream
            // 0x80000000: The file's name is UTF-16 encoding (Windows wide char)
            if cfg!(target_os = "windows") {
                BASS_StreamCreateFile(false, path.as_mut_slice().as_mut_ptr() as *mut c_void, 0, 0,
                    0x200000 | 0x80000000)
            } else {
                BASS_StreamCreateFile(false, path.as_mut_slice().as_mut_ptr() as *mut c_void, 0, 0,
                    0x200000)
            }
        };
        Self {
            handle: stream_handle
        }
    }

    pub fn play(&self, from_head: bool) {
        unsafe { BASS_ChannelPlay(self.handle, from_head); }
    }

    pub fn pause(&self) {
        unsafe { BASS_ChannelPause(self.handle); }
    }

    pub fn stop(&self) {
        unsafe { BASS_ChannelStop(self.handle); }
    }

    pub fn get_state(&self) -> PlayingState {
        match unsafe { BASS_ChannelIsActive(self.handle) } {
            0 => PlayingState::Stopped,
            1 => PlayingState::Playing,
            2 => PlayingState::Stalled,
            3 => PlayingState::Paused,
            _ => unreachable!(),
        }
    }

    pub fn enable_effect(&mut self, effect_type: EffectType) {
        effect_type.enable(self);
    }

    pub fn apply_effect(&self, effect: &Effect) {
        effect.apply(self);
    }

    pub fn get_handle(&self) -> u32 {
        self.handle
    }

    pub fn set_handle(&mut self, handle: u32) {
        self.handle = handle;
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        if self.get_state() == PlayingState::Playing || self.get_state() == PlayingState::Paused { self.stop(); }
        unsafe { BASS_StreamFree(self.handle); }
    }
}
