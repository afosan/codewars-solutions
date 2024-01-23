//! https://www.codewars.com/kata/5dd259444228280032b1ed2a/train/rust

pub fn solve(sum: u32, gcd: u32) -> Option<(u32, u32)> {
    if sum % gcd == 0 {
        Some((gcd, sum - gcd))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_tests() {
        assert_eq!(solve(8, 2), Some((2, 6)));
        assert_eq!(solve(12, 4), Some((4, 8)));
        assert_eq!(solve(12, 5), None);
        assert_eq!(solve(50, 10), Some((10, 40)));
        assert_eq!(solve(50, 4), None);
        assert_eq!(solve(10, 5), Some((5, 5)));
        assert_eq!(solve(100, 5), Some((5, 95)));
        assert_eq!(solve(1000, 5), Some((5, 995)));
    }
}
