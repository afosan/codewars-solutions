//! https://www.codewars.com/kata/5b1b27c8f60e99a467000041/train/rust

use std::collections::{HashMap, HashSet};

pub fn anagram_difference(w1: &str, w2: &str) -> u32 {
    let mut c1 = HashMap::<char, usize>::new();
    let mut c2 = HashMap::<char, usize>::new();
    
    w1.chars().for_each(|c| *c1.entry(c).or_insert(0) += 1);
    w2.chars().for_each(|c| *c2.entry(c).or_insert(0) += 1);

    let k = HashSet::<char>::from_iter(w1.chars().chain(w2.chars()));
    
    k.iter().map(|c| {
        let n1 = c1.get(c).unwrap_or(&0);
        let n2 = c2.get(c).unwrap_or(&0);
        
        if n1 >= n2 { n1 - n2 } else { n2 - n1 }
    }).sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::anagram_difference;
        
    fn dotest(w1: &str, w2: &str, expected: u32) {
        let actual = anagram_difference(w1, w2);
        assert!(actual == expected, 
            "With w1 = \"{w1}\", w2 = \"{w2}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        for (w1, w2, expected) in [
                                    ("", "", 0),
                                    ("a", "", 1),
                                    ("", "a", 1),
                                    ("ab", "a", 1),
                                    ("ab", "ba", 0),
                                    ("ab", "cd", 4),
                                    ("codewars", "hackerrank", 10)
                                    ] {
            dotest(w1, w2, expected)
        }
    }
}
