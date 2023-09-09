#[cfg(test)]
mod unit_test;

use crate::note::Note;

/// Representation of a piano key sound.
#[derive(Debug, Clone, Copy)]
pub struct Sound {
    note: Note,
    octave: usize,
}
impl Sound {
    pub fn init(note: Note, octave: usize) -> Self {
        assert!(octave > 0);
        Self { note, octave }
    }
    pub fn note(&self) -> Note {
        self.note
    }
    pub fn octave(&self) -> usize {
        self.octave
    }
    pub fn range(&self) -> usize {
        (self.octave - 1) * 12 + self.note.to_usize()
    }
}
impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        (self.note == other.note) & (self.octave == other.octave)
    }
}
