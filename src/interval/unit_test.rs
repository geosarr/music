#[cfg(test)]
mod tests {
    use super::super::Interval;
    use crate::{
        accidental::{Flat, Natural, Sharp},
        note::Note,
        sound::Sound,
    };

    #[test]
    fn test_interval_algebra() {
        let sound_one = Sound::init(Note::A, 3);
        let sound_two = Sound::init(Note::Gs, 2);
        let interval = Interval::init(sound_one, sound_two);
        let accidental = Sharp::init(2);
        let expected_sound_one = Sound::init(Note::B, 3);
        let expected_sound_two = Sound::init(Note::As, 2);
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&interval + &accidental, expected_interval);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&accidental + &interval, expected_interval);

        let accidental = Flat::init(3);
        let expected_sound_one = Sound::init(Note::Fs, 3);
        let expected_sound_two = Sound::init(Note::F, 2);
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&interval + &accidental, expected_interval);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&accidental + &interval, expected_interval);

        let accidental = Natural::init(3);
        let expected_sound_one = sound_one;
        let expected_sound_two = sound_two;
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&interval + &accidental, expected_interval);
        assert_eq!(interval + accidental, expected_interval);
        assert_eq!(&accidental + &interval, expected_interval);
    }
}
