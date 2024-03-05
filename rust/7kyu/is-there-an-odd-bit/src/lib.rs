//! https://www.codewars.com/kata/5d6f49d85e45290016bf4718/train/rust

pub fn any_odd(x: u32) -> u8 {
    if format!("{x:b}").chars().rev().enumerate().any(|(i, b)| i % 2 == 1 && b == '1') {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(any_odd(2863311530), 1);
        assert_eq!(any_odd(128), 1);
        assert_eq!(any_odd(131), 1);
        assert_eq!(any_odd(2), 1);
        assert_eq!(any_odd(24082), 1);
        assert_eq!(any_odd(0), 0);
        assert_eq!(any_odd(85), 0);
        assert_eq!(any_odd(1024), 0);
        assert_eq!(any_odd(1), 0);
        assert_eq!(any_odd(1365), 0);
    }
}
