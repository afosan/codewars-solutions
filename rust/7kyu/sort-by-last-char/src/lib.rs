//! https://www.codewars.com/kata/57eba158e8ca2c8aba0002a0/train/rust

pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut words = s.split_whitespace().map(|word| word.to_string()).collect::<Vec<String>>();
    words.sort_by_key(|word| word.chars().last().expect("expected non-empty word"));

    words
}

#[cfg(test)]
mod tests {
    use super::sort_by_last_char;

    #[test]
    fn sample_tests() {
        assert_eq!(sort_by_last_char("man i need a taxi up to ubud"), 
            vec!["a", "need", "ubud", "i", "taxi", "man", "to", "up"]);
        assert_eq!(sort_by_last_char("what time are we climbing up the volcano"),
            vec!["time", "are", "we", "the", "climbing", "volcano", "up", "what"]);
        assert_eq!(sort_by_last_char("take me to semynak"),
            vec!["take", "me", "semynak", "to"]);
        assert_eq!(sort_by_last_char("massage yes massage yes massage"),
            vec!["massage", "massage", "massage", "yes", "yes"]);
        assert_eq!(sort_by_last_char("take bintang and a dance please"),
            vec!["a", "and", "take", "dance", "please", "bintang"]);
    }
}
