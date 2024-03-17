#[cfg(test)]
mod tests {
    use crate::{Note, Scale, ScaleType, Sound};

    #[test]
    fn test_notes() {
        let scale = Scale::init(Note::C, ScaleType::Major);
        assert_eq!(vec![0, 2, 4, 5, 7, 9, 11], scale.usize_notes());
    }
    #[test]
    fn test_sounds() {
        let scale = Scale::init(Note::Bb, ScaleType::Major);
        let expected_scale_sounds = vec![
            Sound::init(Note::Bb, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::D, 5),
            Sound::init(Note::Eb, 5),
            Sound::init(Note::F, 5),
            Sound::init(Note::G, 5),
            Sound::init(Note::A, 5),
        ];
        assert_eq!(scale.sounds(4), expected_scale_sounds);

        let scale = Scale::init(Note::A, ScaleType::Minor);
        let expected_scale_sounds = vec![
            Sound::init(Note::A, 2),
            Sound::init(Note::B, 2),
            Sound::init(Note::C, 3),
            Sound::init(Note::D, 3),
            Sound::init(Note::E, 3),
            Sound::init(Note::F, 3),
            Sound::init(Note::Gs, 3),
        ];
        assert_eq!(scale.sounds(2), expected_scale_sounds);
    }
}
