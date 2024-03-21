use std::collections::HashMap;
use num::bigint::{BigUint, ToBigUint};

fn fac(s: usize) -> BigUint {
    (1..=s).map(|i| i.to_biguint().unwrap()).product()
}

pub fn uniq_count(s: &str) -> BigUint {
    let mut cs = HashMap::<char, usize>::new();
    
    s.to_lowercase().chars().for_each(|c| *cs.entry(c).or_insert(0) += 1);
    let t = s.len();
    
    fac(t) / (cs.values().copied().map(fac).product::<BigUint>())
}

#[cfg(test)]
mod tests {
    use num::bigint::{BigUint, ToBigUint};
    use num::traits::One;
    use super::uniq_count;
        
    fn dotest(s: &str, expected: BigUint) {
        let actual = uniq_count(s);
        assert!(actual.clone() == expected.clone(), 
            "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("AB", 2.to_biguint().unwrap());
        dotest("ABC", 6.to_biguint().unwrap());
        dotest("AbA", 3.to_biguint().unwrap());
        dotest("ABBb", 4.to_biguint().unwrap());
        dotest("AbcD", 24.to_biguint().unwrap());
        dotest("AAA", BigUint::one());
        dotest("", BigUint::one());
        dotest("ASTON", 120.to_biguint().unwrap());
        dotest("BEST", 24.to_biguint().unwrap());
    }
}
