//! https://www.codewars.com/kata/5873b2010565844b9100026d/train/rust

pub fn one_two_three(n: u32) -> [String;2] {
    [
        format!("{}{}", "9".repeat(n as usize / 9), if n == 0 { "0".to_string() } else if n % 9 == 0 { "".to_string() } else { (n % 9).to_string() })
        , if n == 0 { "0".to_string() } else { "1".repeat(n as usize) }
    ]
}

#[cfg(test)]
mod tests {
    use super::one_two_three;
        
    fn dotest(n: u32, expected: [String;2]) {
        let actual = one_two_three(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, ["0".to_string(), "0".to_string()]);
        dotest(1, ["1".to_string(), "1".to_string()]);
        dotest(2, ["2".to_string(), "11".to_string()]);
        dotest(3, ["3".to_string(), "111".to_string()]);
        dotest(19, ["991".to_string(), "1111111111111111111".to_string()]);
    }
}
