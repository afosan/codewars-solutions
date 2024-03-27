//! https://www.codewars.com/kata/542f3d5fd002f86efc00081a/train/rust

// TODO: timeouts

pub fn prime_factors(n: u32) -> Vec<u32> {
    let mut out = vec![];
    let mut n = n;
    let mut p = 2;
    
    while n > 1 {
        while n % p == 0 {
            out.push(p);
            n /= p;
        }
        
        p += 1;
    }
    
    out
}

#[cfg(test)]
mod tests {
    use super::prime_factors;
        
    fn dotest(n: u32, expected: &[u32]) {
        let actual = prime_factors(n);
        assert!(actual == expected, "With n = {n}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, &[]);
        dotest(4, &[2,2]);
        dotest(8, &[2,2,2]);
        dotest(9, &[3,3]);
        dotest(12, &[2,2,3]);
    }
}
