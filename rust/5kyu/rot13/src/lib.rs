//! https://www.codewars.com/kata/530e15517bc88ac656000716/train/rust

pub fn rot13(message: &str) -> String {
    message.chars().map(|c| match c {
        'a'..='z' => char::from_u32(((c as u32 - 'a' as u32) + 13) % 26 + 'a' as u32).expect("unexpected error"),
        'A'..='Z' => char::from_u32(((c as u32 - 'A' as u32) + 13) % 26 + 'A' as u32).expect("unexpected error"),
        _ => c,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::rot13;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
    }
}
