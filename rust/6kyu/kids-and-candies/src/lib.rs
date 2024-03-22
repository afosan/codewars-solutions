//! https://www.codewars.com/kata/56cca888a9d0f25985000036/train/rust

use num::bigint::{BigUint, ToBigUint};

fn primes(n: u16) -> Vec<u16> {
    let mut ps = vec![];
    
    (2..=n).for_each(|i| {
        if ps.iter().all(|p| i % p != 0) {
            ps.push(i);
        }
    });
    
    ps
}

pub fn candies_to_buy(amount_of_kids_invited: u16) -> BigUint {
    let ps = primes(amount_of_kids_invited);
    let n = amount_of_kids_invited as f64;
    
    ps.iter().map(|p| {
        let i = n.log(*p as f64) as u32;
        p.pow(i).to_biguint().unwrap()
    }).product()
}

#[cfg(test)]
mod tests {
    use super::candies_to_buy;
    use num::bigint::BigUint;
    use num::bigint::ToBigUint;
    use num::traits::One;


    #[test]
    fn sample_tests() {
        assert_eq!(candies_to_buy(1), BigUint::one());
        assert_eq!(candies_to_buy(2), 2.to_biguint().unwrap());
        assert_eq!(candies_to_buy(5), 60.to_biguint().unwrap());
        assert_eq!(candies_to_buy(6), 60.to_biguint().unwrap());
        assert_eq!(candies_to_buy(9), 2520.to_biguint().unwrap());
        assert_eq!(candies_to_buy(10), 2520.to_biguint().unwrap());
    }
}
