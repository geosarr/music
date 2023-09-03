#[cfg(test)]
mod tests {
    use super::super::{Bemol, Natural, Sharp};
    #[test]
    fn test_bemol() {
        let n = 5;
        let bemol = Bemol::init(n);
        assert_eq!(bemol.to_string().chars().count(), n as usize);
    }
    #[test]
    fn test_sharp() {
        let n = 10;
        let sharp = Sharp::init(n);
        assert_eq!(sharp.to_string().chars().count(), n as usize);
    }
    #[test]
    fn test_natural() {
        let n = 4;
        let sharp = Natural::init(n);
        assert_eq!(sharp.to_string().chars().count(), n as usize);
    }
}
