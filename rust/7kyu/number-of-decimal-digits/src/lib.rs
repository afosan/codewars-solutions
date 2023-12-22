//! https://www.codewars.com/kata/58fa273ca6d84c158e000052/train/rust

pub fn digits(n: u64) -> usize {
    if n == 0 {
        return 1;
    }

    let mut n = n;
    let mut i = 0usize;

    while n > 0 {
        i += 1;
        n /= 10;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(digits(5), 1);
        assert_eq!(digits(12345), 5);
        assert_eq!(digits(9876543210), 10);
    }    
}
