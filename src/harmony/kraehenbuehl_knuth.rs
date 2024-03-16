// Algorithm taken from https://www.gathering4gardner.org/g4g14gift/G4G14-NeilBickford-AlgorithmsForMusicalHarmonization.pdf

use crate::{Chord, Flat, Note, Scale, Sharp, Sound};

use rand::prelude::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
pub struct KraehenbuehlKnuth {
    melody: Vec<Sound>,
    scale: Note,
    next_position: u8,
    seed: u64,
}

impl KraehenbuehlKnuth {
    pub fn init(melody: Vec<Sound>, scale: Option<Note>) -> Self {
        if let Some(_scale) = scale {
            Self {
                melody,
                next_position: 0,
                scale: _scale,
                seed: 0,
            }
        } else {
            let scale = Self::find_scale(melody.clone());
            Self {
                melody,
                next_position: 0,
                scale,
                seed: 0,
            }
        }
    }

    fn find_scale(melody: Vec<Sound>) -> Note {
        Note::C
    }

    pub fn sound_below(&self, sound: Sound, num: u8) -> Sound {
        sound + Flat::init(num)
    }

    fn get_chord(&mut self, sound: Sound) -> Vec<Sound> {
        let mut chord = vec![
            self.sound_below(sound, 11),
            self.sound_below(sound, 4),
            self.sound_below(sound, 2),
            sound,
        ];
        if self.next_position == 1 {
            chord = vec![
                self.sound_below(sound, 7),
                self.sound_below(sound, 5),
                self.sound_below(sound, 3),
                sound,
            ];
        } else if self.next_position == 2 {
            chord = vec![
                self.sound_below(sound, 9),
                self.sound_below(sound, 5),
                self.sound_below(sound, 2),
                sound,
            ];
        }
        let mut rng = ChaCha20Rng::seed_from_u64(self.seed);
        let random_number = rng.gen_range(0..=1); // equal to 0 or 1.
        self.next_position = (self.next_position + 1 + 2 * random_number) % 3;
        chord
    }

    pub fn harmonize(&mut self) -> Vec<Chord> {
        let mut harmonics = Vec::with_capacity(self.melody.len());
        for sound in self.melody.clone() {
            let chord = Chord::from_vec(self.get_chord(sound));
            harmonics.push(chord);
        }
        harmonics
    }
}
