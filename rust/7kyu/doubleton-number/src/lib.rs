//! https://www.codewars.com/kata/604287495a72ae00131685c7/train/rust

use std::collections::HashSet;

fn is_doubleton(n: u32) -> bool {
    if n == 0 { return false; }
    
    let mut n = n;
    let mut seen = HashSet::new();
    
    while n > 0 {
        seen.insert(n % 10);
        n /= 10;
    }
    
    seen.len() == 2
}

pub fn doubleton(num: u32) -> u32 {
    (num + 1..).skip_while(|i| !is_doubleton(*i)).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::doubleton;

    #[test]
    fn sample_tests() {
        assert_eq!(doubleton(1), 10);
        assert_eq!(doubleton(10), 12);
        assert_eq!(doubleton(120), 121);
        assert_eq!(doubleton(1234), 1311);
    }
   
}
