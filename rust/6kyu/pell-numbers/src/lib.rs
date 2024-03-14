//! https://www.codewars.com/kata/5818d00a559ff57a2f000082/train/rust

pub fn pell(n: u32) -> u128 {
    let mut p0 = 0;
    let mut p1 = 1;
    let mut c = 0;
    
    while c < n {
        c += 1;
        (p0, p1) = (p1, 2 * p1 + p0);
    }
    
    p0
}

#[cfg(test)]
mod tests {
    use super::pell;
        
    fn dotest(n: u32, expected: u128) {
        let actual = pell(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, 1);
        dotest(2, 2);
        dotest(3, 5);
        dotest(4, 12);
    }
}
