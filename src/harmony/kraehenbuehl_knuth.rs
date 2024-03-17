// Algorithm taken from https://www.gathering4gardner.org/g4g14gift/G4G14-NeilBickford-AlgorithmsForMusicalHarmonization.pdf

use crate::{Chord, Scale, Sound};

use rand::prelude::*;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
pub struct KraehenbuehlKnuth {
    melody: Vec<Sound>,
    scale: Scale,
    next_position: u8,
    seed: u64,
    scale_range: Vec<Sound>,
}

impl KraehenbuehlKnuth {
    pub fn init(melody: Vec<Sound>, scale: Option<Scale>) -> Self {
        if let Some(_scale) = scale {
            Self {
                melody,
                next_position: 0,
                scale: _scale,
                seed: 0,
                scale_range: Vec::new(),
            }
        } else {
            let scale = Self::find_scale(melody.clone());
            Self {
                melody,
                next_position: 0,
                scale,
                seed: 0,
                scale_range: Vec::new(),
            }
        }
    }

    fn find_scale(melody: Vec<Sound>) -> Scale {
        Scale::default()
    }

    pub fn sound_below(&self, sound: Sound, number: usize) -> Sound {
        let sound_position = self
            .scale_range
            .binary_search(&sound)
            .expect("Failed to find sound");
        return self.scale_range[sound_position - number];
    }

    fn adjust_bass(&mut self, chord: &mut Vec<Sound>) {
        let bass_note = chord[0].note();
        let scale_notes = self.scale.notes();
        let leading_tone = scale_notes[scale_notes.len() - 1];
        if bass_note == leading_tone {
            chord[0] = self.sound_below(chord[0], 2);
        }
    }

    fn get_chord(&mut self, sound: Sound) -> Chord {
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
        self.adjust_bass(&mut chord);
        let mut rng = ChaCha20Rng::seed_from_u64(self.seed);
        let random_number = rng.gen_range(0..=1); // equal to 0 or 1.
        println!("{} {}", self.next_position, random_number);
        self.next_position = (self.next_position + 1 + 2 * random_number) % 3;
        Chord::from_vec(chord)
    }

    fn initialise_range(&mut self) {
        self.scale_range = self.scale.sounds(1);
        self.scale_range.extend_from_slice(&self.scale.sounds(2));
        self.scale_range.extend_from_slice(&self.scale.sounds(3));
        self.scale_range.extend_from_slice(&self.scale.sounds(4));
    }

    fn add_octaves(&mut self, octave: usize) {
        let largest_octave = self.scale_range[self.scale_range.len() - 1].octave();
        if octave > largest_octave {
            for oct in largest_octave + 1..octave + 1 {
                self.scale_range.extend_from_slice(&self.scale.sounds(oct));
            }
        }
    }

    pub fn harmonize(&mut self) -> Vec<Chord> {
        self.initialise_range();
        let mut harmonics = Vec::with_capacity(self.melody.len());
        for sound in self.melody.clone() {
            self.add_octaves(sound.octave());
            harmonics.push(self.get_chord(sound));
        }
        harmonics
    }
}
