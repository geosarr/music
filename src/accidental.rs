#[cfg(test)]
mod unit_test;
use std::fmt;
pub struct Bemol {
    number: u8, // number of bemols to represent
}
impl Bemol {
    pub fn init(number: u8) -> Self {
        assert!(number > 0);
        Self { number }
    }
    pub fn number(&self) -> usize {
        self.number as usize
    }
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
    pub fn init(number: u8) -> Self {
        assert!(number > 0);
        Self { number }
    }
    pub fn number(&self) -> usize {
        self.number as usize
    }
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
