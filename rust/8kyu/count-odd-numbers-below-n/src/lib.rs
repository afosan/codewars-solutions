//! https://www.codewars.com/kata/59342039eb450e39970000a6/train/rust

pub fn odd_count(n: u64) -> u64 {
    n / 2
}

#[cfg(test)]
mod tests {
    use super::odd_count;

    #[test]
    fn sample_tests() {
        assert_eq!(odd_count(15), 7);
        assert_eq!(odd_count(15023), 7511);
    }
}
