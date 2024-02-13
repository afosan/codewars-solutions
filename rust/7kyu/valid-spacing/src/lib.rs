//! https://www.codewars.com/kata/5f77d62851f6bc0033616bd8/train/rust

pub fn valid_spacing(s: &str) -> bool {
    s.split(" ").filter(|ss| ss.len() == 0).count() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(valid_spacing("Hello world"), true, "Testing 'Hello world'");
        assert_eq!(valid_spacing(" Hello world"), false, "Testing ' Hello world'");
        assert_eq!(valid_spacing("Hello  world "), false, "Testing 'Hello  world '");
        assert_eq!(valid_spacing("Hello"), true, "Testing 'Hello'");
        assert_eq!(valid_spacing("Helloworld"), true, "Testing 'Helloworld'");
    }
}
