//! https://www.codewars.com/kata/57e76bc428d6fbc2d500036d/train/rust

pub fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| w.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::string_to_array;

    fn dotest(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert!(actual == expected, "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("Robin Singh", &["Robin", "Singh"]);
        dotest("CodeWars", &["CodeWars"]);
        dotest("I love arrays they are my favorite", &["I", "love", "arrays", "they", "are", "my", "favorite"]);
        dotest("1 2 3", &["1", "2", "3"]);
    }
}
