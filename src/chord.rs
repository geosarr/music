#[cfg(test)]
mod unit_test;
use crate::Sound;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    sounds: Vec<Sound>,
}

impl Chord {
    /// Creates an empty representation of a chord of sounds.
    /// ```
    /// use music::{Chord};
    /// let chord = Chord::init(2);
    /// assert!(chord.is_empty());
    /// ```
    pub fn init(num: usize) -> Self {
        Self {
            sounds: Vec::<Sound>::with_capacity(num),
        }
    }
    /// Gives the number of sounds in the chord.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let b_bemol_2 = Sound::init(Note::Bb, 2);
    /// let c_sharp_4 = Sound::init(Note::Cs, 4);
    /// let chord = Chord::from_vec(vec![b_bemol_2, c_sharp_4]);
    /// assert_eq!(chord.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.sounds.len()
    }
    /// Indicates whether or not the chord is empty (i.e. does not contain any sound).
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let a_1 = Sound::init(Note::A, 1);
    /// let chord = Chord::from_vec(vec![a_1]);
    /// assert!(!chord.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Creates a chord from a `Vec` of sounds.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let e_bemol_5 = Sound::init(Note::Eb, 5);
    /// let chord = Chord::from_vec(vec![e_bemol_5]);
    /// assert_eq!(chord.len(), 1);
    /// ```
    pub fn from_vec(sounds: Vec<Sound>) -> Self {
        Self { sounds }
    }
    /// Gives a reference to the sounds in the chord.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let d_1 = Sound::init(Note::D, 1);
    /// let chord = Chord::from_vec(vec![d_1]);
    /// assert_eq!(chord.sounds(), &vec![d_1]);
    /// ```
    pub fn sounds(&self) -> &Vec<Sound> {
        &self.sounds
    }
    /// Converts the chord in its "fondamental/natural" state.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let c_2 = Sound::init(Note::C, 2);
    /// let e_2 = Sound::init(Note::E, 2);
    /// let g_2 = Sound::init(Note::G, 2);
    /// let chord = Chord::from_vec(vec![e_2, g_2, c_2]);
    /// let fondamental_chord = Chord::from_vec(vec![c_2, e_2, g_2]);
    /// assert_eq!(chord.to_fondamental(), fondamental_chord);
    /// ```
    pub fn to_fondamental(mut self) -> Self {
        self.sounds.sort();
        self
    }
    pub fn add_sound(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }
    pub fn range(&self) -> usize {
        if !self.sounds.is_empty() {
            let mut sounds = self.sounds.clone();
            sounds.sort();
            sounds[sounds.len() - 1].range() - sounds[0].range()
        } else {
            panic!("Not enough sounds.");
        }
    }
}
