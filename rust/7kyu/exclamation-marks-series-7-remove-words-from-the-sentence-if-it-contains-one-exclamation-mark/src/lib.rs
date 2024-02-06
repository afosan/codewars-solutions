//! https://www.codewars.com/kata/57fafb6d2b5314c839000195/train/rust

pub fn remove(s: &str) -> String {
    s.split_whitespace().filter(|w| w.chars().filter(|&c| c == '!').count() != 1).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::remove;

    #[test]
    fn fixed_tests() {
        assert_eq!(remove("Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi! Hi!"), String::from(""));
        assert_eq!(remove("Hi Hi! Hi!"), String::from("Hi"));
        assert_eq!(remove("Hi! !Hi Hi!"), String::from(""));
        assert_eq!(remove("Hi! Hi!! Hi!"), String::from("Hi!!"));
        assert_eq!(remove("Hi! !Hi! Hi!"), String::from("!Hi!"));
        assert_eq!(remove("Hi! !Hi! Hi! Hi"), String::from("!Hi! Hi"));
    }
}
