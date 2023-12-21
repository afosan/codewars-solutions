//! https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/rust

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn is_pangram(s: &str) -> bool {
    let mut hs: HashSet<char> = HashSet::from_iter('a'..='z');
    s.to_lowercase().chars().for_each(|c| {
        hs.remove(&c);
    });

    hs.len() == 0
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert!(actual == expected, "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
