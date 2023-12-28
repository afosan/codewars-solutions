//! https://www.codewars.com/kata/542c0f198e077084c0000c2e/train/rust

use std::collections::HashMap;

fn pf(n: u32) -> HashMap<u32, u32> {
    let mut out = HashMap::<u32, u32>::new();
    let mut n = n;
    let mut p = 2;
    let mut i = 0;

    while n > 1 {
        if n % p == 0 {
            i += 1;
            n /= p;
        } else {
            out.insert(p, i);
            i = 0;
            p += 1;
        }
    }
    if i > 0 {
        out.insert(p, i);
    }

    out
}

pub fn divisors(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        pf(n).values().map(|i| *i + 1).product()
    }
}

#[cfg(test)]
mod tests {
    use super::divisors;

    #[test]
    fn sample_tests() {
        assert_eq!(divisors(1), 1);
        assert_eq!(divisors(4), 3);
        assert_eq!(divisors(5), 2);
        assert_eq!(divisors(12), 6);
        assert_eq!(divisors(25), 3);
        assert_eq!(divisors(4096), 13);
    }
}
