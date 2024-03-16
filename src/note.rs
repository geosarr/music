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

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_usize().partial_cmp(&other.to_usize())
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

impl Note {
    /// Converts the note to its integer representative
    /// that is beetween 0 and 11.
    /// ```
    /// use music::Note;
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
    /// use music::Note;
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
    /// Gives the major scale whose tonic is the current note.
    pub fn major_scale_from_tonic(&self) -> Vec<usize> {
        vec![0, 2, 4, 5, 7, 9, 11]
            .iter()
            .map(|note| (note + self.to_usize()) % 12)
            .collect()
    }
    /// Gives the minor scale whose tonic is the current note.
    pub fn minor_scale_from_tonic(&self) -> Vec<usize> {
        vec![0, 2, 3, 5, 7, 8, 11]
            .iter()
            .map(|note| (note + self.to_usize()) % 12)
            .collect()
    }
    fn test_is_in_scale(&self, scale: Vec<usize>) -> bool {
        scale.iter().any(|num| num == &self.to_usize())
    }
    /// Tests whether or not the note is in a given major scale.
    /// ```
    /// use music::Note;
    /// assert!(Note::D.is_in_major_scale(Note::D));
    /// assert!(Note::E.is_in_major_scale(Note::D));
    /// assert!(Note::Fs.is_in_major_scale(Note::D));
    /// assert!(Note::G.is_in_major_scale(Note::D));
    /// assert!(Note::A.is_in_major_scale(Note::D));
    /// assert!(Note::B.is_in_major_scale(Note::D));
    /// assert!(Note::Cs.is_in_major_scale(Note::D));
    /// ```
    pub fn is_in_major_scale(&self, tonic: Note) -> bool {
        match tonic {
            Note::A => self.test_is_in_scale(Note::A.major_scale_from_tonic()),
            Note::As => self.test_is_in_scale(Note::As.major_scale_from_tonic()),
            Note::Bb => self.test_is_in_scale(Note::Bb.major_scale_from_tonic()),
            Note::B => self.test_is_in_scale(Note::B.major_scale_from_tonic()),
            Note::Bs => self.test_is_in_scale(Note::Bs.major_scale_from_tonic()),
            Note::Cb => self.test_is_in_scale(Note::Cb.major_scale_from_tonic()),
            Note::C => self.test_is_in_scale(Note::C.major_scale_from_tonic()),
            Note::Cs => self.test_is_in_scale(Note::Cs.major_scale_from_tonic()),
            Note::Db => self.test_is_in_scale(Note::Db.major_scale_from_tonic()),
            Note::D => self.test_is_in_scale(Note::D.major_scale_from_tonic()),
            Note::Ds => self.test_is_in_scale(Note::Ds.major_scale_from_tonic()),
            Note::Eb => self.test_is_in_scale(Note::Eb.major_scale_from_tonic()),
            Note::E => self.test_is_in_scale(Note::E.major_scale_from_tonic()),
            Note::Es => self.test_is_in_scale(Note::Es.major_scale_from_tonic()),
            Note::Fb => self.test_is_in_scale(Note::Fb.major_scale_from_tonic()),
            Note::F => self.test_is_in_scale(Note::F.major_scale_from_tonic()),
            Note::Fs => self.test_is_in_scale(Note::Fs.major_scale_from_tonic()),
            Note::Gb => self.test_is_in_scale(Note::Gb.major_scale_from_tonic()),
            Note::G => self.test_is_in_scale(Note::G.major_scale_from_tonic()),
            Note::Gs => self.test_is_in_scale(Note::Gs.major_scale_from_tonic()),
            Note::Ab => self.test_is_in_scale(Note::Ab.major_scale_from_tonic()),
        }
    }
    /// Tests whether or not the note is in a given minor scale.
    /// ```
    /// use music::Note;
    /// assert!(Note::A.is_in_minor_scale(Note::A));
    /// assert!(Note::B.is_in_minor_scale(Note::A));
    /// assert!(Note::C.is_in_minor_scale(Note::A));
    /// assert!(Note::D.is_in_minor_scale(Note::A));
    /// assert!(Note::E.is_in_minor_scale(Note::A));
    /// assert!(Note::F.is_in_minor_scale(Note::A));
    /// assert!(Note::Gs.is_in_minor_scale(Note::A));
    /// ```
    pub fn is_in_minor_scale(&self, tonic: Note) -> bool {
        match tonic {
            Note::A => self.test_is_in_scale(Note::A.minor_scale_from_tonic()),
            Note::As => self.test_is_in_scale(Note::As.minor_scale_from_tonic()),
            Note::Bb => self.test_is_in_scale(Note::Bb.minor_scale_from_tonic()),
            Note::B => self.test_is_in_scale(Note::B.minor_scale_from_tonic()),
            Note::Bs => self.test_is_in_scale(Note::Bs.minor_scale_from_tonic()),
            Note::Cb => self.test_is_in_scale(Note::Cb.minor_scale_from_tonic()),
            Note::C => self.test_is_in_scale(Note::C.minor_scale_from_tonic()),
            Note::Cs => self.test_is_in_scale(Note::Cs.minor_scale_from_tonic()),
            Note::Db => self.test_is_in_scale(Note::Db.minor_scale_from_tonic()),
            Note::D => self.test_is_in_scale(Note::D.minor_scale_from_tonic()),
            Note::Ds => self.test_is_in_scale(Note::Ds.minor_scale_from_tonic()),
            Note::Eb => self.test_is_in_scale(Note::Eb.minor_scale_from_tonic()),
            Note::E => self.test_is_in_scale(Note::E.minor_scale_from_tonic()),
            Note::Es => self.test_is_in_scale(Note::Es.minor_scale_from_tonic()),
            Note::Fb => self.test_is_in_scale(Note::Fb.minor_scale_from_tonic()),
            Note::F => self.test_is_in_scale(Note::F.minor_scale_from_tonic()),
            Note::Fs => self.test_is_in_scale(Note::Fs.minor_scale_from_tonic()),
            Note::Gb => self.test_is_in_scale(Note::Gb.minor_scale_from_tonic()),
            Note::G => self.test_is_in_scale(Note::G.minor_scale_from_tonic()),
            Note::Gs => self.test_is_in_scale(Note::Gs.minor_scale_from_tonic()),
            Note::Ab => self.test_is_in_scale(Note::Ab.minor_scale_from_tonic()),
        }
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
