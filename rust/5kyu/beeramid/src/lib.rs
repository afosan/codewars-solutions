//! https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/rust

pub fn beeramid (bonus: i32, price: f32) -> usize {
    (1..).skip_while(|n| ((n * (n + 1) * (2 * n + 1) / 6) as f32) <= bonus as f32 / price).nth(0).unwrap_or(1) - 1
}

#[cfg(test)]
mod tests {
    use super::beeramid;

    #[test]
    fn sample_tests() {
        assert_eq!(beeramid(9, 2.0), 1);
        assert_eq!(beeramid(10, 2.0), 2);
        assert_eq!(beeramid(11, 2.0), 2);
        assert_eq!(beeramid(21, 1.5), 3);
        assert_eq!(beeramid(454, 5.0), 5);
        assert_eq!(beeramid(455, 5.0), 6);
        assert_eq!(beeramid(4, 4.0), 1);
        assert_eq!(beeramid(3, 4.0), 0);
        assert_eq!(beeramid(0, 4.0), 0);
        assert_eq!(beeramid(-1, 4.0), 0);
    }
}
