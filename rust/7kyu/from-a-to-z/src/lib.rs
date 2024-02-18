//! https://www.codewars.com/kata/6512b3775bf8500baea77663/train/rust

pub fn gimme_the_letters(sp: &str) -> String {
    let mut it = sp.chars();
    let first = it.nth(0).unwrap();
    let second = it.nth(1).unwrap();
    
    (first..=second).collect()
}

#[cfg(test)]
mod tests {
    use super::gimme_the_letters;
        
    fn dotest(sp: &str, expected: &str) {
        let actual = gimme_the_letters(sp);
        assert!(actual == expected, 
            "With sp = \"{sp}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("a-z", "abcdefghijklmnopqrstuvwxyz");
        dotest("h-o", "hijklmno");
        dotest("Q-Z", "QRSTUVWXYZ");
        dotest("J-J", "J");
        dotest("a-b", "ab");
        dotest("A-A", "A");
        dotest("g-i", "ghi");
        dotest("H-I", "HI");
        dotest("y-z", "yz");
        dotest("e-k", "efghijk");
        dotest("a-q", "abcdefghijklmnopq");
        dotest("F-O", "FGHIJKLMNO");
    }
}
