//! https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust

pub fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(
            |s| s
                    .to_lowercase()
                    .chars()
                    .enumerate()
                    .map(
                        |(i, c)| if i == (c as usize - 'a' as usize) { 1 } else { 0 }
                    )
                    .sum::<usize>()
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            solve(&["abode".to_string(), "ABc".to_string(), "xyzD".to_string()]),
            vec![4, 3, 1]
        );
        
        assert_eq!(
            solve(&["abide".to_string(), "ABc".to_string(), "xyz".to_string()]),
            vec![4, 3, 0]
        );
        
        assert_eq!(
            solve(&[
                "IAMDEFANDJKL".to_string(),
                "thedefgh".to_string(),
                "xyzDEFghijabc".to_string()
            ]),
            vec![6, 5, 7]
        );
        
        assert_eq!(
            solve(&[
                "encode".to_string(),
                "abc".to_string(),
                "xyzD".to_string(),
                "ABmD".to_string()
            ]),
            vec![1, 3, 1, 3]
        );
    }
}
