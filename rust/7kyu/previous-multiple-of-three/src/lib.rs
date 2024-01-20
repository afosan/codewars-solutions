//! https://www.codewars.com/kata/61123a6f2446320021db987d/train/rust

pub fn prev_mult_of_three(n: i32) -> i32 {
    let mut num = n;

    while num % 3 != 0 {
        num /= 10;
    }

    if num == 0 { -1 } else { num }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(prev_mult_of_three(1), -1);
        assert_eq!(prev_mult_of_three(25), -1);
        assert_eq!(prev_mult_of_three(36), 36);
        assert_eq!(prev_mult_of_three(1244), 12);
        assert_eq!(prev_mult_of_three(952406), 9);
    }
}
