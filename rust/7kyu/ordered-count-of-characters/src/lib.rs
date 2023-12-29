//! https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust

use std::collections::{HashMap, HashSet};

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut char_counts = HashMap::<char, i32>::new();

    sip.chars().for_each(|c| *char_counts.entry(c).or_insert(0) += 1);

    let mut seen = HashSet::<char>::new();
    sip.chars().filter(|c| match seen.get(&c) {
        Some(_) => false,
        None => {
            seen.insert(*c);
            true
        },
    }).map(
        |c| (c, *char_counts.get(&c).unwrap())
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
