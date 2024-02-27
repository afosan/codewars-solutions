//! https://www.codewars.com/kata/62c93765cef6f10030dfa92b/train/rust

pub fn cats_and_shelves(start: u8, finish: u8) -> u8 {
    let jumps = finish - start;
    jumps / 3 + jumps % 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mew() {
        assert_eq!(cats_and_shelves(1, 5), 2);
    }
    #[test]
    fn freezy_cat() {
        assert_eq!(cats_and_shelves(1, 1), 0);
    }
    #[test]
    fn one_more_mew() {
        assert_eq!(cats_and_shelves(2, 5), 1);
    }
    #[test]
    fn woolen_bag() {
        assert_eq!(cats_and_shelves(2, 4), 2);
    }
}
