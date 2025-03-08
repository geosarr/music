#[cfg(test)]
mod unit_test;

use crate::Sound;

/// Implements an interval between two sounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    sound_one: Sound,
    sound_two: Sound,
}

impl Interval {
    /// Creates a representation of an interval of two sounds.
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::C, 2);
    /// let sound_two = Sound::init(Note::F, 3);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert_eq!(interval.first_sound(), sound_one);
    /// assert_eq!(interval.second_sound(), sound_two);
    /// ```
    pub fn init(sound_one: Sound, sound_two: Sound) -> Self {
        Self {
            sound_one,
            sound_two,
        }
    }
    /// Gives the first sound of the interval.
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::Cs, 2);
    /// let sound_two = Sound::init(Note::Bb, 3);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert_eq!(interval.first_sound(), sound_one);
    /// ```
    pub fn first_sound(&self) -> Sound {
        self.sound_one
    }
    /// Gives the second sound of the interval.
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::Cs, 2);
    /// let sound_two = Sound::init(Note::Bb, 3);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert_eq!(interval.second_sound(), sound_two);
    /// ```
    pub fn second_sound(&self) -> Sound {
        self.sound_two
    }
    /// Test whether or not the interval is strictly ascending (unison excluded).
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::As, 3);
    /// let sound_two = Sound::init(Note::As, 5);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert!(interval.is_ascending());
    /// let sound_one = Sound::init(Note::Bb, 4);
    /// let sound_two = Sound::init(Note::Bb, 4);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert!(!interval.is_ascending());
    /// let sound_one = Sound::init(Note::D, 5);
    /// let sound_two = Sound::init(Note::Bb, 4);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert!(!interval.is_ascending());
    /// ```
    pub fn is_ascending(&self) -> bool {
        self.sound_one < self.sound_two
    }
    /// Test whether or not the interval is in unison:
    /// that is when the two sounds making up the interval are equal.
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::Bb, 4);
    /// let sound_two = Sound::init(Note::Bb, 4);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert!(interval.is_in_unison());
    /// ```
    pub fn is_in_unison(&self) -> bool {
        self.sound_one == self.sound_two
    }
    /// Computes the length of the interval in half tones.
    /// ```
    /// use music::Interval;
    /// use music::Sound;
    /// use music::Note;
    /// let sound_one = Sound::init(Note::A, 1);
    /// let sound_two = Sound::init(Note::E, 3);
    /// let interval = Interval::init(sound_one, sound_two);
    /// assert_eq!(interval.distance(), 19);
    /// ```
    pub fn distance(&self) -> usize {
        if self.is_ascending() {
            self.sound_two.range() - self.sound_one.range()
        } else {
            self.sound_one.range() - self.sound_two.range()
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance().cmp(&other.distance())
    }
}
