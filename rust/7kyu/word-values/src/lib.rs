//! https://www.codewars.com/kata/598d91785d4ce3ec4f000018/train/rust

pub fn word_value(words: &[&str]) -> Vec<i32> {
    words.iter().enumerate().map(|(i, word)| (i as i32 + 1) * word.chars().filter(|c| *c >= 'a' && *c <= 'z').map(|c| c as i32 - 'a' as i32 + 1).sum::<i32>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(word_value(&["abc", "abc", "abc", "abc"]), [6, 12, 18, 24]);
        assert_eq!(word_value(&["codewars", "abc", "xyz"]), [88, 12, 225]);
    }
}
