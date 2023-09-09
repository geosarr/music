#[cfg(test)]
mod tests {
    use super::super::Interval;
    use crate::{
        accidental::{Bemol, Natural, Sharp},
        note::Note,
        sound::Sound,
    };

    #[test]
    fn test_interval_algebra() {
        let sound_one = Sound::init(Note::A, 3);
        let sound_two = Sound::init(Note::Gs, 2);
        let interval = Interval::init(sound_one, sound_two);
        let accident = Sharp::init(2);
        let expected_sound_one = Sound::init(Note::B, 3);
        let expected_sound_two = Sound::init(Note::As, 2);
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&interval + &accident, expected_interval);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&accident + &interval, expected_interval);

        let accident = Bemol::init(3);
        let expected_sound_one = Sound::init(Note::Fs, 3);
        let expected_sound_two = Sound::init(Note::F, 2);
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&interval + &accident, expected_interval);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&accident + &interval, expected_interval);

        let accident = Natural::init(3);
        let expected_sound_one = sound_one;
        let expected_sound_two = sound_two;
        let expected_interval = Interval::init(expected_sound_one, expected_sound_two);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&interval + &accident, expected_interval);
        assert_eq!(interval + accident, expected_interval);
        assert_eq!(&accident + &interval, expected_interval);
    }
}
