//! https://www.codewars.com/kata/5559e4e4bbb3925164000125/train/rust

use num::bigint::{BigUint, ToBigUint};
use num::traits::{One, Zero};

pub fn fib_rabbits(n: u8, b: u8) -> BigUint {
    let mut acc = (BigUint::one(), BigUint::zero());
    let b = b.to_biguint().unwrap();

    for _ in 0..n {
        acc = (acc.1.clone() * b.clone(), acc.0 + acc.1);
    }

    acc.1
}

#[cfg(test)]
mod tests {
    use super::fib_rabbits;
    use num::bigint::BigUint;
    use num::bigint::ToBigUint;
    use num::traits::One;
    use num::traits::Zero;

    #[test]
    fn sample_tests() {
        assert_eq!(fib_rabbits(0, 4), BigUint::zero());
        assert_eq!(fib_rabbits(1, 4), BigUint::one());
        assert_eq!(fib_rabbits(4, 0), BigUint::one());
        assert_eq!(fib_rabbits(6, 3), 40.to_biguint().unwrap());
        assert_eq!(fib_rabbits(8, 12), 8425.to_biguint().unwrap());
        assert_eq!(fib_rabbits(7, 4), 181.to_biguint().unwrap());
    }
}
