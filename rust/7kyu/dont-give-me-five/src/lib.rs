//! https://www.codewars.com/kata/5813d19765d81c592200001a/train/rust

pub fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end).filter(|n| n % 10 != 5).count() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(dont_give_me_five(1, 9), 8);
        assert_eq!(dont_give_me_five(4, 17), 12);
    }
}
