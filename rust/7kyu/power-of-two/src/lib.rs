//! https://www.codewars.com/kata/534d0a229345375d520006a0/train/rust

pub fn power_of_two(x: u64) -> bool {
    if x == 0 {
        return false;
    }
    if x == 1 {
        return true;
    }

    (x / 2) * 2 == x && power_of_two(x / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        assert_eq!(power_of_two(0), false);
        assert_eq!(power_of_two(2), true);
        assert_eq!(power_of_two(5), false);
        assert_eq!(power_of_two(6), false);
        assert_eq!(power_of_two(8), true);
        assert_eq!(power_of_two(1024), true);
        assert_eq!(power_of_two(4096), true);
        assert_eq!(power_of_two(8191), false);
    }    
}
