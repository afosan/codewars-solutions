//! https://www.codewars.com/kata/5a99a03e4a6b34bb3c000124/train/rust

use std::collections::HashSet;

pub fn num_primorial(n: usize) -> u64 {
    if n == 0 {
        panic!("expected n to be positive");
    }

    let mut primes = HashSet::<u64>::new();

    (2..).filter(|i| {
        if primes.iter().all(|p| i % p != 0) {
            primes.insert(*i);
            true
        } else {
            false
        }
    }).take(n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(num_primorial(3), 30);
        assert_eq!(num_primorial(4), 210);
        assert_eq!(num_primorial(5), 2310);
        assert_eq!(num_primorial(8), 9699690);
        assert_eq!(num_primorial(9), 223092870);
    }
}
