//! https://www.codewars.com/kata/5a262cfb8f27f217f700000b/train/rust

use std::collections::HashSet;

pub fn solve(a: &str, b: &str) -> String {
    let chars_a = HashSet::<char>::from_iter(a.chars());
    let chars_b = HashSet::<char>::from_iter(b.chars());
    
    a.chars().filter(|c| !chars_b.contains(&c))
        .chain(
            b.chars().filter(|c| !chars_a.contains(&c))
        )
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(solve("xyab","xzca"),"ybzc".to_string());
        assert_eq!(solve("xyabb","xzca"),"ybbzc".to_string());
        assert_eq!(solve("abcd","xyz"),"abcdxyz".to_string());
        assert_eq!(solve("xxx","xzca"),"zca".to_string());
    }
}
