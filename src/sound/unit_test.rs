#[cfg(test)]
mod tests {
    use super::super::Sound;
    use crate::accidental::Sharp;
    use crate::note::Note;

    #[test]
    fn test_() {
        let sound = Sound::init(Note::D, 2);
        let expected_sound = Sound::init(Note::D, 3);
        assert_eq!(sound + Sharp::init(12), expected_sound);

        let sound = Sound::init(Note::Cs, 1);
        let expected_sound = Sound::init(Note::Gb, 1);
        assert_eq!(Sharp::init(5) + sound, expected_sound);
    }
}
