#[cfg(test)]
mod tests {
    use super::super::Sound;
    use crate::accidental::{Bemol, Sharp};
    use crate::note::Note;

    #[test]
    fn test_sound_algebra() {
        let sound = Sound::init(Note::D, 2);
        let expected_sound = Sound::init(Note::D, 3);
        assert_eq!(sound + Sharp::init(12), expected_sound);

        let sound = Sound::init(Note::Cs, 1);
        let expected_sound = Sound::init(Note::Gb, 1);
        assert_eq!(sound + Sharp::init(5), expected_sound);

        let sound = Sound::init(Note::Cs, 2);
        let expected_sound = Sound::init(Note::G, 1);
        assert_eq!(sound + Bemol::init(6), expected_sound);
    }
}
