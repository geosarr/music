#[cfg(test)]
mod tests {
    use super::super::{Flat, Natural, Sharp};
    #[test]
    fn test_flat() {
        let n = 5;
        let flat = Flat::init(n);
        assert_eq!(flat.display().chars().count(), n as usize);
    }
    #[test]
    fn test_sharp() {
        let n = 10;
        let sharp = Sharp::init(n);
        assert_eq!(sharp.display().chars().count(), n as usize);
    }
    #[test]
    fn test_natural() {
        let n = 4;
        let sharp = Natural::init(n);
        assert_eq!(sharp.display().chars().count(), n as usize);
    }
}
