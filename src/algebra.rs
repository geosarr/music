use crate::{Bemol, Chord, Interval, Natural, Note, Sharp, Sound};

use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

macro_rules! operation {
    ($($Accidental:ty)*) => ($(
        impl std::ops::Add<$Accidental> for Note {
            type Output = Note;
            fn add(self, accidental: $Accidental) -> Self::Output {
                accidental.add_note(&self)
            }
        }
        impl std::ops::Add<Note> for $Accidental {
            type Output = Note;
            fn add(self, note: Note) -> Self::Output {
                note + self // commutative operation
            }
        }
        impl std::ops::Add<&$Accidental> for &Note {
            type Output = Note;
            fn add(self, accidental: &$Accidental) -> Self::Output {
                accidental.add_note(self)
            }
        }
        impl std::ops::Add<&Note> for &$Accidental {
            type Output = Note;
            fn add(self, note: &Note) -> Self::Output {
                note + self // commutative operation
            }
        }
        impl std::ops::Add<$Accidental> for Sound {
            type Output = Sound;
            fn add(self, accidental: $Accidental) -> Self::Output {
                let expected_range = match type_of(&accidental) {
                    "music::accidental::Sharp" => {
                        self.range() + accidental.number()
                    },
                    "music::accidental::Bemol" => {
                        if self.range() < accidental.number() {
                            panic!("Too much bemol to add.")
                        } else {
                            self.range() - accidental.number()
                        }
                    }
                    "music::accidental::Natural" => self.range(),
                    _ => panic!("Not implemented"),

                };
                let note = Note::from_usize(expected_range % 12);
                let octave = 1 + (expected_range / 12);
                Sound::init(note, octave)
            }
        }
        impl std::ops::Add<Sound> for $Accidental {
            type Output = Sound;
            fn add(self, note: Sound) -> Self::Output {
                note + self // commutative operation
            }
        }
        impl std::ops::Add<&$Accidental> for &Sound {
            type Output = Sound;
            fn add(self, accidental: &$Accidental) -> Self::Output {
                *self + *accidental
            }
        }
        impl std::ops::Add<&Sound> for &$Accidental {
            type Output = Sound;
            fn add(self, sound: &Sound) -> Self::Output {
                *self + *sound
            }
        }
        impl std::ops::Add<$Accidental> for Interval {
            type Output = Interval;
            fn add(self, accidental: $Accidental) -> Self::Output {
                // Translating accidents to the sounds of the interval.
                let expected_sound_one = self.first_sound() + accidental;
                let expected_sound_two = self.second_sound() + accidental;
                Interval::init(expected_sound_one, expected_sound_two)
            }
        }
        impl std::ops::Add<Interval> for $Accidental {
            type Output = Interval;
            fn add(self, interval: Interval) -> Self::Output {
                interval + self
            }
        }
        impl std::ops::Add<&$Accidental> for &Interval {
            type Output = Interval;
            fn add(self, accidental: &$Accidental) -> Self::Output {
                // Translating accidents to the sounds of the interval.
                *self + *accidental
            }
        }
        impl std::ops::Add<&Interval> for &$Accidental {
            type Output = Interval;
            fn add(self, interval: &Interval) -> Self::Output {
                interval + self
            }
        }
        impl std::ops::Add<$Accidental> for Chord {
            type Output = Chord;
            fn add(self, accidental: $Accidental) -> Self::Output {
                // Translating accidents to the sounds of the Chord.
                let expected_sounds = self.sounds()
                                        .iter()
                                        .map(|sound| sound + &accidental)
                                        .collect::<Vec<Sound>>();
                Chord::from_vec(expected_sounds)
            }
        }
        impl std::ops::Add<Chord> for $Accidental {
            type Output = Chord;
            fn add(self, chord: Chord) -> Self::Output {
                chord + self
            }
        }
        impl std::ops::Add<&$Accidental> for &Chord {
            type Output = Chord;
            fn add(self, accidental: &$Accidental) -> Self::Output {
                // Translating accidents to the sounds of the interval.
                let expected_sounds = self.sounds()
                                        .iter()
                                        .map(|sound| sound + accidental)
                                        .collect::<Vec<Sound>>();
                Chord::from_vec(expected_sounds)
            }
        }
        impl std::ops::Add<&Chord> for &$Accidental {
            type Output = Chord;
            fn add(self, chord: &Chord) -> Self::Output {
                chord + self
            }
        }

    )*);
}
operation! {Sharp Bemol Natural}
