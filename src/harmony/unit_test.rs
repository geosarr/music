#[cfg(test)]
mod tests {
    use crate::{KraehenbuehlKnuth, Note, Sound};

    #[test]
    fn test_kraehenbuel_knuth_with_scale() {
        let melody = vec![Sound::init(Note::C, 4), Sound::init(Note::E, 4)];
        let mut harmonizer = KraehenbuehlKnuth::init(melody, Some(Note::C));
        let harmonics = harmonizer.harmonize();
        println!("{:?}", harmonics);
    }
}
