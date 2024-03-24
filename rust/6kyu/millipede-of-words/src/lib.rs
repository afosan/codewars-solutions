//! https://www.codewars.com/kata/6344701cd748a12b99c0dbc4/train/rust

use std::collections::HashMap;

pub fn millipede(words: &[&str]) -> bool {
    let firsts = words.iter().map(|w| w.chars().nth(0).unwrap()).collect::<Vec<_>>();
    let lasts = words.iter().map(|w| w.chars().last().unwrap()).collect::<Vec<_>>();
    
    let mut first_counts = HashMap::<char, i64>::new();
    firsts.iter().for_each(|&c| *first_counts.entry(c).or_insert(0) += 1);
    
    let mut last_counts = HashMap::<char, i64>::new();
    lasts.iter().for_each(|&c| *last_counts.entry(c).or_insert(0) += 1);
    
    first_counts.iter().for_each(|(&k, v)| *last_counts.entry(k).or_insert(0) -= v);
    
    *last_counts.values().max().unwrap() <= 1 && 
    *last_counts.values().min().unwrap() >= -1 && 
    last_counts.values().sum::<i64>() == 0
}

#[cfg(test)]
mod tests {
    use super::millipede;

    #[test]
    fn example_true() {
        assert_eq!(millipede(&["excavate", "endure", "screen", "desire", "theater", "excess", "night"]), true);
    }
    #[test]
    fn example_false() {
        assert_eq!(millipede(&["trade", "pole", "view", "grave", "ladder", "mushroom", "president"]), false);
    }
    #[test]
    fn tests_are_not_broken() {
        assert_eq!(millipede(&["excavate", "east", "strike", "transport"]), true);
    }
    #[test]
    fn ah2023_false() {
        assert_eq!(millipede(&["stereotype", "evaluate", "stereotype", "empire"]), false);
    }
    #[test]
    fn five_words_true() {
        assert_eq!(millipede(&["screen", "desire", "theater", "excess", "night"]), true);
    }
    #[test]
    fn four_words_false() {
        assert_eq!(millipede(&["trade", "pole", "view", "grave"]), false);
    }
}
