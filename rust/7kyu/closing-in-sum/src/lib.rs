//! https://www.codewars.com/kata/65126d52a5de2b11c94096d2/train/rust

pub fn closing_in_sum(n: u64) -> u32 {
    let mut n = n;
    let mut l = n.to_string().len() as u32;
    let mut summ = 0;
    
    while l >= 1 {
        if l == 1 {
            summ += n;
            break;
        } else {
            let p = 10_u64.pow(l - 1);
            let first = n / p;
            let last = n % 10;
            let num = 10 * first + last;
            summ += num;
            l -= 2;
            n = (n % p) / 10; 
        }
    }
    
    summ as u32
}

#[cfg(test)]
mod tests {
    use super::closing_in_sum;
        
    fn dotest(n: u64, expected: u32) {
        let actual = closing_in_sum(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(121, 13);
        dotest(1039, 22);
        dotest(22225555, 100);
        dotest(2520, 72);
        dotest(5332824166496569, 331);
        dotest(1979672314137318116, 485);
        dotest(1795459644013947776, 548);
        dotest(2801980378842185820, 426);
        dotest(3440584288422776554, 430);
        dotest(1985124000275401986, 342);
    }
}
