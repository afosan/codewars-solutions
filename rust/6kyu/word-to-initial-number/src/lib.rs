//! https://www.codewars.com/kata/5bb148b840196d1be50000b1/train/rust

use std::collections::HashMap;

pub fn convert(word: &str) -> u64 {
    let word = word.to_lowercase();
    let mut mapping = HashMap::<char, u64>::new();
    let mut next_digit = 1;
    
    word.chars().for_each(|c| {
        if mapping.get(&c).is_none() {
            mapping.insert(c, next_digit);
            
            next_digit = if next_digit == 1 { 0 } else if next_digit == 0 { 2 } else { next_digit + 1 };
        }
    });

    word.chars().fold(0_u64, |acc, c| {
        let v = mapping.get(&c).unwrap();
        10 * acc + v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let expected = 10234567;
        let s = "CodeWars";

        assert_eq!(expected, convert(s));
    }

    #[test]
    fn test1() {
        let expected = 1020;
        let s = "KATA";

        assert_eq!(expected, convert(s));
    }
}
