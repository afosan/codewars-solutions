//! https://www.codewars.com/kata/54d512e62a5e54c96200019e/train/rust

use std::collections::HashMap;

pub fn prime_factors(n: i64) -> String {
    let mut n = n as u64;
    let mut pfs = HashMap::<u64, u64>::new();
    let mut i = 2;

    while n > 1 {
        while n % i == 0 {
            *pfs.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += if i == 2 { 1 } else { 2 };
    }

    let mut pis = pfs.into_iter().collect::<Vec<(u64, u64)>>();
    pis.sort_by_key(|pi| pi.0);

    pis.iter().map(|(p, i)| if *i == 1 { format!("({})", p) } else { format!("({}**{})", p, i) }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }
    
    #[test]
    fn basics_prime_factors() {
        
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17*17*93*677, "(3)(17**2)(31)(677)");
        
    }
}
