//! https://www.codewars.com/kata/58712dfa5c538b6fc7000569/train/rust

pub fn count_red_beads(n: u32) -> u32 {
    if n < 2 {
        0
    } else {
        2 * (n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_red_beads(0), 0);
        assert_eq!(count_red_beads(1), 0);
        assert_eq!(count_red_beads(3), 4);
        assert_eq!(count_red_beads(5), 8);
    }
}
