//! https://www.codewars.com/kata/59f061773e532d0c87000d16/train/rust

pub fn elevator_distance(floors: &[i16]) -> i16 {
    floors.windows(2).map(|s| (s[0] - s[1]).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(elevator_distance(&[5, 2, 8]), 9);
        assert_eq!(elevator_distance(&[1, 2, 3]), 2);
        assert_eq!(elevator_distance(&[7, 1, 7, 1]), 18);
    }
}
