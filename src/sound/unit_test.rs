#[cfg(test)]
mod tests {
    use super::super::Sound;
    use crate::accidental::{Bemol, Natural, Sharp};
    use crate::note::Note;

    #[test]
    fn test_sound_algebra() {
        let sound = Sound::init(Note::D, 2);
        let expected_sound = Sound::init(Note::D, 3);
        let accidental = Sharp::init(12);
        assert_eq!(sound + accidental, expected_sound);
        assert_eq!(&sound + &accidental, expected_sound);
        assert_eq!(accidental + sound, expected_sound);
        assert_eq!(&accidental + &sound, expected_sound);

        let sound = Sound::init(Note::Gs, 5);
        let expected_sound = Sound::init(Note::C, 6);
        let accidental = Sharp::init(4);
        assert_eq!(sound + accidental, expected_sound);
        assert_eq!(&sound + &accidental, expected_sound);
        assert_eq!(accidental + sound, expected_sound);
        assert_eq!(&accidental + &sound, expected_sound);

        let sound = Sound::init(Note::Cs, 1);
        let expected_sound = Sound::init(Note::Gb, 1);
        let accidental = Sharp::init(5);
        assert_eq!(sound + accidental, expected_sound);
        assert_eq!(&sound + &accidental, expected_sound);
        assert_eq!(accidental + sound, expected_sound);
        assert_eq!(&accidental + &sound, expected_sound);

        let sound = Sound::init(Note::Cs, 2);
        let expected_sound = Sound::init(Note::G, 1);
        let accidental = Bemol::init(6);
        assert_eq!(sound + accidental, expected_sound);
        assert_eq!(&sound + &accidental, expected_sound);
        assert_eq!(accidental + sound, expected_sound);
        assert_eq!(&accidental + &sound, expected_sound);

        let sound = Sound::init(Note::Ab, 2);
        let expected_sound = sound;
        let accidental = Natural::init(6);
        assert_eq!(sound + accidental, expected_sound);
        assert_eq!(&sound + &accidental, expected_sound);
        assert_eq!(accidental + sound, expected_sound);
        assert_eq!(&accidental + &sound, expected_sound);
    }
}
