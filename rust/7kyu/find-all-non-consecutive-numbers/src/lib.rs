//! https://www.codewars.com/kata/58f8b35fda19c0c79400020f/train/rust

pub fn all_non_consecutive(arr: &[i32]) -> Vec<(usize, i32)> {
    arr.windows(2).enumerate().filter(|(_, w2)| w2[1] != w2[0] + 1).map(|(i, w2)| (i + 1, w2[1])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expect: Vec<(usize, i32)> = vec![
            (4, 6), (7, 10)
        ];
        let result = all_non_consecutive(&[1, 2, 3, 4, 6, 7, 8, 10]);
        
        assert_eq!(expect, result);
    }
}
