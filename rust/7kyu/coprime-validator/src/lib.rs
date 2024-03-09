//! https://www.codewars.com/kata/585c50e75d0930e6a7000336/train/rust

fn gcd(n: u8, m: u8) -> u8 {
    let (mi, mx) = (m.min(n), m.max(n));
    
    if mi == 0 { mx } else { gcd(mx % mi, mi) }
}

pub fn are_coprime(n: u8, m: u8) -> bool {
    gcd(n, m) == 1
}

#[cfg(test)]
mod tests {
    use super::are_coprime;
    
    fn dotest(n: u8, m: u8, expected: bool) {
        let actual = are_coprime(n, m);
        assert!(actual == expected, "Test failed with n = {n} and m = {m}\nExpected {expected} but got {actual}")
    } 
    
    #[test]
    fn sample_tests() {
        dotest(20, 27, true);
        dotest(12, 39, false);
    }
}
