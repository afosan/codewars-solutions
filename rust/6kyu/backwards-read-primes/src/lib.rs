//! https://www.codewars.com/kata/5539fecef69c483c5a000015/train/rust

use std::collections::HashSet;

pub fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut ps = HashSet::<u64>::new();

    (2..=10 * stop).for_each(|i| {
        if ps.iter().all(|p| i % p != 0) {
            ps.insert(i);
        }
    });
    
    let mut out = ps.iter().filter(|&p| {
        let r = p.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        *p >= start && *p <= stop && *p != r && ps.contains(&r)
    }).copied().collect::<Vec<_>>();
    
    out.sort_unstable();
    
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(start: u64, stop: u64, exp: Vec<u64>) -> () {
        assert_eq!(backwards_prime(start, stop), exp)
    }
    
    #[test]
    fn tests_backwards_prime() {
    
        let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
        testing(1, 100, a);
        let a = vec![13, 17, 31];
        testing(1, 31, a);
    }    
}
