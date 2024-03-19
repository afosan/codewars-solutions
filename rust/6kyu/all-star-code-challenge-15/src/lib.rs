//! https://www.codewars.com/kata/586560a639c5ab3a260000f3/train/rust

pub fn rotate(s: &str) -> Vec<String> {
    let mut acc = s.to_string();
    (0..s.len()).map(|_| {
        acc = format!("{}{}", &acc[1..], &acc[..1]);
        acc.clone()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_empty() {
        assert_eq!(rotate(""), Vec::<String>::new(), "Should return empty string for input: {:?}", "");
    }

    #[test]
    fn test_rotate_single() {
        assert_eq!(rotate("a"), vec!["a"], "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "a");
    }

    #[test]
    fn test_rotate_triple() {
        assert_eq!(rotate("abc"),  vec!["bca", "cab", "abc"], 
            "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "abc");
    }

    #[test]
    fn test_rotate_long() {
        assert_eq!(rotate("hello"), vec!["elloh", "llohe", "lohel", "ohell", "hello"],
            "\n\nYour result (left) did not match the expected output (right) for the input: {:?}", "hello");
    }
}
