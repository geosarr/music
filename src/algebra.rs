use crate::accidental::{Bemol, Natural, Sharp};
use crate::note::Note;
use crate::sound::Sound;

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
    )*);
}
operation! {Sharp Bemol Natural}
