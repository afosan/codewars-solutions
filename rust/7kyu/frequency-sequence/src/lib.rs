//! https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust

use std::collections::HashMap;

pub fn freq_seq(s: &str, sep: &str) -> String {
    let mut counts = HashMap::<char, usize>::new();
    s.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    s.chars().map(|c| counts.get(&c).unwrap().to_string()).collect::<Vec<_>>().join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
        assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
        assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
    }    
}
