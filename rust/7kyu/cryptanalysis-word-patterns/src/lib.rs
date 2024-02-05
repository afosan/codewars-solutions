use std::collections::HashMap;

pub fn word_pattern(word: &str) -> String {
    let word = word.to_lowercase();
    let mut hm = HashMap::<char, usize>::new();
    word.chars().for_each(|c| {
        if !hm.contains_key(&c) {
            hm.insert(c, hm.len());
        }
    });

    word.chars().map(|c| hm.get(&c).unwrap().to_string()).collect::<Vec<_>>().join(".")
}


#[cfg(test)]
mod tests {
    use super::word_pattern;

    fn dotest(s: &str, expected: &str) {
        let actual = word_pattern(s);
        assert!(actual == expected, "With word = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("hello", "0.1.2.2.3");
        dotest("heLlo", "0.1.2.2.3");
        dotest("helLo", "0.1.2.2.3");
        dotest("Hippopotomonstrosesquippedaliophobia", "0.1.2.2.3.2.3.4.3.5.3.6.7.4.8.3.7.9.7.10.11.1.2.2.9.12.13.14.1.3.2.0.3.15.1.13");
    }
}
