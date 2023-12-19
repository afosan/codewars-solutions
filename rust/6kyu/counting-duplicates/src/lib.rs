//! https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust

use std::collections::HashMap;

pub fn count_duplicates(text: &str) -> u32 {
    let mut hmap = HashMap::new();

    text.to_lowercase().chars().for_each(|c| *hmap.entry(c).or_insert(0) += 1);

    hmap.values().filter(|&&v| v > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
