//! https://www.codewars.com/kata/53d40c1e2f13e331fc000c26/train/rust

use num::bigint::BigInt;

pub fn fib(n: i32) -> BigInt {
    if n >= 0 {
        fib_helper(n).0
    } else {
        if n % 2 == 0 {
            -fib_helper(-n).0
        } else {
            fib_helper(-n).0
        }
    }
}

fn fib_helper(n: i32) -> (BigInt, BigInt) {
    if n == 0 {
        (BigInt::from(0), BigInt::from(1))
    }
    else if n == 1 {
        (BigInt::from(1), BigInt::from(1))
    } else {
        let (v0, v1) = fib_helper(n / 2);
        let p: BigInt = v0.clone() * (2 * v1.clone() - v0.clone());
        let q: BigInt = v1.clone() * v1 + v0.clone() * v0;
        
        if n % 2 == 0 {
            (p.clone(), q.clone())
        } else {
            (q.clone(), p + q)
        }
    }
}

#[cfg(test)]
mod sample_tests {
    use super::fib;
    use num::bigint::BigInt;
    use num::traits::{Zero, One};
    use std::str::FromStr;
    
    fn dotest(n: i32, expected: BigInt) {
        let actual = fib(n);
        assert!(actual == expected,
            "Test failed with n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\"")
    }
    
    #[test]
    fn small_positive_numbers() {
        dotest(0, BigInt::zero());
        dotest(1, BigInt::one());
        dotest(2, BigInt::one());
        dotest(3, BigInt::from(2));
        dotest(4, BigInt::from(3));
        dotest(5, BigInt::from(5));
    }
    
    #[test]
    fn small_negative_numbers() {
        dotest(-1, BigInt::from(1));
        dotest(-6, BigInt::from(-8));
        dotest(-96, BigInt::from_str("-51680708854858323072").unwrap());
        
    }
    
    #[test]
    fn large_numbers() {
        dotest(
            -500,
            BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
            .unwrap()
        );
        
        dotest(
            1000,
            BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
            .unwrap()
        );
    }
}
