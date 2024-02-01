//! https://www.codewars.com/kata/588e2a1ad1140d31cb00008c/train/rust

pub fn generate_pairs(m: i16, n: i16) -> Vec<(i16, i16)> {
    (m..=n).flat_map(|i| (i..=n).map(move |j| (i, j))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_case() {
        assert_eq!(generate_pairs(2, 4), vec![(2, 2), (2, 3), (2, 4), (3, 3), (3, 4), (4, 4)]);
        assert_eq!(generate_pairs(0, 1), vec![(0, 0), (0, 1), (1, 1) ]);
        assert_eq!(generate_pairs(0, 0), vec![(0, 0)]);
    }
}
