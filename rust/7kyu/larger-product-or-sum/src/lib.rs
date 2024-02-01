//! https://www.codewars.com/kata/5c4cb8fc3cf185147a5bdd02/train/rust

use std::cmp::Ordering;

pub fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut nums = list.to_vec();
    nums.sort_unstable();

    let sm: i64 = nums.iter().rev().take(n).sum();
    let pr: i64 = nums.iter().take(n).product();

    match sm.cmp(&pr) {
        Ordering::Less => "product",
        Ordering::Equal => "same",
        Ordering::Greater => "sum",
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sum_or_product(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), "sum");
        assert_eq!(sum_or_product(&[10, 41, 8, 16, 20, 36, 9, 13, 20], 3), "product");
        assert_eq!(sum_or_product(&[10, 20, 3, 30, 5, 4], 3), "same");
    }
}
