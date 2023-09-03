use crate::accidental::{Bemol, Natural, Sharp};
use crate::note::Note;
use crate::sound::Sound;

impl std::ops::Add<Sharp> for Note {
    type Output = Note;
    fn add(self, accidental: Sharp) -> Self::Output {
        let int_note = (accidental.number() + self.to_usize()) % 12;
        Note::from_usize(int_note).unwrap()
    }
}
impl std::ops::Add<Note> for Sharp {
    type Output = Note;
    fn add(self, note: Note) -> Self::Output {
        note + self
    }
}

impl<'a, 'b> std::ops::Add<&'a Sharp> for &'b Note {
    type Output = Note;
    fn add(self, accidental: &'a Sharp) -> Self::Output {
        let int_note = (accidental.number() + self.to_usize()) % 12;
        Note::from_usize(int_note).unwrap()
    }
}
impl<'a, 'b> std::ops::Add<&'a Note> for &'b Sharp {
    type Output = Note;
    fn add(self, note: &'a Note) -> Self::Output {
        note + self
    }
}

impl std::ops::Add<Bemol> for Note {
    type Output = Note;
    fn add(self, accidental: Bemol) -> Self::Output {
        let mut res_int_note = 0;
        let int_acc = accidental.number() % 12;
        let int_note = self.to_usize() % 12;
        if int_note >= int_acc {
            res_int_note = int_note - int_acc;
        } else {
            res_int_note = (int_note + 12 - int_acc) % 12;
        };
        Note::from_usize(res_int_note).unwrap()
    }
}
impl std::ops::Add<Note> for Bemol {
    type Output = Note;
    fn add(self, note: Note) -> Self::Output {
        note + self
    }
}

impl<'a, 'b> std::ops::Add<&'a Bemol> for &'b Note {
    type Output = Note;
    fn add(self, accidental: &'a Bemol) -> Self::Output {
        let mut res_int_note = 0;
        let int_acc = accidental.number() % 12;
        let int_note = self.to_usize() % 12;
        if int_note >= int_acc {
            res_int_note = int_note - int_acc;
        } else {
            res_int_note = (int_note + 12 - int_acc) % 12;
        };
        Note::from_usize(res_int_note).unwrap()
    }
}
impl<'a, 'b> std::ops::Add<&'a Note> for &'b Bemol {
    type Output = Note;
    fn add(self, note: &'a Note) -> Self::Output {
        note + self
    }
}

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

impl std::ops::Add<Sharp> for Sound {
    type Output = Sound;
    fn add(self, accidental: Sharp) -> Self::Output {
        let note = self.note() + &accidental;
        let octave = self.octave() + (accidental.number() / 12);
        Sound::init(note, octave)
    }
}
impl std::ops::Add<Sound> for Sharp {
    type Output = Sound;
    fn add(self, sound: Sound) -> Self::Output {
        sound + self
    }
}
