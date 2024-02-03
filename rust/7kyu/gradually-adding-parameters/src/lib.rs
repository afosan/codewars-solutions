//! https://www.codewars.com/kata/555b73a81a6285b6ce000047/train/rust

pub fn add(args: &[i64]) -> i64 {
    args.iter().enumerate().map(|(i, a)| (i as i64 + 1) * a).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add(&[]), 0);
        assert_eq!(add(&[4,-3,-2]), -8);
    }
}
