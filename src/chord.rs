#[cfg(test)]
mod unit_test;
use crate::Sound;

/// Implementation of a chord (generalizing intervals).
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
    /// let b_flat_2 = Sound::init(Note::Bb, 2);
    /// let c_sharp_4 = Sound::init(Note::Cs, 4);
    /// let chord = Chord::from_vec(vec![b_flat_2, c_sharp_4]);
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
    /// let e_flat_5 = Sound::init(Note::Eb, 5);
    /// let chord = Chord::from_vec(vec![e_flat_5]);
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
    /// Sorts the sounds of the chord from lower to higher pitch.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let c_2 = Sound::init(Note::C, 2);
    /// let e_2 = Sound::init(Note::E, 2);
    /// let g_2 = Sound::init(Note::G, 2);
    /// let mut chord = Chord::from_vec(vec![e_2, g_2, c_2]);
    /// chord.sort();
    /// let sorted_chord = Chord::from_vec(vec![c_2, e_2, g_2]);
    /// assert_eq!(chord, sorted_chord);
    /// ```
    pub fn sort(&mut self) {
        self.sounds.sort();
    }
    /// Converts the chord into a chord with sorted sounds from lower to higher pitch.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let b_1 = Sound::init(Note::B, 1);
    /// let d_2 = Sound::init(Note::D, 2);
    /// let f_3 = Sound::init(Note::F, 3);
    /// let chord = Chord::from_vec(vec![d_2, b_1, f_3]);
    /// let sorted_chord = Chord::from_vec(vec![b_1, d_2, f_3]);
    /// assert_eq!(chord.to_sorted(), sorted_chord);
    /// ```
    pub fn to_sorted(mut self) -> Self {
        self.sort();
        self
    }
    /// Adds a sound to the chord.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let e_1 = Sound::init(Note::E, 1);
    /// let g_flat_2 = Sound::init(Note::Gb, 2);
    /// let mut chord = Chord::init(2);
    /// chord.add_sound(e_1);
    /// chord.add_sound(g_flat_2);
    /// assert_eq!(chord.len(), 2);
    /// ```
    pub fn add_sound(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }
    /// Gives the interval length (in half tone) between the lowest and highest pitch
    /// sound of the chord. It clones the sounds container to perform the computation.
    /// # Panics
    /// It panics when the chord container is empty.
    /// ```
    /// use music::{Chord, Sound, Note};
    /// let c_1 = Sound::init(Note::C, 1);
    /// let g_sharp_1 = Sound::init(Note::Gs, 1);
    /// let e_flat_1 = Sound::init(Note::Eb, 1);
    /// let mut chord = Chord::from_vec(vec![c_1, g_sharp_1, e_flat_1]);
    /// assert_eq!(chord.range(), 8);
    /// ```
    pub fn range(&self) -> usize {
        if !self.sounds.is_empty() {
            let mut sounds = self.sounds.clone();
            sounds.sort();
            sounds[sounds.len() - 1].range() - sounds[0].range()
        } else {
            panic!("Chord is empty.");
        }
    }
}
