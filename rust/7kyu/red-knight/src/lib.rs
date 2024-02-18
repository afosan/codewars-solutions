//! https://www.codewars.com/kata/5fc4349ddb878a0017838d0f/train/rust

pub fn red_knight(n: u64, p: u64) -> (&'static str, u64) {
    (if (n + p) % 2 == 0 { "White" } else { "Black" }, 2 * p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(red_knight(0, 8), ("White", 16));
        assert_eq!(red_knight(0, 7), ("Black", 14));
        assert_eq!(red_knight(1, 6), ("Black", 12));
        assert_eq!(red_knight(1, 5), ("White", 10));
    }
}
