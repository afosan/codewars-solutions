//! https://www.codewars.com/kata/55c9172ee4bb15af9000005d/train/rust

pub fn count_odd_pentafib(n: u16) -> u16 {
    if n <= 1 {
        return n;
    }

    (n - 1) / 6 * 2 + ((n - 1) % 6 + 1).min(2) - 1
}

#[cfg(test)]
mod tests {
    use super::count_odd_pentafib;

    #[test]
    fn basic_tests() {
        assert_eq!(count_odd_pentafib(5), 1);
        assert_eq!(count_odd_pentafib(10), 3);
        assert_eq!(count_odd_pentafib(15), 5);
        assert_eq!(count_odd_pentafib(45), 15);
        assert_eq!(count_odd_pentafib(68), 23);
    }
    
    
    #[test]
    fn edge_cases() {
        assert_eq!(count_odd_pentafib(0), 0);
        assert_eq!(count_odd_pentafib(1), 1);
        assert_eq!(count_odd_pentafib(2), 1);
    }
}
