//! https://www.codewars.com/kata/5875b200d520904a04000003/train/rust

pub fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    i32::max(0, on + wait - cap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enough() {
        assert_eq!(enough(10, 5, 5), 0, "enough(10, 5, 5) should return 0");
        assert_eq!(enough(100, 60, 50), 10, "enough(100, 60, 50) should return 10");
        assert_eq!(enough(20, 5, 5), 0, "enough(20, 5, 5) should return 0");
    }
}
