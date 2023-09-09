#[cfg(test)]
mod unit_test;
use std::fmt;

use crate::note::Note;

/// Implements the bemol symbol ('♭').
#[derive(Debug, Clone, Copy)]
pub struct Bemol {
    number: u8, // number of bemols to represent
}
impl Bemol {
    /// Creates bemol symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::Bemol;
    /// let bemol = Bemol::init(3);
    /// for char in bemol.display().chars(){
    ///     assert_eq!(char, '♭');
    /// }
    /// println!("{bemol}");
    /// ```
    pub fn init(number: u8) -> Self {
        assert!(number > 0);
        Self { number }
    }
    /// Gives the number of bemols.
    /// ```
    /// use music::Bemol;
    /// let bemol = Bemol::init(3);
    /// assert_eq!(bemol.number(), 3);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts bemols to string.
    /// ```
    /// use music::Bemol;
    /// let bemol = Bemol::init(10);
    /// assert!(!bemol.display().chars().any(|char| char!='♭'));
    /// ```
    pub fn display(&self) -> String {
        "♭".repeat(self.number())
    }
    /// Adds a Note to the accidental.
    /// ```
    /// use music::Bemol;
    /// use music::Note;
    /// let bemol = Bemol::init(3);
    /// assert_eq!(Note::Eb + bemol, Note::C);
    /// ```
    pub fn add_note(&self, note: &Note) -> Note {
        let int_acc = self.number() % 12;
        let int_note = note.to_usize() % 12;
        let res_int_note = if int_note >= int_acc {
            int_note - int_acc
        } else {
            (int_note + 12 - int_acc) % 12
        };
        Note::from_usize(res_int_note)
    }
}

/// Implements the Sharp symbol ("#").
#[derive(Debug, Clone, Copy)]
pub struct Sharp {
    number: u8, // number of sharps to represent
}
impl Sharp {
    /// Creates sharp symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::Sharp;
    /// let sharp = Sharp::init(3);
    /// for char in sharp.display().chars(){
    ///     assert_eq!(char, '#');
    /// }
    /// println!("{sharp}");
    /// ```
    pub fn init(number: u8) -> Self {
        assert!(number > 0);
        Self { number }
    }
    /// Gives the number of sharps.
    /// ```
    /// use music::Sharp;
    /// let sharp = Sharp::init(5);
    /// assert_eq!(sharp.number(), 5);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts sharps to string.
    /// ```
    /// use music::Sharp;
    /// let sharp = Sharp::init(10);
    /// assert!(!sharp.display().chars().any(|char| char!='#'));
    /// ```
    pub fn display(&self) -> String {
        "#".repeat(self.number())
    }
    /// Adds a Note to the accidental.
    /// ```
    /// use music::Sharp;
    /// use music::Note;
    /// let sharp = Sharp::init(2);
    /// assert_eq!(Note::Ds + sharp, Note::F);
    /// ```
    pub fn add_note(&self, note: &Note) -> Note {
        let int_note = (self.number() + note.to_usize()) % 12;
        Note::from_usize(int_note)
    }
}

/// Implements the Sharp symbol ("♮").
#[derive(Debug, Clone, Copy)]
pub struct Natural {
    number: u8, // number of sharps to represent
}
impl Natural {
    /// Creates natural symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::Natural;
    /// let natural = Natural::init(2);
    /// for char in natural.display().chars(){
    ///     assert_eq!(char, '♮');
    /// }
    /// println!("{natural}");
    /// ```
    pub fn init(number: u8) -> Self {
        assert!(number > 0);
        Self { number }
    }
    /// Gives the number of naturals.
    /// ```
    /// use music::Natural;
    /// let natural = Natural::init(6);
    /// assert_eq!(natural.number(), 6);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts naturals to string.
    /// ```
    /// use music::Natural;
    /// let natural = Natural::init(10);
    /// assert!(!natural.display().chars().any(|char| char!='♮'));
    /// ```
    pub fn display(&self) -> String {
        "♮".repeat(self.number())
    }
    /// Adds a Note to the accidental.
    /// ```
    /// use music::Natural;
    /// use music::Note;
    /// let natural = Natural::init(2);
    /// assert_eq!(Note::Ds + natural, Note::D);
    /// ```
    pub fn add_note(&self, note: &Note) -> Note {
        match *note {
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
            _ => *note,
        }
    }
}
macro_rules! display_acc {
    ($($Accidental:ty)*) => ($(
        impl fmt::Display for $Accidental {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let accidental = Self::display(self);
                write!(f, "{accidental}")
            }
        }
    )*);
}
display_acc! {Sharp Bemol Natural}
