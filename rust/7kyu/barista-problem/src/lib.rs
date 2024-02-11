//! https://www.codewars.com/kata/6167e70fc9bd9b00565ffa4e/train/rust

use itertools::Itertools;

pub fn barista(coffees: &[u8]) -> u16 {
    let l = coffees.len() as u16;
    
    if l > 0 {
        coffees.iter().sorted().enumerate().map(|(i, c)| (l - i as u16) * *c as u16).sum::<u16>() + (l - 1) * l
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::barista;

    #[test]
    fn sample_tests() {
        assert_eq!(barista(&[]), 0);
        assert_eq!(barista(&[2, 10, 5, 3, 9]), 85);
        assert_eq!(barista(&[4, 3, 2]), 22);
        assert_eq!(barista(&[20, 5]), 32);
        assert_eq!(barista(&[20, 5, 4, 3, 1, 5, 7, 8]), 211);
        assert_eq!(barista(&[5, 4, 3, 2, 1]), 55);
    }
}
