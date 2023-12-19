//! https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust

use std::collections::HashMap;

pub fn find_odd(arr: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    arr.iter().for_each(|n| *counts.entry(n).or_insert(0) += 1);

    for (d, c) in counts {
        if c % 2 == 1 {
            return *d;
        }
    }

    panic!("expected at least one element to appear odd number of times");
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(find_odd(&vec![20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]), 5);
        assert_eq!(find_odd(&vec![1,1,2,-2,5,2,4,4,-1,-2,5]), -1);
        assert_eq!(find_odd(&vec![20,1,1,2,2,3,3,5,5,4,20,4,5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1,1,1,1,1,1,10,1,1,1,1]), 10);
        assert_eq!(find_odd(&vec![5,4,3,2,1,5,4,3,2,10,10]), 1);
    }
}
