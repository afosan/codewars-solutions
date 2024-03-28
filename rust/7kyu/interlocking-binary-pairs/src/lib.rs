//! https://www.codewars.com/kata/628e3ee2e1daf90030239e8a/train/rust

pub fn interlockable(a: u64, b: u64) -> bool {
    a & b == 0
}

#[cfg(test)]
mod tests {
    use super::interlockable;
        
    fn dotest(a: u64, b: u64, expected: bool) {
        let actual = interlockable(a, b);
        assert!(actual == expected, "With\na = {a}\nb = {b}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        for (a, b, expected) in [(9, 4, true),
                                 (3, 6, false),
                                 (2, 5, true),
                                 (7, 1, false),
                                 (0, 8, true)]
        {
            dotest(a, b, expected);
        }
    }
}
