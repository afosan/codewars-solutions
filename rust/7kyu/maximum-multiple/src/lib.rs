//! https://www.codewars.com/kata/5aba780a6a176b029800041c/train/rust

pub fn max_multiple(divisor: u32, bound: u32) -> u32 {
    bound / divisor * divisor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(max_multiple(2,7),6);
        assert_eq!(max_multiple(3,10),9);
        assert_eq!(max_multiple(7,17),14);
        assert_eq!(max_multiple(10,50),50);
    }
}
