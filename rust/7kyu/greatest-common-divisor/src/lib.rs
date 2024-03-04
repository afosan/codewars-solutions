//! https://www.codewars.com/kata/5500d54c2ebe0a8e8a0003fd/train/rust

pub fn gcd (a: u32, b: u32) -> u32 {
    let (mi, mx) = (a.min(b), a.max(b));

    if mi == 0 { mx } else { gcd(mx % mi, mi) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(gcd(30, 12), 6);
        assert_eq!(gcd(8, 9), 1);
        assert_eq!(gcd(1, 1), 1);
    }
}
