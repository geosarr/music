#[cfg(test)]
mod unit_test;

use crate::Note;

/// Representation of a piano key sound.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Sound {
    note: Note,
    octave: usize,
}
impl Sound {
    /// Creates a representation of a piano key/sound.
    /// # Panics
    /// It panics when the octave is set to 0.
    /// ```
    /// use music::Sound;
    /// use music::Note;
    /// let note = Note::C;
    /// let sound = Sound::init(note, 2);
    /// assert_eq!(sound.note(), note);
    /// assert_eq!(sound.octave(), 2);
    /// ```
    pub fn init(note: Note, octave: usize) -> Self {
        assert!(octave > 0);
        Self { note, octave }
    }
    /// Gives the note of the sound.
    /// ```
    /// use music::Sound;
    /// use music::Note;
    /// let sound = Sound::init(Note::C, 1);
    /// assert_eq!(sound.note(), Note::C);
    /// ```
    pub fn note(&self) -> Note {
        self.note
    }
    /// Gives the octave of the sound.
    /// ```
    /// use music::Sound;
    /// use music::Note;
    /// let sound = Sound::init(Note::G, 3);
    /// assert_eq!(sound.octave(), 3);
    /// ```
    pub fn octave(&self) -> usize {
        self.octave
    }
    /// Gives the range of the key/sound, from 0 to +∞,
    /// 0 representing the lowest C note.
    /// ```
    /// use music::Sound;
    /// use music::Note;
    /// let sound = Sound::init(Note::Gb, 2);
    /// assert_eq!(sound.range(), 18);
    /// let sound = Sound::init(Note::C, 2);
    /// assert_eq!(sound.range(), 12);
    /// ```
    pub fn range(&self) -> usize {
        (self.octave - 1) * 12 + self.note.to_usize()
    }
}
impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        (self.note == other.note) & (self.octave == other.octave)
    }
}
impl PartialOrd for Sound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range().partial_cmp(&other.range())
    }
}
impl Ord for Sound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range().cmp(&other.range())
    }
}
