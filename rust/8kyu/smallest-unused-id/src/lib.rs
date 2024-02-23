//! https://www.codewars.com/kata/55eea63119278d571d00006a/train/rust

use std::collections::HashSet;

pub fn next_id(ids: &[usize]) -> usize {
    let hs = HashSet::<usize>::from_iter(ids.iter().copied());

    (0..).filter(|i| !hs.contains(i)).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(next_id(&[0,1,2,4,5]), 3);
        assert_eq!(next_id(&[0,1,2,3,4,5,6,7,8,9,10]), 11);
    }
}
