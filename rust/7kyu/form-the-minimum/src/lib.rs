//! https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust

pub fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    let mut out = 0;
    let mut last_digit = 0;

    for digit in digits {
        if digit != last_digit {
            last_digit = digit;
            out = 10 * out + digit;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(min_value(vec![1, 3, 1]), 13);
        assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
        assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
    }
}
