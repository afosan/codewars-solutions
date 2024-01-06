//! https://www.codewars.com/kata/554ca54ffa7d91b236000023/train/rust

use std::collections::HashMap;

pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = HashMap::<u8, usize>::new();

    lst
        .iter()
        .filter(|i| {
            let c = *counts.get(&i).unwrap_or(&0);
            counts.insert(**i, c + 1);
            c < n
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}
