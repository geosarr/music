use crate::sound::Sound;

pub struct Chord {
    sounds: Vec<Sound>,
}

impl Chord {
    pub fn init(num: usize) -> Self {
        Self {
            sounds: Vec::<Sound>::with_capacity(num),
        }
    }
    pub fn from_vec(sounds: Vec<Sound>) -> Self {
        Self { sounds }
    }
    pub fn sounds(&self) -> &Vec<Sound> {
        &self.sounds
    }
    pub fn to_fondamental(mut self) -> Self {
        self.sounds.sort();
        self
    }
    pub fn add_sound(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }
}
