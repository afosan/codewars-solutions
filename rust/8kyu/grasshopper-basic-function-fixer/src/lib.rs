//! https://www.codewars.com/kata/56200d610758762fb0000002/train/rust

pub fn add_five(num: i32) -> i32 {
    num + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(add_five(5), 10);
        assert_eq!(add_five(0), 5);
        assert_eq!(add_five(-5), 0);
    }
}
