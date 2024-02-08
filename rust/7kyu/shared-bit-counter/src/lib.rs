//! https://www.codewars.com/kata/58a5aeb893b79949eb0000f1/train/rust

pub fn shared_bits(a: u32, b: u32) -> bool {
    (a & b).count_ones() >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert!(!shared_bits(1, 2), "Expected shared_bits(1, 2) to be false");
        assert!(!shared_bits(16, 8), "Expected shared_bits(16, 8) to be false");
        assert!(!shared_bits(1, 1), "Expected shared_bits(1, 1) to be false");
        assert!(!shared_bits(2, 3), "Expected shared_bits(2, 3) to be false");
        assert!(!shared_bits(7, 10), "Expected shared_bits(7, 10) to be false");
        assert!( shared_bits(43, 77), "Expected shared_bits(43, 77) to be true");
        assert!( shared_bits(7, 15), "Expected shared_bits(7, 15) to be true");
        assert!( shared_bits(23, 7), "Expected shared_bits(23, 7) to be true");
    }
}
