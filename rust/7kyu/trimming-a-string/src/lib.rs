//! https://www.codewars.com/kata/563fb342f47611dae800003c/train/rust

pub fn trim(phrase: &str, length: usize) -> String {
    let l = phrase.len();

    if l <= length {
        phrase.to_string()
    } else if length <= 3 {
        format!("{}...", &phrase[..length])
    } else {
        format!("{}...", &phrase[..length-3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn dotest(phrase: &str, length: usize, expected: &str) {
        let actual = trim(phrase, length);
        assert_eq!(actual, expected, "\n\nIncorrect answer for:\n  phrase = \"{phrase}\"\n  length = {length}\nExpected: \"{expected}\"\n  Actual: \"{actual}\"")
    }

    #[test]
    fn example_tests() {
        dotest("Creating kata is fun", 14, "Creating ka...");
        dotest("He", 1, "H...");
        dotest("Hey", 2, "He...");
        dotest("Hey", 3, "Hey");
        dotest("Creating kata is fun", 2, "Cr...");
        dotest("Code Wars is pretty rad", 3, "Cod...");
        dotest("Coding rocks", 12, "Coding rocks");
        dotest("Code Wars is pretty rad", 50, "Code Wars is pretty rad");
        dotest("London is freezing", 18, "London is freezing");
    }
}
