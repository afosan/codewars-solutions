//! https://www.codewars.com/kata/55a5c82cd8e9baa49000004c/train/rust

pub fn divisible_count(x: u64, y: u64, k: u64) -> u64 {
    y / k - x / k + if x % k == 0 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::divisible_count;

    #[test]
    fn fixed_tests() {
        assert_eq!(divisible_count(6, 11, 2), 3);
        assert_eq!(divisible_count(11, 345, 17), 20);
        assert_eq!(divisible_count(0, 1, 7), 1);
        assert_eq!(divisible_count(20, 20, 2), 1);
        assert_eq!(divisible_count(20, 20, 8), 0);
        assert_eq!(divisible_count(19, 20, 2), 1);
        assert_eq!(divisible_count(0, 10, 1), 11);
        assert_eq!(divisible_count(11, 14, 2), 2);
        assert_eq!(divisible_count(64, 73, 27), 0);
        assert_eq!(divisible_count(101, i32::MAX as u64, 11), 195225777);
        assert_eq!(divisible_count(1005, i32::MAX as u64, 109), 19701675);
        assert_eq!(divisible_count(101, u32::MAX as u64, 11), 390451563);
        assert_eq!(divisible_count(1005, u32::MAX as u64, 109), 39403360);
        assert_eq!(divisible_count(101, i64::MAX as u64, 11), 838488366986797791);
        assert_eq!(divisible_count(1005, i64::MAX as u64, 109), 84618092081236466);
        assert_eq!(divisible_count(101, u64::MAX, 11), 1676976733973595592);
        assert_eq!(divisible_count(1005, u64::MAX, 109), 169236184162472941);
    }
}
