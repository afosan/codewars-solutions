//! https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust

pub fn reverse_words(words: &str) -> String {
    words.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
        assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}
