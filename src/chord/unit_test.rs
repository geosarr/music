#[cfg(test)]
mod tests {
    use crate::{Chord, Note, Sharp, Sound};

    #[test]
    fn test_chord_algebra() {
        let sounds = vec![Sound::init(Note::E, 2), Sound::init(Note::D, 1)];
        let chord = Chord::from_vec(sounds);
        let expected_chord =
            Chord::from_vec(vec![Sound::init(Note::Fs, 2), Sound::init(Note::E, 1)]);
        let accidental = Sharp::init(2);
        assert_eq!(&chord + &accidental, expected_chord);
        assert_eq!(&accidental + &chord, expected_chord);
        assert_eq!(chord + accidental, expected_chord);
        // assert_eq!(accidental + chord, expected_chord);
    }
}
