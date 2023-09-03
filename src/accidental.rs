#[cfg(test)]
mod unit_test;
use std::fmt;

/// Implements the bemol symbol
pub struct Bemol {
    number: u8, // number of bemols to represent
}
impl Bemol {
    /// Creates bemol symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::accidental::Bemol;
    /// let bemol = Bemol::init(3);
    /// for char in bemol.to_string().chars(){
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
    /// use music::accidental::Bemol;
    /// let bemol = Bemol::init(3);
    /// assert_eq!(bemol.number(), 3);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts bemols to string.
    /// ```
    /// use music::accidental::Bemol;
    /// let bemol = Bemol::init(10);
    /// assert!(!bemol.to_string().chars().any(|char| char!='♭'));
    /// ```
    pub fn to_string(&self) -> String {
        "♭".repeat(self.number())
    }
}

impl fmt::Display for Bemol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bemol = Self::to_string(&self);
        write!(f, "{bemol}")
    }
}

pub struct Sharp {
    number: u8, // number of sharps to represent
}
impl Sharp {
    /// Creates sharp symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::accidental::Sharp;
    /// let sharp = Sharp::init(3);
    /// for char in sharp.to_string().chars(){
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
    /// use music::accidental::Sharp;
    /// let sharp = Sharp::init(5);
    /// assert_eq!(sharp.number(), 5);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts sharps to string.
    /// ```
    /// use music::accidental::Sharp;
    /// let sharp = Sharp::init(10);
    /// assert!(!sharp.to_string().chars().any(|char| char!='#'));
    /// ```
    pub fn to_string(&self) -> String {
        "#".repeat(self.number())
    }
}

impl fmt::Display for Sharp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sharp = Self::to_string(&self);
        write!(f, "{sharp}")
    }
}
pub struct Natural {
    number: u8, // number of sharps to represent
}
impl Natural {
    /// Creates natural symbols concatenated a given number of times.
    /// # Panics
    /// It panics when the number of symbols is set to 0.
    /// ```
    /// use music::accidental::Natural;
    /// let natural = Natural::init(2);
    /// for char in natural.to_string().chars(){
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
    /// use music::accidental::Natural;
    /// let natural = Natural::init(6);
    /// assert_eq!(natural.number(), 6);
    /// ```
    pub fn number(&self) -> usize {
        self.number as usize
    }
    /// Converts naturals to string.
    /// ```
    /// use music::accidental::Natural;
    /// let natural = Natural::init(10);
    /// assert!(!natural.to_string().chars().any(|char| char!='♮'));
    /// ```
    pub fn to_string(&self) -> String {
        "♮".repeat(self.number())
    }
}
impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sharp = Self::to_string(&self);
        write!(f, "{sharp}")
    }
}
