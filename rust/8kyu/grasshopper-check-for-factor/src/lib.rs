//! https://www.codewars.com/kata/55cbc3586671f6aa070000fb/train/rust

pub fn check_for_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(check_for_factor(10, 2), true);
        assert_eq!(check_for_factor(63, 7), true);
        assert_eq!(check_for_factor(2450, 5), true);
        assert_eq!(check_for_factor(24612, 3), true);
        assert_eq!(check_for_factor(9, 2), false);
        assert_eq!(check_for_factor(653, 7), false);
        assert_eq!(check_for_factor(2453, 5), false);
        assert_eq!(check_for_factor(24617, 3), false);
    }
}
