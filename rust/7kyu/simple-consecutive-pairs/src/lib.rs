//! https://www.codewars.com/kata/5a3e1319b6486ac96f000049/train/rust

pub fn pairs(arr: &Vec<i32>) -> usize {
    arr.chunks_exact(2).filter(|s| (s[0] - s[1]).abs() == 1).count()
}

#[cfg(test)]
mod tests {
    use super::pairs;

    #[test]
    fn example_tests() {
        assert_eq!(pairs(&vec![1, 2, 5, 8, -4, -3, 7, 6, 5]), 3);
        assert_eq!(pairs(&vec![21, 20, 22, 40, 39, -56, 30, -55, 95, 94]), 2);
        assert_eq!(pairs(&vec![81, 44, 80, 26, 12, 27, -34, 37, -35]), 0);
        assert_eq!(pairs(&vec![-55, -56, -7, -6, 56, 55, 63, 62]), 4);
        assert_eq!(pairs(&vec![73, 72, 8, 9, 73, 72]), 3);
    }
}
