//! https://www.codewars.com/kata/526571aae218b8ee490006f4/train/rust

pub fn count_bits(n: i64) -> u32 {
    if n == 0 {
        return 0;
    }

    match n % 2 == 0 {
        true => count_bits(n / 2),
        false => 1 + count_bits(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
    }
}
