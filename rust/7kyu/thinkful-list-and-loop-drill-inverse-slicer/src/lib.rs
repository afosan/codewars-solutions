//! https://www.codewars.com/kata/586ec0b8d098206cce001141/train/rust

pub fn inverse_slice<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    input.iter().enumerate().filter(|(i, _)| !(a <= *i && *i < b)).map(|(_, v)| v).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 2, 4), [12, 14, 55, 24]);
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 0, 3), [72, 55, 24]);
        assert_eq!(
            inverse_slice(&["Intuition", "is", "a", "poor", "guide", "when", "facing", "probabilistic", "evidence"], 5, 13),
            ["Intuition", "is", "a", "poor", "guide"]);
        assert_eq!(inverse_slice::<i32>(&[], 1, 4), []);
        assert_eq!(inverse_slice(&[0, 0, 0, 1, 0], 0, 3), [1, 0]);
    }
}
