//! https://www.codewars.com/kata/59b139d69c56e8939700009d/train/rust

pub fn get_exponent(n: i32, p: u32) -> Option<u32> {
    if p <= 1 {
        None
    } else {
        let mut c = 0;
        let mut n = n.abs() as u32;
        
        while n % p == 0 {
            c += 1;
            n /= p;
        }
        
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::get_exponent;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(n: i32, p: u32, expected: Option<u32>) {
        assert_eq!(get_exponent(n, p), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn fixed_tests() {
        dotest(27, 3, Some(3));
        dotest(28, 3, Some(0));
        dotest(280, 7, Some(1));
        dotest(-250, 5, Some(3));
        dotest(18, 1, None);
        dotest(128, 4, Some(3));
        dotest(0, 0, None);
    }
}
