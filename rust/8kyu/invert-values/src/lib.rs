//! https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust

pub fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|n| -n).collect()
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1,2,3,4,5]), vec![-1,-2,-3,-4,-5]);
        assert_eq!(invert(&vec![1,-2,3,-4,5]), vec![-1,2,-3,4,-5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
}
