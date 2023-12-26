//! https://www.codewars.com/kata/577b9960df78c19bca00007e/train/rust

pub fn find_digit(num: i32, nth: i32) -> i32 {
    if num < 0 {
        return find_digit(-num, nth);
    }

    if nth <= 0 {
        return -1;
    }

    if nth == 1 {
        return num % 10;
    }

    find_digit(num / 10, nth - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(find_digit(5673, 4), 5);
        assert_eq!(find_digit(129, 2), 2);
        assert_eq!(find_digit(-2825, 3), 8);
        assert_eq!(find_digit(-456, 4), 0);
        assert_eq!(find_digit(0, 20), 0);
        assert_eq!(find_digit(65, 0), -1);
        assert_eq!(find_digit(24, -8), -1);
    }
}
