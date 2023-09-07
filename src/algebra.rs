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
                let int_note = accidental.add_to_int_note(self.to_usize());
                Note::from_usize(int_note).unwrap()
            }
        }
        impl std::ops::Add<Note> for $Accidental {
            type Output = Note;
            fn add(self, note: Note) -> Self::Output {
                note + self
            }
        }
        impl std::ops::Add<&$Accidental> for &Note {
            type Output = Note;
            fn add(self, accidental: &$Accidental) -> Self::Output {
                let int_note = accidental.add_to_int_note(self.to_usize());
                Note::from_usize(int_note).unwrap()
            }
        }
        impl std::ops::Add<&Note> for &$Accidental {
            type Output = Note;
            fn add(self, note: &Note) -> Self::Output {
                note + self
            }
        }
        impl std::ops::Add<$Accidental> for Sound {
            type Output = Sound;
            fn add(self, accidental: $Accidental) -> Self::Output {
                let note = self.note() + &accidental;
                let diff = self.octave() - (accidental.number() / 12);
                println!("{}", type_of(&accidental));
                let octave = match type_of(&accidental) {
                    "music::accidental::Sharp" => self.octave() + (accidental.number() / 12),
                    "music::accidental::Bemol" => if diff > 0 {diff} else {panic!("Too many bemols (♭).")},
                    "music::accidental::Natural" => *(self.octave()),
                    _ => panic!("Not implemented"),

                };
                Sound::init(note, octave)
            }
        }
        impl std::ops::Add<Sound> for $Accidental {
            type Output = Sound;
            fn add(self, note: Sound) -> Self::Output {
                note + self
            }
        }


    )*);
}
operation! {Sharp Bemol}

impl std::ops::Add<Natural> for Note {
    type Output = Note;
    fn add(self, _: Natural) -> Self::Output {
        match self {
            Note::As => Note::A,
            Note::Ab => Note::A,
            Note::Bs => Note::B,
            Note::Bb => Note::B,
            Note::Cs => Note::C,
            Note::Cb => Note::C,
            Note::Ds => Note::D,
            Note::Db => Note::D,
            Note::Es => Note::E,
            Note::Eb => Note::E,
            Note::Fb => Note::F,
            Note::Fs => Note::F,
            Note::Gs => Note::G,
            Note::Gb => Note::G,
            _ => self,
        }
    }
}
impl std::ops::Add<Note> for Natural {
    type Output = Note;
    fn add(self, note: Note) -> Self::Output {
        note + self
    }
}

impl<'a, 'b> std::ops::Add<&'a Natural> for &'b Note {
    type Output = &'b Note;
    fn add(self, _: &'a Natural) -> Self::Output {
        match self {
            Note::As => &Note::A,
            Note::Ab => &Note::A,
            Note::Bs => &Note::B,
            Note::Bb => &Note::B,
            Note::Cs => &Note::C,
            Note::Cb => &Note::C,
            Note::Ds => &Note::D,
            Note::Db => &Note::D,
            Note::Es => &Note::E,
            Note::Eb => &Note::E,
            Note::Fb => &Note::F,
            Note::Fs => &Note::F,
            Note::Gs => &Note::G,
            Note::Gb => &Note::G,
            _ => self,
        }
    }
}
impl<'a: 'b, 'b> std::ops::Add<&'a Note> for &'b Natural {
    type Output = &'b Note;
    fn add(self, note: &'a Note) -> Self::Output {
        note + self
    }
}
