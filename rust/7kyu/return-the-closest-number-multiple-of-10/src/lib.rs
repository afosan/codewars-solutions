//! https://www.codewars.com/kata/58249d08b81f70a2fc0001a4/train/rust

pub fn closest_multiple_of_10(n: u32) -> u32 {
    n / 10 * 10 + if n % 10 < 5 { 0 } else { 10 }
}

#[cfg(test)]
mod tests {
    use super::closest_multiple_of_10;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(n: u32, expected: u32) {
        assert_eq!(closest_multiple_of_10(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn fixed_tests() {
        dotest(54, 50);
        dotest(55, 60);
    }
}
