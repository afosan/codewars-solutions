//! https://www.codewars.com/kata/511f11d355fe575d2c000001/train/rust

pub fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let l = ages.len();
    let mut v = ages.to_vec();
    v.sort_unstable();
    
    [v[l - 2], v[l - 1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_oldest_ages() {
        assert_eq!(two_oldest_ages(&[1, 5, 87, 45, 8, 8]), [45, 87]);
        assert_eq!(two_oldest_ages(&[6, 5, 83, 5, 3, 18]), [18, 83]);
        assert_eq!(two_oldest_ages(&[10, 1]), [1, 10]);
        assert_eq!(two_oldest_ages(&[1, 3, 10, 0]), [3, 10]);
    }
}
