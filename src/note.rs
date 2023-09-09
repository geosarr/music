#[cfg(test)]
mod unit_test;
use std::fmt;

/// Implementation of a note
#[derive(Debug, Clone, Copy, Eq)]
pub enum Note {
    A,
    As, // A#
    Bb, // B♭
    B,
    Bs, // B#
    Cb, // C♭
    C,
    Cs, // C#
    Db, // D♭
    D,
    Ds, // D#
    Eb, // E♭
    E,
    Es, // E#
    Fb, // F♭
    F,
    Fs, // F#
    Gb, // G♭
    G,
    Gs, // G#
    Ab, // A♭
}
impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.to_usize() == other.to_usize()
    }
}

impl Note {
    /// Converts the note to its integer representative
    /// that is beetween 0 and 11.
    /// ```
    /// use music::note::Note;
    /// assert_eq!(Note::Bb.to_usize(), 10);
    /// assert_eq!(Note::Ab.to_usize(), Note::Gs.to_usize());
    /// ```
    pub fn to_usize(&self) -> usize {
        match self {
            Note::C => 0,
            Note::Cs => 1,
            Note::Db => 1,
            Note::D => 2,
            Note::Ds => 3,
            Note::Eb => 3,
            Note::E => 4,
            Note::Fb => 4,
            Note::Es => 5,
            Note::F => 5,
            Note::Fs => 6,
            Note::Gb => 6,
            Note::G => 7,
            Note::Gs => 8,
            Note::Ab => 8,
            Note::A => 9,
            Note::As => 10,
            Note::Bb => 10,
            Note::B => 11,
            Note::Cb => 11,
            Note::Bs => 0,
        }
    }
    /// Converts an integer to a note.
    /// It computes the remainder of `num` in the euclidean division by 12 and maps it to a note.
    /// ```
    /// use music::note::Note;
    /// assert_eq!(Note::from_usize(13), Note::from_usize(1));
    /// ```
    pub fn from_usize(num: usize) -> Self {
        let num_remainder = num % 12;
        match num_remainder {
            0 => Note::C,
            1 => Note::Cs,
            2 => Note::D,
            3 => Note::Eb,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Fs,
            7 => Note::G,
            8 => Note::Ab,
            9 => Note::A,
            10 => Note::Bb,
            11 => Note::B,
            _ => panic!("Not implemented."),
        }
    }
    /// Tests whether or not the note is in C major.
    /// ```
    /// use music::note::Note;
    /// assert!(Note::C.is_in_c_major());
    /// assert!(!Note::Ds.is_in_c_major());
    /// assert!(Note::Fb.is_in_c_major());
    /// ```
    pub fn is_in_c_major(&self) -> bool {
        vec![0, 2, 4, 5, 7, 9, 11]
            .iter()
            .any(|num| num == &self.to_usize())
    }
    /// Computes the distance (in half tone) to upper C note.
    /// ```
    /// use music::note::Note;
    /// assert_eq!(Note::D.dist_to_upper_c(), 10);
    /// assert_eq!(Note::Fs.dist_to_upper_c(), 6);
    /// ```
    pub fn dist_to_upper_c(&self) -> usize {
        12 - self.to_usize()
    }
    /// Computes the distance  (in half tone) to lower C note.
    /// ```
    /// use music::note::Note;
    /// assert_eq!(Note::D.dist_to_lower_c(), 2);
    /// assert_eq!(Note::Fs.dist_to_lower_c(), 6);
    /// ```
    pub fn dist_to_lower_c(&self) -> usize {
        self.to_usize()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Note::C => write!(f, "C"),
            Note::Cs => write!(f, "C#"),
            Note::Db => write!(f, "D♭"),
            Note::D => write!(f, "D"),
            Note::Ds => write!(f, "D#"),
            Note::Eb => write!(f, "E♭"),
            Note::E => write!(f, "E"),
            Note::Fb => write!(f, "F♭"),
            Note::Es => write!(f, "E#"),
            Note::F => write!(f, "F"),
            Note::Fs => write!(f, "F#"),
            Note::Gb => write!(f, "G♭"),
            Note::G => write!(f, "G"),
            Note::Gs => write!(f, "G#"),
            Note::Ab => write!(f, "A♭"),
            Note::A => write!(f, "A"),
            Note::As => write!(f, "A#"),
            Note::Bb => write!(f, "B♭"),
            Note::B => write!(f, "B"),
            Note::Cb => write!(f, "C♭"),
            Note::Bs => write!(f, "B#"),
        }
    }
}
