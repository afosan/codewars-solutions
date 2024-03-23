//! https://www.codewars.com/kata/5626ec066d35051d4500009e/train/rust

fn prime_factors(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut i = 2;
    let mut pfs = vec![];
    
    while n > 1 {
        while n % i == 0 {
            n /= i;
            pfs.push(i);
        }
        i += 1;
    }
    
    pfs
}

fn is_included(n: u32) -> bool {
    let pfs = prime_factors(n);
    
    pfs.len() > 1 && n % pfs.iter().sum::<u32>() == 0
}

pub fn mult_primefactor_sum(a: u32, b: u32) -> Vec<u32> {
    (a..=b).filter(|i| is_included(*i)).collect()
}

#[cfg(test)]
mod tests {
    use super::mult_primefactor_sum;
        
    fn dotest(a: u32, b: u32, expected: &[u32]) {
        let actual = mult_primefactor_sum(a, b);
        assert!(actual == expected, 
            "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(10, 100, &[16, 27, 30, 60, 70, 72, 84]);
        dotest(80, 150, &[84, 105, 150]);
    }
}
