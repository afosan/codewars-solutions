//! https://www.codewars.com/kata/52dbae61ca039685460001ae/train/rust

use std::collections::HashSet;

pub fn change(string: &str) -> String {
    let exists = string.to_lowercase().chars().collect::<HashSet<char>>();

    ('a'..='z').map(|c| if exists.contains(&c) { "1" } else { "0" }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(change("a **&  bZ"), "11000000000000000000000001");
    }
}
