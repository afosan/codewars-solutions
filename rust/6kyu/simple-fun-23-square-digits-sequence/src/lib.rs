//! https://www.codewars.com/kata/5886d65e427c27afeb0000c1/train/rust

use std::collections::HashSet;

fn digits(n: u32) -> Vec<u32> {
    if n == 0 { return vec![0]; }
    
    let mut n = n;
    let mut v = vec![];
    
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    
    v
}

fn sum_of_squared_digits(n: u32) -> u32 {
    let ds = digits(n);
    
    ds.iter().map(|d| d * d).sum()
}

pub fn square_digits_sequence(a0: u32) -> usize {
    let mut seen = HashSet::<u32>::new();
    let mut n = a0;
    seen.insert(n);

    loop {
        let r = sum_of_squared_digits(n);
        
        if !seen.insert(r) {
            return seen.len() + 1;
        }
        
        n = r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(square_digits_sequence(16), 9);
        assert_eq!(square_digits_sequence(103), 4);
        assert_eq!(square_digits_sequence(1), 2);
    }
}
