#[cfg(test)]
mod tests {
    use super::super::{Bemol, Note, Sharp};
    #[test]
    fn test_Note_eq_duplicates() {
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
    fn test_Note_algebra() {
        let note = Note::A;
        let accidental = Sharp::init(4);
        assert_eq!(&note + &accidental, Note::Cs);
        assert_eq!(&note + &accidental, Note::Db);
        assert_eq!(note + accidental, Note::Cs);
    }
}
