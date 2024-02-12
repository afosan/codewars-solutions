//! https://www.codewars.com/kata/52ceafd1f235ce81aa00073a/train/rust

pub fn plural (n: f64) -> bool {
    n != 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(plural(0.0), true);
        assert_eq!(plural(0.5), true);
        assert_eq!(plural(1.0), false);
        assert_eq!(plural(100.0), true);
    }
}
