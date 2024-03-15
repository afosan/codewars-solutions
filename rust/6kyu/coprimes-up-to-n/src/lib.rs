//! https://www.codewars.com/kata/59e0dbb72a7acc3610000017/train/rust

use std::collections::HashSet;

pub fn coprimes(n: u32) -> Vec<u32> {
    let mut factors = HashSet::<u32>::new();
    let mut nn = n;
    let mut f = 2;

    while nn > 1 {
        if nn % f == 0 {
            factors.insert(f);
            
            while nn % f == 0 {
                nn /= f;
            }
        }

        f += 1;
    }
    
    (1..=n).filter(|i| factors.iter().all(|f| i % f != 0)).collect()
}

#[cfg(test)]
mod tests {
    use super::coprimes;

    fn dotest(n: u32, expected: &[u32]) {
        let actual = coprimes(n);
        assert!(actual == expected,
        "Test failed with n = {n}\nExpected {expected:?} but got {actual:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(2, &[1]);
        dotest(3, &[1, 2]);
        dotest(6, &[1, 5]);
        dotest(10, &[1, 3, 7, 9]);
        dotest(20, &[1, 3, 7, 9, 11, 13, 17, 19]);
        dotest(25, &[1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24]);
        dotest(30, &[1, 7, 11, 13, 17, 19, 23, 29]);
    }
}
