//! https://www.codewars.com/kata/5aec1ed7de4c7f3517000079/train/rust

pub fn first_n_smallest (arr: &[i32], n: usize) -> Vec<i32> {
    if n == 0 { return vec![] };

    let mut v = arr.to_vec();
    v.sort_unstable();

    let t = v[n - 1];

    arr.iter().filter(|a| **a <= t).take(n).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_n_smallest(&[1,2,3,4,5],3), [1,2,3]);
        assert_eq!(first_n_smallest(&[5,4,3,2,1],3), [3,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,1,2],3), [1,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,-4,0],3), [1,-4,0]);
        assert_eq!(first_n_smallest(&[1,2,3,4,5],0), []);
        assert_eq!(first_n_smallest(&[1,2,3,4,5],5), [1,2,3,4,5]);
        assert_eq!(first_n_smallest(&[1,2,3,4,2],4), [1,2,3,2]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],2), [2,1]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],3), [2,1,2]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],4), [2,1,2,2]);
    }
}
