//! https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust

use std::collections::HashMap;

fn char_counts(word: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    word.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

    map
} 

pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let counts = char_counts(&word);

    word.chars().map(|c| match counts.get(&c).expect("no way of getting none") {
        1 => "(",
        _ => ")",
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"),"(((");
        assert_eq!(duplicate_encode("recede"),"()()()");
        assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
        assert_eq!(duplicate_encode("(( @"),"))((");
    }
}
