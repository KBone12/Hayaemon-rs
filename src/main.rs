use std::os::raw::c_void;
use std::ptr;

#[link(name = "bass")]
extern "C" {
    fn BASS_Init(device: i32, frequency: u32, flags: u32, window: *mut c_void, class_id: *mut c_void) -> bool;
    fn BASS_Free() -> bool;
}

fn main() {
    unsafe {
        BASS_Init(-1, 44100, 0, ptr::null_mut(), ptr::null_mut());
        BASS_Free();
    }
}
