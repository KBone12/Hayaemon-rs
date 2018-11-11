use bass::music::Music;

pub trait Effect {
    fn apply(&self, music: &mut Music);
}
