use bass::music::Music;

#[link(name = "bass")]
extern "C" {
    fn BASS_ChannelSetAttribute(handle: u32, attribute: u32, value: f32) -> bool;
}

#[link(name = "bass_fx")]
extern "C" {
    fn BASS_FX_TempoCreate(handle: u32, flags: u32) -> u32;
}

pub trait Effect {
    fn apply(&self, music: &mut Music);
}

pub struct Tempo {
    ratio: f32,
}

impl Tempo {
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio
        }
    }
}

impl Effect for Tempo {
    fn apply(&self, music: &mut Music) {
        unsafe {
            *music.handle() = BASS_FX_TempoCreate(*music.handle(), 0x10000);    // BASS_FX_FREESOURCE
            BASS_ChannelSetAttribute(*music.handle(), 0x10000, self.ratio);     // BASS_ATTRIB_TEMPO
        }
    }
}
