//! https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust

pub fn pyramid(balls: u16) -> u16 {    
    (0..).take_while(|i| i * (i + 1) / 2 <= balls).last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(1), 1);
        assert_eq!(pyramid(4), 2);
        assert_eq!(pyramid(20), 5);
        assert_eq!(pyramid(100), 13);
        assert_eq!(pyramid(2211), 66);
        assert_eq!(pyramid(9999), 140);
    }
}
