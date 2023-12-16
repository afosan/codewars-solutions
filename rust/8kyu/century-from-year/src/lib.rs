//! https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/train/rust

pub fn century(year: u32) -> u32 {
    year / 100 + 1 - (year % 100 == 0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(century(1705), 18);
        assert_eq!(century(1900), 19);
        assert_eq!(century(1601), 17);
        assert_eq!(century(2000), 20);
        assert_eq!(century(89), 1);
    }
}
