//! https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust

pub fn increment_string(s: &str) -> String {
    if let Some(last_char) = s.chars().last() {
        if let Some(last_digit) = last_char.to_digit(10) {
            let s_head = &s[..s.len() - 1];
            if last_digit < 9 {
                format!("{s_head}{}", last_digit + 1)
            } else {
                format!("{}0", increment_string(s_head))
            }
        } else {
            format!("{s}1")
        }
    } else {
        "1".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }
    
    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
