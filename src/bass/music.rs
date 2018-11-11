use std::os::raw::c_void;
#[cfg(unix)] use std::os::unix::ffi::OsStrExt;
#[cfg(windows)] use std::os::windows::ffi::OsStrExt;
use std::path::Path;

#[link(name = "bass")]
extern "C" {
    fn BASS_StreamCreateFile(from_memory: bool, location: *mut c_void, offset: u64, length: u64, flags: u32) -> u32;
    fn BASS_ChannelPlay(handle: u32, restart: bool) -> bool;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32);
}

pub struct Music {
    handle: u32,
}

impl Music {
    pub fn from_file(path: &Path) -> Self {
        #[cfg(windows)] let mut path = Path::new(path).as_os_str().encode_wide().collect::<Vec<_>>();
        #[cfg(unix)] let mut path = Path::new(path).as_os_str().as_bytes().to_vec();
        path.push(0);   // Add '\0' to the last
        let stream_handle = unsafe { BASS_StreamCreateFile(false, path.as_mut_slice().as_mut_ptr() as *mut c_void, 0, 0, 0) };
        Self {
            handle: stream_handle
        }
    }

    pub fn play(&self) {
        unsafe { BASS_ChannelPlay(self.handle, false); }
    }

    pub fn apply_effect(&mut self, effect: &Effect) {
        effect.apply(&mut self);
    }

    pub fn is_active(&self) -> bool {
        unsafe { BASS_ChannelIsActive(self.handle) != 0 }
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe { BASS_StreamFree(self.handle); }
    }
}
