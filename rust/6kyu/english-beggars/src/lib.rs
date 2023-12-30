//! https://www.codewars.com/kata/59590976838112bfea0000fa/train/rust

pub fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    values.iter().enumerate().fold(vec![0; n], |mut acc, (i, v)| {
        acc[i % n] += v;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 1), [15]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 2), [9, 6]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 3), [5, 7, 3]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 6), [1, 2, 3, 4, 5, 0]);
    }
    
    #[test]
    fn test_zero_beggars() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 0), []);
    }
}
