#[cfg(test)]
mod tests {
    use super::super::{Bemol, Sharp};
    #[test]
    fn test_bemol() {
        let bemol = Bemol::init(5);
        println!("{bemol}");
    }
    #[test]
    fn test_sharp() {
        let sharp = Sharp::init(5);
        println!("{sharp}");
    }
}
