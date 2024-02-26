//! https://www.codewars.com/kata/5b39e3772ae7545f650000fc/train/rust

use std::collections::HashSet;

pub fn remove_duplicate_words(s: &str) -> String {
    let mut words = HashSet::<&str>::new();
    
    s
        .split_whitespace()
        .filter(|&w| words.insert(&w))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(remove_duplicate_words("alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"), "alpha beta gamma delta");
        assert_eq!(remove_duplicate_words("my cat is my cat fat"), "my cat is fat");
    }   
}
