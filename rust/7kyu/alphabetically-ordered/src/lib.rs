//! https://www.codewars.com/kata/5a8059b1fd577709860000f6/train/rust

pub fn alphabetic(s: &str) -> bool {
    s.chars().collect::<Vec<_>>().windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod tests {
    use super::alphabetic;
        
    fn dotest(s: &str, expected: bool) {
        let actual = alphabetic(s);
        assert!(actual == expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("asd", false);
        dotest("codewars", false);
        dotest("door", true);
        dotest("cell", true);
        dotest("z", true);
        dotest("", true);
    }
}
