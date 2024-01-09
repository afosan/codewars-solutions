//! https://www.codewars.com/kata/5a946d9fba1bb5135100007c/train/rust

// use std::collections::HashSet;

pub fn minimum_number(xs: &[u32]) -> u32 {
    // let sum = xs.iter().sum::<u32>();
    // let mut primes = HashSet::new();
    // let mut x = 2;
    // loop {
    //     if primes.iter().all(|p| x % p != 0) {
    //         primes.insert(x);
    //         if x >= sum {
    //             break;
    //         }
    //     }
    //     x += 1;
    // }

    // x - sum

    let sum = xs.iter().sum::<u32>();
    (sum..).skip_while(|n| (2..).take_while(|i| i * i <= *n).any(|i| n % i == 0)).nth(0).unwrap() - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(minimum_number(&[3,1,2]), 1);
        assert_eq!(minimum_number(&[5,2]), 0);
        assert_eq!(minimum_number(&[1,1,1]), 0);
        assert_eq!(minimum_number(&[2,12,8,4,6]), 5);
        assert_eq!(minimum_number(&[50,39,49,6,17,28]), 2);
    }   
}
