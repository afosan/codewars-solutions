//! https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a/train/rust

pub fn stones_to_remove(stones: &str) -> usize {
    let mut c = 0_usize;

    for (i, j) in stones.chars().zip(stones.chars().skip(1)) {
        if i == j {
            c += 1;
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::stones_to_remove;

    #[test]
    fn sample_tests() {
        assert_eq!(stones_to_remove("RGBRGBRGGB"), 1);
        assert_eq!(stones_to_remove("RGGRGBBRGRR"), 3);
        assert_eq!(stones_to_remove("RRRRGGGGBBBB"), 9);
    }
}
