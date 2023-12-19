//! https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust

pub fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(repeat_str("a", 4), "aaaa");
        assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
        assert_eq!(repeat_str("abc", 2), "abcabc");
    }
}
