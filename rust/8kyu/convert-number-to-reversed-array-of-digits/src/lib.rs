//! https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust

pub fn digitize(n: u64) -> Vec<u8> {
    n.to_string().chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348597), vec![7,9,5,8,4,3]);
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
