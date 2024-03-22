//! https://www.codewars.com/kata/5676ffaa8da527f234000025/train/rust

use std::collections::HashSet;
use itertools::Itertools;

fn digits(num: u32) -> Vec<u8> {
    if num == 0 { return vec![0]; }

    let mut v = vec![];
    let mut n = num;

    while n > 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }
    
    v
}

pub fn sc_perm_comb(num: u32) -> u64 {
    let ds = digits(num);
    let perms: HashSet<u64> = (1..=ds.len()).flat_map(|i| ds.iter().permutations(i).map(|v| {
        v.iter().map(|n| {
            n.to_string()
        }).collect::<String>().parse::<u64>().unwrap()
    })).collect();
    
    perms.iter().sum()
}

#[cfg(test)]
mod tests {
    // use rand::{thread_rng, Rng};
    use super::sc_perm_comb;
        
    fn dotest(n: u32, expected: u64) {
        let actual = sc_perm_comb(n);
        assert!(actual == expected, 
            "With num = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(348, 3675);
        dotest(340, 1631);
        dotest(333, 369);
        dotest(6, 6);
        dotest(0, 0);
    }
}
