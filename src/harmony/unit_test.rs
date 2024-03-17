#[cfg(test)]
mod tests {
    use crate::{KraehenbuehlKnuth, Note, Scale, ScaleType, Sound};

    #[test]
    fn test_kraehenbuel_knuth_with_scale() {
        let scale = Scale::init(Note::Eb, ScaleType::Major);
        let melody = vec![
            Sound::init(Note::C, 5),
            Sound::init(Note::C, 5),
            Sound::init(Note::C, 5),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::Ab, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::G, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Eb, 5),
            Sound::init(Note::C, 5),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::G, 4),
            Sound::init(Note::Ab, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Eb, 5),
            Sound::init(Note::Eb, 5),
            Sound::init(Note::Bb, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Ab, 4),
            Sound::init(Note::Ab, 4),
            Sound::init(Note::C, 5),
            Sound::init(Note::Bb, 4),
        ];
        let mut harmonizer = KraehenbuehlKnuth::init(melody, Some(scale));
        let harmonics: Vec<crate::Chord> = harmonizer.harmonize();
        for chord in &harmonics {
            println!("{:?}", chord);
        }
        // println!("{:?}", Note::C.major_scale_from_tonic());
        // println!("{:?}", Note::D.major_scale_from_tonic());
        // assert_eq!(Sound::init(Note::Cs, 4), Sound::init(Note::Db, 4));
    }
}
