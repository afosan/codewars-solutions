//! https://www.codewars.com/kata/561e9c843a2ef5a40c0000a4/train/rust

use std::collections::HashSet;

fn primes(n: u64) -> HashSet<u64> {
    let mut ps = HashSet::<u64>::new();
    let mut i = 2;

    while i <= n {
        if ps.iter().all(|p| i % p != 0) {
            ps.insert(i);
        }
        i += if i == 2 { 1 } else { 2 };
    }

    ps
}

pub fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut ps = primes((n as f64).sqrt().floor() as u64);
    let mut p0: Option<u64> = None;
    let mut p1: Option<u64> = None; 

    for i in m..=n {
        if ps.iter().filter(|p| *p * *p <= i).all(|p| i % p != 0) {
            ps.insert(i);
            match (p0, p1) {
                (None, _) => {
                    p0 = Some(i);
                },
                (Some(v1), None) => {
                    if i - v1 == g as u64 {
                        return Some((v1, i));
                    }
                    p1 = Some(i);
                },
                (Some(_), Some(v2)) => {
                    if i - v2 == g as u64 {
                        return Some((v2, i));
                    }
                    p0 = p1;
                    p1 = Some(i);
                },
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
        assert_eq!(gap(g, m, n), exp)
    }
    
    #[test]
    fn basics_gap() {
        testing(2,100,110, Some((101, 103)));
        testing(4,100,110, Some((103, 107)));
        testing(6,100,110, None);
        testing(8,300,400, Some((359, 367)));
    }
}
