//! https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4/train/rust

use std::collections::HashMap;

pub fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    let mut cnts = HashMap::<String, usize>::new();
    
    a1.iter().for_each(|w| *cnts.entry(w.to_string()).or_insert(0) += 1);
    
    a2.iter().map(|w| *cnts.get(w).unwrap_or_else(|| &0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abcd".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "uap".to_string()]
            ),
            vec![2, 1, 0]
        );
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "xyz".to_string()]
            ),
            vec![2, 1, 2]
        );
        assert_eq!(
            match_counts(
                &[
                    "quick".to_string(),
                    "brown".to_string(),
                    "fox".to_string(),
                    "is".to_string(),
                    "quick".to_string()
                ],
                &["quick".to_string(), "abc".to_string(), "fox".to_string()]
            ),
            vec![2, 0, 1]
        );
    }
}
