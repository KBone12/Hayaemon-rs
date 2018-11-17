use bass::music::Music;

#[link(name = "bass")]
extern "C" {
    fn BASS_ChannelSetAttribute(handle: u32, attribute: u32, value: f32) -> bool;
}

#[link(name = "bass_fx")]
extern "C" {
    fn BASS_FX_TempoCreate(handle: u32, flags: u32) -> u32;
}

pub enum EffectType {
    Tempo
}

impl EffectType {
    pub fn enable(self, music: &mut Music) {
        match self {
            EffectType::Tempo => Tempo::enable(music),
        };
    }
}

pub trait Effect {
    fn apply(&self, music: &Music);
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

    pub fn enable(music: &mut Music) {
        unsafe {
            let current_handle = music.get_handle();
            music.set_handle(BASS_FX_TempoCreate(current_handle, 0x10000));    // BASS_FX_FREESOURCE
        }
    }
}

impl Effect for Tempo {
    fn apply(&self, music: &Music) {
        unsafe {
            BASS_ChannelSetAttribute(music.get_handle(), 0x10000, self.ratio);     // BASS_ATTRIB_TEMPO
        }
    }
}
