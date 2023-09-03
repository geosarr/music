use crate::note::Note;

// Piano convention
#[derive(Debug)]
pub struct Sound {
    note: Note,
    octave: u8,
}
impl Sound {
    pub fn init(note: Note, octave: u8) -> Self {
        assert!(octave > 0);
        Self { note, octave }
    }
    pub fn note(&self) -> &Note {
        &self.note
    }
    pub fn octave(&self) -> &u8 {
        &self.octave
    }
}
