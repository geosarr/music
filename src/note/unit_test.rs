#[cfg(test)]
mod tests {
    use super::super::Note;
    use crate::accidental::{Bemol, Natural, Sharp};

    #[test]
    fn test_note_eq_duplicates() {
        assert_eq!(Note::As, Note::Bb);
        assert_eq!(Note::B, Note::Cb);
        assert_eq!(Note::Bs, Note::C);
        assert_eq!(Note::Cs, Note::Db);
        assert_eq!(Note::Ds, Note::Eb);
        assert_eq!(Note::E, Note::Fb);
        assert_eq!(Note::Es, Note::F);
        assert_eq!(Note::Fs, Note::Gb);
        assert_eq!(Note::Gs, Note::Ab);
    }
    #[test]
    fn test_note_algebra() {
        let note = Note::E;
        let accidental = Bemol::init(1);
        assert_eq!(&accidental + &note, Note::Eb);
        assert_eq!(&note + &accidental, Note::Ds);

        let note = Note::A;
        let accidental = Sharp::init(4);
        assert_eq!(&note + &accidental, Note::Cs);
        assert_eq!(&accidental + &note, Note::Db);
        assert_eq!(note + accidental, Note::Cs);

        let note = Note::C;
        let accidental = Bemol::init(11);
        assert_eq!(&note + &accidental, Note::Db);
        assert_eq!(note + accidental, Note::Cs);

        let note = Note::F;
        let accidental = Natural::init(2);
        assert_eq!(&accidental + &note, &note);
        assert_eq!(note + accidental, Note::F);

        let note = Note::Es;
        let accidental = Natural::init(2);
        assert_eq!(&note + &accidental, &Note::E);
        assert_eq!(accidental + note, Note::E);
    }
}
